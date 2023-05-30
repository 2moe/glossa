use crate::{
    deser::{
        data_map::CfgFiles,
        locale::{get_locale_list, iter_over_locale_list},
    },
    generator::Generator,
    prelude::MapWriter,
};
use std::{
    collections::{BTreeMap, HashSet},
    fs, io,
    path::PathBuf,
};

impl<'p, 'res, 'ver> Generator<'ver, 'p, 'res> {
    pub fn run(&self, mut writer: MapWriter) -> io::Result<()> {
        self.defind_rs_file_header(writer.get_tmp_file_mut())?;

        let sub_locale_map = phf_codegen::Map::new();
        // Create a new, empty map for localisation data
        let data_map = phf_codegen::Map::new();

        let org_dir = self.get_l10n_path();

        let mut locale_treemap = BTreeMap::new();
        let fpath = PathBuf::with_capacity(org_dir.capacity() + 2);

        // Sub-map name for detecting the same name
        let sub_sets = HashSet::with_capacity(20);

        let cfg_file = fpath.clone();
        let cfg_text = String::with_capacity(120);

        let files = CfgFiles::new(
            fpath,
            cfg_file,
            cfg_text,
            data_map,
            sub_locale_map,
            sub_sets,
            #[cfg(feature = "highlight")]
            self.get_highlight().as_ref(),
            #[cfg(not(feature = "highlight"))]
            None,
        );

        // Create a new, empty map for locale data
        let mut locale_map = phf_codegen::Map::new();

        let locale_list = get_locale_list(org_dir)?;

        iter_over_locale_list(
            locale_list.capacity(),
            &locale_list,
            files,
            org_dir,
            &mut locale_map,
            &mut locale_treemap,
            &mut writer,
        )?;

        writer.build_locale_phf_map(locale_map)?;
        writer.build_lc_treemap(locale_treemap)?;

        fs::rename(writer.get_tmp_path(), writer.get_rs_path())?;
        Ok(())
    }
}
