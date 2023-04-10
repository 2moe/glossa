use glossa_codegen::{consts::*, prelude::*};
use std::{
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let version = Some(get_pkg_version!());
    // let version = None;

    // Create a new `PathBuf` from the result of calling `get_l10n_rs_file_arr()`
    let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&path, version)? {
        return Ok(());
    }

    append_to_l10n_mod(&path)?;

    let mut file = BufWriter::new(File::create(&path)?);

    // Update the `PathBuf` to point to the directory containing the localisation data
    path = PathBuf::from_iter(parent_l10n_dir_arr());

    // Deserialise the config files in the given path
    deser_cfg_to_map(&mut file, &mut path, "pub(crate)", version, true)
}
