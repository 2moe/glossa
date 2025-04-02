use std::path::PathBuf;

use clap::{ColorChoice, Parser};
use getset::Getters;
use glossa_codegen::MiniStr;

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub with_prefix")]
#[command(arg_required_else_help = true)]
#[command(color = ColorChoice::Always)]
pub struct Cli {
  #[arg(
      short,
      long,
      value_name = r#""Monokai Extended", "ayu-dark""#,
      // group = "theme-name",
      // num_args = 0..=1,
      // default_missing_value = "",
      // help = get_args_text("theme"),
      // long_help = get_args_md("theme-help"),
    )]
  theme: Option<MiniStr>,

  #[arg(
      long,
      value_name = "/path/to/theme_file.dump",
      value_hint = clap::ValueHint::FilePath,
      // group = "theme-file",
      visible_alias = "tf",
      // requires = "theme",
      // help = get_args_text("theme-file"),
      // long_help = get_args_md("theme-file-help"),
    )]
  theme_file: Option<PathBuf>,
}
