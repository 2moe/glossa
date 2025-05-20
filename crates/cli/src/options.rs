use std::path::PathBuf;

use clap::{ColorChoice, Parser};
use getset::Getters;
use glossa::{LocaleContext, MiniStr, sys::new_once_lock, traits::ChainProvider};
use glossa_codegen::{Visibility, generator::MapType};

trait GetL10nText: ChainProvider {
  fn try_get<'t>(&self, map_name: &str, key: &[u8]) -> Option<&'t str> {
    let lookup = |(language, map_name, key)| {
      let _map = match map_name {
        "cli" => crate::l10n::all::map,
        // todo: +more maps
        _ => crate::l10n::all::map,
      };
      match _map(language, key) {
        "" => None,
        s => Some(s),
      }
    };

    self
      .provide_chain()?
      .iter()
      .map(|lang| (lang.as_bytes(), map_name, key))
      .find_map(lookup)
  }
}

impl GetL10nText for LocaleContext {}

fn static_locale_context() -> &'static LocaleContext {
  new_once_lock!(L: LocaleContext);
  L.get_or_init(|| {
    LocaleContext::default()
      .with_all_locales(crate::l10n::locale_registry::all_locales())
  })
}

fn get_static_text<'a>(key: &[u8], map_name: Option<&str>) -> &'a str {
  let map_name = map_name.unwrap_or("cli");

  static_locale_context() //
    .try_get(map_name, key)
    .unwrap_or_default()
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
#[command(version)]
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
  // #[arg(long, help_heading = "Local", value_name = "e.g., en, zh, de, es.")]
  // language: Option<String>,

  // #[arg(long, help_heading = "Local", value_name = "/path/to/all.bincode")]
  // load_custom_1l0n_bincode: Option<PathBuf>,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct GeneratorOpt {
  #[arg(
    long,
    value_hint = clap::ValueHint::DirPath,
    help_heading = "Generator",
    value_name = "/path/to/output_dir",
    // help = get_args_text("theme-file"),
    // long_help = get_args_md("theme-file-help"),
    help = get_static_text(b"outdir", None),
  )]
  outdir: Option<PathBuf>,

  #[arg(
    long,
    visible_alias = "vis",
    value_name = "pub | pub(crate)",
    help_heading = "Generator",
    help = get_static_text(b"visibility", None),
  )]
  visibility: Option<Visibility>,

  #[arg(long, value_name = "string", help_heading = "Generator",
    help = get_static_text(b"bincode_suffix", None),
  )]
  bincode_suffix: Option<MiniStr>,

  #[arg(long, value_name = "string", help_heading = "Generator",
  help = get_static_text(b"mod_prefix", None),
  )]
  mod_prefix: Option<MiniStr>,

  // -------
  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_bincode", None),
  )]
  output_bincode: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_bincode_all_in_one", None),
  )]
  output_bincode_all_in_one: bool,

  #[arg(long, help_heading = "Output", group = "output_map_fn",
    help = get_static_text(b"output_match_fn", None),
  )]
  output_match_fn: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_match_fn_all_in_one", None),
  )]
  output_match_fn_all_in_one: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_match_fn_all_in_one_by_language", None),
  )]
  output_match_fn_all_in_one_by_language: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_match_fn_all_in_one_by_language_and_key", None),
  )]
  output_match_fn_all_in_one_by_language_and_key: bool,

  #[arg(long, help_heading = "Output", group = "output_map_fn",
    help = get_static_text(b"output_phf", None),
  )]
  output_phf: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_phf_all_in_one", None),
  )]
  output_phf_all_in_one: bool,

  #[arg(long, help_heading = "Output",
    help = get_static_text(b"output_locales_fn", None),
  )]
  output_locales_fn: bool,

  #[arg(long, help_heading = "Output Debug",
    help = get_static_text(b"output_ron", None),
  )]
  output_ron: bool,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct ResourcesOpt {
  #[arg(long, help_heading = "L10nResources", value_name = "/path/to/L10nDir",
    help = get_static_text(b"input", None),
  )]
  input: PathBuf,

  #[arg(long, help_heading = "L10nResources", value_name = "string",
    help = get_static_text(b"dsl_suffix", None),
  )]
  dsl_suffix: Option<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "en,zh,fr,ru,ar,es,etc.",
    value_delimiter = ',',
    help = get_static_text(b"include_languages", None),
  )]
  include_languages: Vec<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ',',
    help = get_static_text(b"include_map_names", None),
  )]
  include_map_names: Vec<MiniStr>,

  #[arg(
    long,
    help_heading = "L10nResources",
    value_name = "lang1,lang2,..",
    value_delimiter = ',',
    help = get_static_text(b"exclude_languages", None),
  )]
  exclude_languages: Vec<MiniStr>,

  #[arg(
    long,
    value_name = "Vec<map_name>",
    help_heading = "L10nResources",
    value_name = "name1,name2,..",
    value_delimiter = ',',
    help = get_static_text(b"exclude_map_names", None),
  )]
  exclude_map_names: Vec<MiniStr>,
}

#[derive(Parser, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct HighlightOpt {
  #[arg(long, help_heading = "HighlightKey", value_name = "Vec<String>",
    help = get_static_text(b"base_name", None),
  )]
  base_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightKey", value_name = "Vec<String>",
    help = get_static_text(b"suffix", None),
  )]
  suffix: Vec<MiniStr>,

  // --------
  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<bool>",
    help = get_static_text(b"true_color", None),
  )]
  true_color: Vec<bool>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<String>",
    help = get_static_text(b"syntax_name", None),
  )]
  syntax_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<String>",
    help = get_static_text(b"theme_name", None),
  )]
  theme_name: Vec<MiniStr>,

  #[arg(long, help_heading = "HighlightValue", value_name = "Vec<bool>",
    help = get_static_text(b"background", None),
  )]
  background: Vec<bool>,

  #[arg(
    long,
    help_heading = "HighlightValue",
    value_name = r#"Vec<"/path/to/syntaxset-file">"#,
    value_hint = clap::ValueHint::FilePath,
    help = get_static_text(b"custom_syntax_set", None),
  )]
  custom_syntax_set: Vec<PathBuf>,

  #[arg(
    long,
    help_heading = "HighlightValue",
    value_name = r#"Vec<"/path/to/themeset-file">"#,
    value_hint = clap::ValueHint::FilePath,
    help = get_static_text(b"custom_theme_set", None),
  )]
  custom_theme_set: Vec<PathBuf>,

  #[arg(long, help_heading = "HighlightValue Debug",
    help = get_static_text(b"show_all_syntaxes", None),
  )]
  show_all_syntaxes: bool,

  #[arg(long, help_heading = "HighlightValue Debug",
    help = get_static_text(b"show_all_themes", None),
  )]
  show_all_themes: bool,
}
