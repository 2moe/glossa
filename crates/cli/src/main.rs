use glossa_cli::parser::parse_args;
use glossa_codegen::AnyResult;

fn main() -> AnyResult<()> {
  parse_args()?;
  //
  Ok(())
}
