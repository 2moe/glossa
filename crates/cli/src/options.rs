use std::path::PathBuf;

use clap::{ColorChoice, Parser};
use getset::Getters;
use glossa::MiniStr;
use glossa_codegen::{Visibility, generator::MapType};

use crate::{context::get_text, envs::format_glossa_env};

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
#[command(version)]
#[command(arg_required_else_help = true)]
#[command(color = ColorChoice::Always)]
#[command(long_about = format_glossa_env())]
pub struct Cli {
  #[command(flatten)]
  resources: Box<ResourcesOpt>,

  #[command(flatten)]
  generator: Box<GeneratorOpt>,

  #[cfg(feature = "highlight")]
  #[command(flatten)]
  highlight: Box<HighlightOpt>,

  #[command(flatten)]
  dbg_options: DbgOpt,

  #[arg(
    long,
    value_name = "regular|highlight|dsl|_",
    default_value = "regular"
  )]
  map_type: MapType,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct DbgOpt {
  #[arg(
    long,
    help_heading = "Debug",
    help = get_text("display_config_dir", None),
  )]
  display_config_dir: bool,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct GeneratorOpt {
  #[arg(
    long,
    short = 'o',
    value_hint = clap::ValueHint::DirPath,
    help_heading = "Generator Core",
    value_name = "/path/to/output_dir",
    // long_help = get_args_md(b"outdir"),
    help = get_text("outdir", None),
  )]
  outdir: Option<PathBuf>,

  #[arg(
    long,
    visible_alias = "vis",
    value_name = "pub| pub(crate)| pub(super)| private",
    help_heading = "Generator Core",
    help = get_text("visibility", None),
  )]
  visibility: Option<Visibility>,

  #[arg(
    long,
    visible_alias = "mod-vis",
    value_name = "pub| pub(crate)| pub(super)| private",
    help_heading = "Generator Core"
  )]
  mod_visibility: Option<Visibility>,

  #[arg(
    long,
    short = 'b',
    value_name = "String",
    help_heading = "Generator Core",
    help = get_text("bincode_suffix", None),
  )]
  bincode_suffix: Option<MiniStr>,

  #[arg(
    long,
    short = 'm',
    value_name = "String",
    help_heading = "Generator Core",
    help = get_text("mod_prefix", None),
  )]
  mod_prefix: Option<MiniStr>,

  #[arg(
    long,
    short = 'f',
    value_name = "String",
    help_heading = "Generator Core"
  )]
  feature_prefix: Option<MiniStr>,

  // -------
  #[arg(long, help_heading = "Output Files",
    help = get_text("output_bincode", None),
  )]
  output_bincode: bool,

  #[arg(long, help_heading = "Output File",
    help = get_text("output_bincode_all_in_one", None),
  )]
  output_bincode_all_in_one: bool,

  #[arg(long, help_heading = "Output Files", group = "output_map_fn",
    help = get_text("output_match_fn", None),
  )]
  output_match_fn: bool,

  #[arg(long, help_heading = "Output Files", group = "output_map_fn",
    help = get_text("output_match_fn_without_map_name", None),
  )]
  output_match_fn_without_map_name: bool,

  #[arg(long, help_heading = "Output File",
    help = get_text("output_match_fn_all_in_one", None),
  )]
  output_match_fn_all_in_one: bool,

  #[arg(long, help_heading = "Output File",
    help = get_text("output_match_fn_all_in_one_by_language", None),
  )]
  output_match_fn_all_in_one_by_language: bool,

  #[arg(long, help_heading = "Output File",
    help = get_text("output_match_fn_all_in_one_without_map_name", None),
  )]
  output_match_fn_all_in_one_without_map_name: bool,

  #[arg(long, help_heading = "Output Files", group = "output_map_fn",
    help = get_text("output_phf", None),
  )]
  output_phf: bool,

  #[arg(long, help_heading = "Output Files", group = "output_map_fn",
    help = get_text("output_phf_without_map_name", None),
  )]
  output_phf_without_map_name: bool,

  #[arg(long, help_heading = "Output File",
    help = get_text("output_phf_all_in_one", None),
  )]
  output_phf_all_in_one: bool,

  #[cfg(feature = "ron")]
  #[arg(long, help_heading = "Output String",
    help = get_text("output_ron", None),
  )]
  output_ron: bool,

  #[arg(long, help_heading = "Output String",
    help = get_text("output_locales_fn", None),
  )]
  output_locales_fn: bool,

  #[arg(long, help_heading = "Output String")]
  output_raw_locales: bool,

  #[arg(long, help_heading = "Output String")]
  output_mod_rs: bool,

  #[arg(long, help_heading = "Output String")]
  output_cargo_features: bool,

  #[arg(long, help_heading = "Output String")]
  output_router_for_match_fns: bool,

  #[arg(long, help_heading = "Output String")]
  output_router_for_match_fns_without_map_name: bool,

  #[arg(long, help_heading = "Output String")]
  output_router_for_phf_maps: bool,

  #[arg(long, help_heading = "Output String")]
  output_router_for_phf_maps_without_map_name: bool,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct ResourcesOpt {
  #[arg(
    long,
    short = 'i',
    help_heading = "L10nResources",
    value_name = "/path/to/L10nDir",
    default_value = "locales",
    help = get_text("input", None),
  )]
  input: PathBuf,

  #[arg(long, help_heading = "L10nResources", value_name = "String",
    help = get_text("dsl_suffix", None),
  )]
  dsl_suffix: Option<MiniStr>,

  #[arg(
    long,
    visible_alias = "langs",
    help_heading = "L10nResources",
    value_name = "en,zh,fr,ru,ar,es,etc.",
    value_delimiter = ',',
    help = get_text("include_languages", None),
  )]
  include_languages: Vec<MiniStr>,

  #[arg(
    long,
    visible_alias = "maps",
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ',',
    help = get_text("include_map_names", None),
  )]
  include_map_names: Vec<MiniStr>,

  #[arg(
    long,
    visible_alias = "ex-langs",
    help_heading = "L10nResources",
    value_name = "lang1,lang2,..",
    value_delimiter = ',',
    help = get_text("exclude_languages", None),
  )]
  exclude_languages: Vec<MiniStr>,

  #[arg(
    long,
    visible_alias = "ex-maps",
    value_name = "Vec<map_name>",
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ',',
    help = get_text("exclude_map_names", None),
  )]
  exclude_map_names: Vec<MiniStr>,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct HighlightOpt {
  #[arg(long, help_heading = "HighlightKey", value_name = "Vec<String>",
    help = get_text("base_name", None),
  )]
  base_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightKey", value_name = "Vec<String>",
    help = get_text("suffix", None),
  )]
  suffix: Vec<MiniStr>,

  // --------
  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<bool>",
    help = get_text("true_color", None),
  )]
  true_color: Vec<bool>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<String>",
    help = get_text("syntax_name", None),
  )]
  syntax_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<String>",
    help = get_text("theme_name", None),
  )]
  theme_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<bool>",
    help = get_text("background", None),
  )]
  background: Vec<bool>,

  #[arg(
    long,
    help_heading = "HighlightValue",
    value_name = r#"Vec<"/path/to/syntaxset-file">"#,
    value_hint = clap::ValueHint::FilePath,
    help = get_text("custom_syntax_set", None),
  )]
  custom_syntax_set: Vec<PathBuf>,

  #[arg(
    long,
    help_heading = "HighlightValue",
    value_name = r#"Vec<"/path/to/themeset-file">"#,
    value_hint = clap::ValueHint::FilePath,
    help = get_text("custom_theme_set", None),
  )]
  custom_theme_set: Vec<PathBuf>,

  #[arg(long, help_heading = "HighlightValue Debug",
    help = get_text("list_all_syntaxes", None),
  )]
  list_all_syntaxes: bool,

  #[arg(long, help_heading = "HighlightValue Debug",
    help = get_text("list_all_themes", None),
  )]
  list_all_themes: bool,
}
