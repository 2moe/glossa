use clap::Parser;

use crate::{AnyResult, options::Cli};

pub fn parse_args() -> AnyResult<()> {
  let args = Cli::parse();
  dbg!(&args);
  //
  Ok(())
}
