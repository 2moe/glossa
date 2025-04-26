use std::path::PathBuf;

use clap::{ColorChoice, Parser};
use getset::Getters;
use glossa_codegen::{MiniStr, Visibility, generator::MapType};

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub with_prefix")]
#[command(arg_required_else_help = true)]
#[command(color = ColorChoice::Always)]
pub struct Cli {
  #[command(flatten)]
  resources: Box<ResourcesOpt>,

  #[command(flatten)]
  generator: Box<GeneratorOpt>,

  #[command(flatten)]
  highlight: Box<HighlightOpt>,

  #[arg(
    long,
    value_name = "regular|highlight|dsl|_",
    default_value = "regular"
  )]
  map_type: MapType,
}

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub with_prefix")]
struct GeneratorOpt {
  #[arg(
    long,
    value_hint = clap::ValueHint::DirPath,
    help_heading = "Generator",
    value_name = "/path/to/output_dir",
    // help = get_args_text("theme-file"),
    // long_help = get_args_md("theme-file-help"),
  )]
  outdir: Option<PathBuf>,

  #[arg(
    long,
    visible_alias = "vis",
    value_name = "pub | pub(crate)",
    help_heading = "Generator"
  )]
  visibility: Option<Visibility>,

  #[arg(long, value_name = "string", help_heading = "Generator")]
  bincode_suffix: Option<MiniStr>,

  #[arg(long, value_name = "string", help_heading = "Generator")]
  mod_prefix: Option<MiniStr>,

  // -------
  #[arg(long, help_heading = "Output")]
  output_bincode: bool,

  #[arg(long, help_heading = "Output")]
  output_bincode_all_in_one: bool,

  #[arg(long, help_heading = "Output", group = "output_map_fn")]
  output_match_fn: bool,

  #[arg(long, help_heading = "Output")]
  output_match_fn_all_in_one: bool,

  #[arg(long, help_heading = "Output")]
  output_match_fn_all_in_one_by_language: bool,

  #[arg(long, help_heading = "Output")]
  output_match_fn_all_in_one_by_language_and_key: bool,

  #[arg(long, help_heading = "Output", group = "output_map_fn")]
  output_phf: bool,

  #[arg(long, help_heading = "Output")]
  output_phf_all_in_one: bool,

  #[arg(long, help_heading = "Output")]
  output_locales_fn: bool,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
struct ResourcesOpt {
  #[arg(long, help_heading = "L10nResources", value_name = "/path/to/L10nDir")]
  input: Option<PathBuf>,

  #[arg(long, help_heading = "L10nResources", value_name = "string")]
  dsl_suffix: Option<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "en,zh,fr,ru,ar,es,etc.",
    value_delimiter = ','
  )]
  include_languages: Vec<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ','
  )]
  include_map_names: Vec<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "lang1,lang2,..",
    value_delimiter = ','
  )]
  exclude_languages: Vec<MiniStr>,

  #[arg(
    long,
    value_name = "Vec<map_name>",
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ','
  )]
  exclude_map_names: Vec<MiniStr>,
}

#[derive(Parser, Debug, Getters)]
#[getset(get = "pub with_prefix")]
struct HighlightOpt {
  #[arg(long, help_heading = "Highlight", value_name = "string")]
  base_name: Option<MiniStr>,

  #[arg(long, help_heading = "Highlight", value_name = "string")]
  suffix: Option<MiniStr>,

  #[arg(
    long,
    value_name = r#""Monokai Extended", "ayu-dark""#,
    help_heading = "Highlight",
    // group = "theme-name",
    // num_args = 0..=1,
    // default_missing_value = "",
    // help = get_args_text("theme"),
    // long_help = get_args_md("theme-help"),
  )]
  theme: Option<MiniStr>,
}
