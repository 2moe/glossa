use glossa_cli::options::Cli;
// use glossa_cli::parser::run;
use glossa_codegen::{AnyResult, glossa_shared::tap::Pipe};

fn main() -> AnyResult<()> {
  init_logger();
  Cli::run()?;
  Ok(())
}

fn init_logger() {
  env_logger::Env::new()
    .filter_or("GLOSSA_CLI_LOG", "info")
    .pipe(env_logger::Builder::from_env)
    .init()
}
