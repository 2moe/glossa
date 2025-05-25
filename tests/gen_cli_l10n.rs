use std::{io, process::Command};

// use anyhow::Result as AnyResult;
// use glossa::MiniStr;
use tap::Tap;

fn run_glossa_cli() -> Command {
  Command::new("glossa-cli").tap_mut(|x| {
    x.args([
      "--include-map-names",
      "cli",
      "--visibility",
      "pub",
      "--outdir",
      "crates/cli/tmp",
    ]);
  })
}

#[ignore]
#[test]
fn test_install_glossa_cli() -> io::Result<()> {
  Command::new("cargo")
    .args(["install", "--path", "crates/cli"])
    .status()?;

  Ok(())
}

#[ignore]
#[test]
fn test_get_help() -> io::Result<()> {
  run_glossa_cli()
    .arg("--help")
    .status()?;

  Ok(())
}

#[ignore]
#[test]
fn test_gen_features() -> io::Result<()> {
  run_glossa_cli()
    .arg("--output-cargo-features")
    .status()?;

  Ok(())
}

#[ignore]
#[test]
fn test_gen_mod_rs() -> io::Result<()> {
  run_glossa_cli()
    .arg("--output-mod-rs")
    .status()?;

  Ok(())
}

#[ignore]
#[test]
fn test_gen_router() -> io::Result<()> {
  run_glossa_cli()
    .arg("--output-router-for-match-fns-without-map-name")
    .status()?;

  Ok(())
}

#[ignore]
#[test]
fn test_output_phf_map() -> io::Result<()> {
  run_glossa_cli()
    .arg("--output-phf")
    .status()?;

  Ok(())
}
