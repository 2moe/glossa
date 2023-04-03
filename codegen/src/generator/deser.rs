use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::generator::{
    conversion::convert_to_fn_name,
    header::defind_rs_file_header,
    map_writer::{DataMap, MapWriter},
};

#[derive(Debug, Deserialize, Clone, Default)]
// struct L10nData<'a>(#[serde(borrow)] HashMap<String, &'a str>);
struct L10nData(HashMap<String, String>);

impl std::ops::Deref for L10nData {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// visibility : Some("pub"), Some("pub(in path)"), etc.
/// If it's None, then use `Some("pub(crate)")`
pub fn deser_cfg_to_map<W: Write>(
    rs_file: &mut W,
    l10n_path: &mut PathBuf,
    visibility: Option<&str>,
    version: Option<&str>,
) -> io::Result<()> {
    writeln!(rs_file, "// Version: {}", version.unwrap_or("None"))?;

    defind_rs_file_header(rs_file)?;

    let pub_crate = visibility.unwrap_or("pub(crate)");

    let mut writer = MapWriter {
        rs_file,
        visibility: pub_crate,
    };

    // Create a new, empty map for locale data
    let mut locale_map = phf_codegen::Map::new();

    #[allow(unused_assignments)]
    let mut sub_locale_map = phf_codegen::Map::new();

    // Create a new, empty map for localisation data
    #[allow(unused_assignments)]
    let mut data_map = phf_codegen::Map::new();

    let org_dir = l10n_path.to_owned();

    let locale_list = get_locale_list(&org_dir, l10n_path)?;

    // To avoid repeated allocation of heap memory in the loop, several new mutable variables will be created
    // let mut locale_hashmap = HashMap::with_capacity(locale_list.capacity());
    let mut locale_treemap = BTreeMap::new();
    let mut fpath = PathBuf::with_capacity(org_dir.capacity() + 2);
    let mut sets = HashSet::with_capacity(locale_list.capacity());

    // Sub-map name for detecting the same name
    let mut sub_sets = HashSet::with_capacity(20);

    let mut cfg_file = fpath.clone();
    let mut cfg_text = String::with_capacity(120);

    // For each locale directory, generate a function that returns a map of localised data
    for locale in &locale_list {
        fpath = org_dir.join(locale);

        // name: get_{locale}_map
        let mut lc_map_fn = get_locale_map_fn_name(locale.trim());

        locale_map_collision_checker(&mut sets, &mut lc_map_fn);

        locale_map.entry(locale, &lc_map_fn);

        let locale = locale.trim();

        locale_treemap
            .entry(locale)
            .or_insert(lc_map_fn.to_owned());

        // Clear the sub locale map
        sub_locale_map = phf_codegen::Map::new();

        iterate_over_all_cfg_files(
            &mut fpath,
            &mut cfg_file,
            &mut cfg_text,
            &mut data_map,
            &lc_map_fn,
            &mut sub_locale_map,
            &mut writer,
            &mut sub_sets,
            locale,
        )?;

        writer.build_sub_locale_map(&lc_map_fn, locale, &mut sub_locale_map)?;
    }
    writer.build_locale_phf_map(locale_map)?;
    writer.build_lc_hashmap(locale_treemap)?;

    Ok(())
}

fn is_supported_format(o: &OsStr, formats: &[&str]) -> bool {
    formats
        .iter()
        .map(OsStr::new)
        .any(|a| o == a)
}

const DE_ERR_MSG: &str = "Failed to deserialise the configuration file.";
/// For each cfg file in the directory, deserialise the data and add it to the map
#[allow(clippy::too_many_arguments)]
fn iterate_over_all_cfg_files<W: Write>(
    fpath: &mut PathBuf,
    cfg_file: &mut PathBuf,
    cfg_text: &mut String,
    data_map: &mut DataMap,
    lc_map_fn: &str,
    sub_locale_map: &mut DataMap,
    writer: &mut MapWriter<W>,
    sub_sets: &mut HashSet<String>,
    locale: &str,
) -> Result<(), io::Error> {
    #[allow(unused_assignments)]
    let mut cfg = L10nData::default();
    let mut data_hashmap = HashMap::new();
    let mut file_sets = HashSet::with_capacity(20);

    let iter = fs::read_dir(&*fpath)?.filter_map(Result::ok);

    for file in iter.filter(|entry| {
        *fpath = entry.path();
        fpath.is_file()
            && fpath
                .extension()
                .map_or(false, |o| {
                    is_supported_format(o, &["yaml", "toml", "ron", "json", "yml"])
                })
    }) {
        *cfg_file = file.path();

        *cfg_text = fs::read_to_string(&*cfg_file)?;

        cfg = match cfg_file
            .extension()
            .map(|x| x.to_string_lossy())
            .expect(DE_ERR_MSG)
            .as_ref()
        {
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => serde_yaml::from_str(cfg_text).expect(DE_ERR_MSG),
            #[cfg(feature = "toml")]
            "toml" => toml::from_str(cfg_text).expect(DE_ERR_MSG),
            #[cfg(feature = "json")]
            "json" => serde_json::from_str(cfg_text).expect(DE_ERR_MSG),
            #[cfg(feature = "ron")]
            "ron" => ron::from_str(cfg_text).expect(DE_ERR_MSG),
            _ => continue,
        };

        // Clear the data map
        *data_map = phf_codegen::Map::new();
        data_hashmap.clear();

        let (mut org, mut sub_map_fn) = get_sub_map_fn_name(cfg_file, lc_map_fn);
        locale_map_collision_checker(sub_sets, &mut sub_map_fn);
        file_collision_checker(&mut file_sets, &mut org, cfg_file.file_name());

        if let Some((k, v)) = cfg.0.iter().next() {
            data_hashmap
                .entry((locale, org.to_owned()))
                .or_insert((k.to_owned(), v.to_owned()));
        }
        for (k, v) in cfg.0 {
            data_map.entry(k, &format!("r##\"{v}\"##"));
        }

        sub_locale_map.entry(org, &sub_map_fn);

        writer.build_data_hashmap(&data_hashmap)?;
        writer.build_data_map(&sub_map_fn, data_map)?;
    }
    Ok(())
}

fn get_sub_map_fn_name(file: &Path, lc_map_fn: &str) -> (String, String) {
    let org = file
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Invalid file name")
        .to_owned();

    let file_name = convert_to_fn_name(&org);
    let fn_name = format!("{}_{}", lc_map_fn, file_name);

    (org, fn_name)
}

fn locale_map_collision_checker(sets: &mut HashSet<String>, lc_map_fn: &mut String) {
    if sets.contains(lc_map_fn) {
        *lc_map_fn = format!("{}_0", lc_map_fn)
    }
    sets.insert(lc_map_fn.to_owned());
}

fn file_collision_checker(
    sets: &mut HashSet<String>,
    org: &mut String,
    fname: Option<&OsStr>,
) {
    if sets.contains(org) {
        match fname.and_then(|s| s.to_str()) {
            Some(x) => *org = x.to_string(),
            _ => *org = format!("{}_0", org),
        }
    }
    sets.insert(org.to_owned());
}

fn get_locale_map_fn_name(locale_name: &str) -> String {
    format!(
        "get_{}_map",
        convert_to_fn_name(locale_name)
            .trim_start_matches('_')
            .trim_end_matches('_')
    )
}

/// Retrieve a vector of all directories in the original directory
fn get_locale_list(
    org_dir: &Path,
    l10n_path: &mut PathBuf,
) -> io::Result<Vec<String>> {
    let mut v = fs::read_dir(org_dir)?
        .filter_map(Result::ok)
        .filter(|x| {
            *l10n_path = x.path();
            l10n_path.is_dir()
        })
        .filter_map(|s| s.file_name().into_string().ok())
        .collect::<Vec<_>>();
    v.sort_unstable();
    Ok(v)
}
