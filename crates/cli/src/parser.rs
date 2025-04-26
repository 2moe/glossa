use std::fs;

use clap::Parser;
use glossa::MiniStr;
use glossa_codegen::{AnyResult, glossa_shared::tap::Pipe, highlight::KString};
use log::trace;
mod collect;
mod init;
mod output;

use crate::{options::Cli, parser::output::output_data};

impl Cli {
  pub fn run() -> AnyResult<()> {
    let args = Cli::parse();
    trace!("args: {args:#?}");

    let generator = init::init_generator(&args)
      .with_resources(init::init_resources(&args))
      .pipe(|res| match init::init_highlight_cfg_map(&args) {
        Some(x) => res.with_highlight(x),
        _ => res,
      });
    trace!("generator: {generator:#?}");

    if let Some(dir) = generator.get_outdir() {
      log::info!("output dir: {dir:?}");
      fs::create_dir_all(dir)?;
    }

    output_data(&args, &generator)?;

    Ok(())
  }
}

pub fn puts<T: core::fmt::Display>(msg: &T) {
  println!("{msg}")
}

fn to_kstr(s: &MiniStr) -> KString {
  KString::from_ref(s)
}
