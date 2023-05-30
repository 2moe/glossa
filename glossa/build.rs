const EARLY_RT: bool = true;
// const EARLY_RT: bool = false;

use glossa_codegen::{consts::*, prelude::*};
use std::{io, path::PathBuf};

fn main() -> io::Result<()> {
    let ver = get_pkg_version!();
    let rs_path = PathBuf::from_iter(default_l10n_rs_file_arr());

    if is_same_version(&rs_path, Some(ver))? && EARLY_RT {
        return Ok(());
    }

    append_to_l10n_mod(&rs_path)?;

    let tmp_path = get_shmem_path(&rs_path)?;
    let writer = MapWriter::new(&tmp_path, &rs_path);

    // The directory containing the localisation data
    let l10n_path = PathBuf::from_iter(parent_l10n_dir_arr());

    let generator = Generator::new(l10n_path).with_version(ver);
    generator.run(writer)
}
