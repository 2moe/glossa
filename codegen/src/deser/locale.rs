use crate::{
    deser::{
        convert_to_fn_name,
        data_map::{iterate_over_all_cfg_files, CfgFiles},
    },
    prelude::MapWriter,
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    fs, io,
    path::Path,
};

pub(crate) fn locale_map_collision_checker(
    sets: &mut HashSet<String>,
    lc_map_fn: &mut String,
) {
    while sets.contains(lc_map_fn) {
        log::warn!("Duplicate map fn name detected");
        *lc_map_fn = format!("{}_0", lc_map_fn);
    }

    sets.insert(lc_map_fn.to_owned());
}

pub(crate) fn get_locale_map_fn_name(locale_name: &str) -> String {
    format!(
        "get_{}_map",
        convert_to_fn_name(locale_name)
            .trim_start_matches('_')
            .trim_end_matches('_')
    )
}

/// Retrieve a vector of all directories in the original directory
pub(crate) fn get_locale_list(org_dir: &Path) -> io::Result<Vec<String>> {
    let mut path = org_dir.to_path_buf();

    let mut v = fs::read_dir(org_dir)?
        .filter_map(Result::ok)
        .filter(|x| {
            path = x.path();
            path.is_dir()
        })
        .filter_map(|s| s.file_name().into_string().ok())
        .collect::<Vec<_>>();
    v.sort_unstable();
    Ok(v)
}

pub(crate) fn iter_over_locale_list<'a>(
    capacity: usize,
    locale_chunks: &'a [String],
    mut files: CfgFiles<'a, '_>,
    org_dir: &Path,
    locale_map: &mut phf_codegen::Map<&'a String>,
    locale_treemap: &mut BTreeMap<&'a str, String>,
    writer: &mut MapWriter,
) -> io::Result<()> {
    let mut sets = HashSet::with_capacity(capacity);
    for locale in locale_chunks {
        *files.get_fpath_mut() = org_dir.join(locale);

        // name: get_{locale}_map
        let mut lc_map_fn = get_locale_map_fn_name(locale.trim());

        locale_map_collision_checker(&mut sets, &mut lc_map_fn);

        locale_map.entry(locale, &lc_map_fn);

        let locale = locale.trim();
        *files.get_locale_mut() = locale;

        locale_treemap
            .entry(locale)
            .or_insert(lc_map_fn.to_owned());

        *files.get_lc_map_fn_mut() = Cow::from(lc_map_fn.to_owned());

        // Clear the sub locale map
        *files.get_sub_locale_map_mut() = phf_codegen::Map::new();

        iterate_over_all_cfg_files(&mut files, writer)?;

        writer.build_sub_locale_map(
            &lc_map_fn,
            locale,
            files.get_sub_locale_map_mut(),
        )?;
    }
    Ok(())
}
