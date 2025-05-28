use glossa_cli::options::Cli;
use glossa_codegen::{AnyResult, glossa_shared::tap::Pipe};
use log::trace;

fn main() -> AnyResult<()> {
  init_logger();
  trace!("logger initialized");

  Cli::run()?;
  Ok(())
}

fn init_logger() {
  env_logger::Env::new()
    .filter_or("GLOSSA_LOG", "info")
    .pipe(env_logger::Builder::from_env)
    .init()
}
