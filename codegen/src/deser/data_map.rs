#[cfg(feature = "highlight")]
use crate::highlight::HighLight;

#[cfg(feature = "highlight")]
use hlight::gen_syntax_highlight;

#[cfg(feature = "highlight")]
use once_cell::sync::OnceCell;

#[cfg(not(feature = "highlight"))]
use std::marker::PhantomData;

use crate::{
    conversion::convert_to_fn_name,
    deser::locale::locale_map_collision_checker,
    map_writer::{DataMap, TupStrMap},
    prelude::MapWriter,
};
use getset::{Getters, MutGetters};
use serde::Deserialize;

use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

const DE_ERR_MSG: &str = "Failed to deserialise the configuration file.";

// BTreeMap is used here, not HashMap
type L10nMap = BTreeMap<String, String>;

#[derive(Debug, Deserialize, Clone, Default)]
struct L10nData(L10nMap);

impl std::ops::Deref for L10nData {
    type Target = L10nMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Getters, MutGetters)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub(crate) struct CfgFiles<'s, 'p> {
    fpath: PathBuf,
    cfg_file: PathBuf,
    cfg_text: String,
    data_map: DataMap,
    lc_map_fn: Cow<'s, str>,
    sub_locale_map: DataMap,
    sub_sets: HashSet<String>,
    locale: &'s str,
    #[cfg(not(feature = "highlight"))]
    highlight: Option<&'s PhantomData<&'p PathBuf>>,
    #[cfg(feature = "highlight")]
    highlight: Option<&'s HighLight<'s, 'p>>,
}

#[allow(clippy::too_many_arguments)]
impl<'s, 'p> CfgFiles<'s, 'p> {
    pub(crate) fn new(
        fpath: PathBuf,
        cfg_file: PathBuf,
        cfg_text: String,
        data_map: DataMap,
        // lc_map_fn: &'s str,
        sub_locale_map: DataMap,
        sub_sets: HashSet<String>,
        #[cfg(feature = "highlight")] highlight: Option<&'s HighLight<'s, 'p>>,
        #[cfg(not(feature = "highlight"))] highlight: Option<
            &'s PhantomData<&'p PathBuf>,
        >,
        // locale: &'s str,
    ) -> Self {
        Self {
            fpath,
            cfg_file,
            cfg_text,
            data_map,
            lc_map_fn: Cow::from(""),
            sub_locale_map,
            sub_sets,
            locale: "",
            highlight,
        }
    }
}

/// For each cfg file in the directory, deserialise the data and add it to the map
pub(crate) fn iterate_over_all_cfg_files(
    files: &mut CfgFiles,
    writer: &mut MapWriter,
) -> Result<(), io::Error> {
    let CfgFiles {
        fpath,
        cfg_file,
        cfg_text,
        data_map,
        lc_map_fn,
        sub_locale_map,
        sub_sets,
        locale,
        highlight,
    } = files;

    #[allow(unused_assignments)]
    let mut cfg = L10nData::default();
    let mut data_doc_map = HashMap::new();
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
        // *data_map = phf_codegen::Map::new();

        let (mut org_key, mut sub_map_fn) = get_sub_map_fn_name(cfg_file, lc_map_fn);

        locale_map_collision_checker(sub_sets, &mut sub_map_fn);
        let fname = cfg_file.file_name();
        file_collision_checker(&mut file_sets, &mut org_key, fname);

        #[cfg(not(feature = "highlight"))]
        log::trace!("The highlight feature is not enabled. {:?}", highlight);
        #[allow(unused_mut)]
        let mut lite_doc = false;

        #[cfg(feature = "highlight")]
        match (*highlight, fname) {
            (Some(h), Some(f)) if h.get_files().contains_key(f) => {
                let hl_fmt = h.get_files().get(f).unwrap();
                let old_org = org_key.clone();
                let old_sub_map_fn = sub_map_fn.clone();

                if let Some(suf) = hl_fmt.get_suffix() {
                    write_to_data_map(
                        &cfg,
                        &mut data_doc_map,
                        locale,
                        &org_key,
                        data_map,
                        sub_locale_map,
                        &sub_map_fn,
                        writer,
                        lite_doc,
                    )?;
                    lite_doc = true;

                    org_key = format!("{}{suf}", org_key);
                    let suf = convert_to_fn_name(&suf);
                    sub_map_fn = format!("{sub_map_fn}{suf}");
                    locale_map_collision_checker(sub_sets, &mut sub_map_fn);
                    let fname = std::ffi::OsString::from(format!(
                        "{:?}{suf}",
                        cfg_file
                            .file_name()
                            .unwrap_or_default()
                    ));
                    file_collision_checker(
                        &mut file_sets,
                        &mut org_key,
                        Some(fname).as_deref(),
                    );
                }

                let syntax = hl_fmt.get_syntax();
                let mut buf: Vec<u8> = Vec::with_capacity(64);

                // Option<HashMap<&'f str, (&'f str, bool)>>
                // key: suffix, value: (theme_name, background)
                if let Some(map) = hl_fmt.get_extra() {
                    let mut tmp_style = h.get_resource().to_owned();
                    #[allow(unused_assignments)]
                    let mut tmp_org = org_key.clone();
                    #[allow(unused_assignments)]
                    let mut tmp_sub_map_fn = sub_map_fn.clone();

                    for (suf, (theme_name, bg)) in map {
                        // Create clone inside the loop, not outside
                        let mut tmp_cfg = cfg.clone();

                        if theme_name.is_empty() {
                            eprintln!(
                                "The extra theme is empty!\n name: {}",
                                theme_name
                            );
                            continue;
                        }

                        for v in tmp_cfg.0.values_mut() {
                            buf.clear();
                            *tmp_style.get_background_mut() = *bg;
                            // *tmp_style.get_background_mut() = *bg;
                            if &tmp_style.get_name().as_ref() != theme_name {
                                *tmp_style.get_name_mut() = Cow::from(*theme_name);
                                *tmp_style.get_theme_mut() = OnceCell::new();
                            }
                            gen_syntax_highlight(
                                syntax.as_ref(),
                                v,
                                Some(&tmp_style),
                                Some(&mut buf),
                            )?;

                            let s = String::from_utf8_lossy(buf.as_slice());
                            *v = s.into_owned();
                        }

                        tmp_org = format!("{}{suf}", old_org);
                        let suf = convert_to_fn_name(&suf);
                        tmp_sub_map_fn = format!("{}{suf}", old_sub_map_fn);
                        locale_map_collision_checker(sub_sets, &mut tmp_sub_map_fn);
                        let fname = std::ffi::OsString::from(format!(
                            "{:?}{suf}",
                            cfg_file
                                .file_name()
                                .unwrap_or_default()
                        ));
                        file_collision_checker(
                            &mut file_sets,
                            &mut tmp_org,
                            Some(fname).as_deref(),
                        );

                        write_to_data_map(
                            &tmp_cfg,
                            &mut data_doc_map,
                            locale,
                            &tmp_org,
                            data_map,
                            sub_locale_map,
                            &tmp_sub_map_fn,
                            writer,
                            true,
                        )?;
                    }
                }

                let style = h.get_resource();

                if style.get_name().is_empty() {
                    const MSG: &str = "The theme name is empty, skipping...";
                    log::warn!("{MSG}");
                    log::debug!("{:?}", style);
                    eprintln!("{MSG}");
                    continue;
                }

                for v in cfg.0.values_mut() {
                    buf.clear();

                    gen_syntax_highlight(
                        syntax.as_ref(),
                        v,
                        Some(style),
                        Some(&mut buf),
                    )?;

                    let s = String::from_utf8_lossy(buf.as_slice());
                    *v = s.into_owned();
                }
            }
            _ => {}
        }

        write_to_data_map(
            &cfg,
            &mut data_doc_map,
            locale,
            &org_key,
            data_map,
            sub_locale_map,
            &sub_map_fn,
            writer,
            lite_doc,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_to_data_map<'a>(
    cfg: &L10nData,
    data_doc_map: &mut TupStrMap<'a>,
    locale: &'a str,
    org: &str,
    data_map: &mut phf_codegen::Map<String>,
    sub_locale_map: &mut phf_codegen::Map<String>,
    sub_map_fn: &str,
    writer: &mut MapWriter,
    lite_doc: bool,
) -> Result<(), io::Error> {
    data_doc_map.clear();
    if let Some((k, v)) = cfg.iter().next() {
        data_doc_map
            .entry((locale, org.to_owned()))
            .or_insert((k.to_owned(), v.to_owned()));
    }
    for (k, v) in &cfg.0 {
        data_map.entry(k.to_owned(), &format!("r##\"{v}\"##"));
    }
    sub_locale_map.entry(org.to_owned(), sub_map_fn);
    if writer.gen_doc {
        writer.build_data_doc(&*data_doc_map, lite_doc)?;
    }
    writer.build_data_map(sub_map_fn, data_map)?;

    *data_map = phf_codegen::Map::new();

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

fn is_supported_format(o: &OsStr, formats: &[&str]) -> bool {
    formats
        .iter()
        .map(OsStr::new)
        .any(|a| o == a)
}

/// Suppose there are three files, opt.toml, opt.yaml, opt.yml. Since filenames are automatically converted to table names, the three tables would be opt, opt.yaml, opt.yml.
/// If the filename contains illegal unicode, then they would become "{org}_0"
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
    while sets.contains(org) {
        log::warn!("Duplicate file detected");
        *org = format!("{}_0", org)
    }
    sets.insert(org.to_owned());
}
