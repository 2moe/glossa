use clap::Parser;
use glossa_codegen::AnyResult;

use crate::options::Cli;

pub fn parse_args() -> AnyResult<()> {
  let args = Cli::parse();
  dbg!(&args);
  //
  Ok(())
}
