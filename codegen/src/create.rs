use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

/// If the file path does not exist, then it will create the file and import the relevant module.
///
/// If it already exists, then it determines if the module in question exists in `mod.rs`. If it does, it does nothing; otherwise, it appends the relevant statements.
///
/// Note: the detection step is not strict, it does not exclude comments.
///
/// If the file path is **src/assets/localisation.rs**, then it will detect **src/assets/mod.rs**, append `mod localisation;` and `use glossa::LangID;` etc to **mod.rs**.
pub fn append_to_l10n_mod<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    let previous = path
        .parent()
        .expect("Invalid path");

    // Create all directories under the parent directory if they do not exist
    fs::create_dir_all(previous)?;

    let mod_stmt = get_mod_stem_name(path);

    let mod_rs = previous.join("mod.rs");

    let mut mod_file = File::options()
        .create(true)
        .append(true)
        .open(&mod_rs)?;

    let mut write_to_mod_rs =
        |contents: &str| -> io::Result<()> { writeln!(mod_file, "{}", contents) };

    let mod_rs_content = || fs::read_to_string(&mod_rs);
    let not_contains =
        |s| -> io::Result<bool> { Ok(!mod_rs_content()?.contains(s)) };

    // `mod localisation;` -> `pub(crate) mod localisation;`
    let pub_crate_mod_stmt = format!("pub(crate) {mod_stmt}");

    // lang_id_consts
    match mod_rs.exists() {
        false => {
            use to_be_imported::*;
            for s in [
                USE_MAP,
                USE_ID,
                USE_MAP,
                USE_CONSTS,
                "\n",
                &pub_crate_mod_stmt,
            ] {
                write_to_mod_rs(s)?;
            }
        }

        true => {
            // const CELL: &str = "OnceCell";
            const MAP: &str = "HashMap";
            const ID: &str = "LangID";
            const ID_CONSTS: &str = "lang_id_consts";

            for s in [
                &mod_stmt, //
                // CELL,
                MAP, ID, ID_CONSTS,
            ] {
                if not_contains(s)? {
                    use to_be_imported::*;
                    write_to_mod_rs(match s {
                        // CELL => USE_CELL,
                        MAP => USE_MAP,
                        ID => USE_ID,
                        ID_CONSTS => USE_CONSTS,
                        s if s == mod_stmt => &pub_crate_mod_stmt,
                        _ => "",
                    })?;
                }
            }
        }
    }
    mod_file.flush()
}

/// Original dependencies(modules):
///
/// ```no_run
/// const USE_MAP: &str = "use ahash::HashMap;";
/// const USE_ID: &str = "use lang_id::LangID;";
/// const USE_CONSTS: &str = "use lang_id::consts as lang_id_consts;";
/// ```
mod to_be_imported {
    // Modules for glossa re-export:
    // const USE_CELL: &str = "use once_cell::sync::OnceCell;";
    // pub(super) const USE_CELL: &str = "use glossa::assets::OnceCell;";
    pub(super) const USE_MAP: &str = "use glossa::assets::HashMap;";
    pub(super) const USE_ID: &str = "use glossa::LangID;";
    pub(super) const USE_CONSTS: &str = "use glossa::assets::lang_id_consts;";
}

/// Gets the module name by file path
///
/// name: `mod mod_name;`
/// If the file path is **assets/localisation.rs**, then the name is `mod localisation;`
fn get_mod_stem_name(path: &Path) -> String {
    format!(
        "mod {};",
        path.file_stem()
            .expect("Invalid rs file")
            .to_str()
            .unwrap_or_else(|| panic!("Invalid file name"))
    )
}
