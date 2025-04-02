use glossa_cli::{AnyResult, parser::parse_args};

fn main() -> AnyResult<()> {
  parse_args()?;

  Ok(())
}
