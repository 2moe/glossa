use std::{env::set_current_dir, path::Path, process::Command};

use anyhow::Result as AnyResult;
use tap::Pipe;

#[ignore]
#[test]
fn cargo_publish() -> AnyResult<()> {
  let workdir = env!("CARGO_MANIFEST_DIR").pipe(Path::new);

  [
    // "crates/codegen",
    "crates/glossa",
    "crates/shared",
    // "crates/l10n",
    // "crates/cli",
    // "crates/ui",
  ]
  .into_iter()
  .try_for_each(|d| {
    workdir
      .join(d)
      .pipe(set_current_dir)?;
    Command::new("cargo-pub").status()?;
    Ok(())
  })
}
