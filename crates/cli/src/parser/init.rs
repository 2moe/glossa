use glossa_codegen::{
  Generator, L10nResources, glossa_shared::tap::Pipe, highlight::HighlightCfgMap,
};
use log::{error, trace};

use crate::{
  options::Cli,
  parser::collect::{collect_highlight_keys, collect_highlight_values},
};

fn show_syntaxes(map: &HighlightCfgMap) -> Option<()> {
  map
    .values()
    .next()
    .map(|v| {
      v.get_resource()
        .get_syntax_set()
        .syntaxes()
    })?
    .iter()
    .for_each(|x| {
      let (name, exts) = (&x.name, &x.file_extensions);
      println!(
        "name: {name}\n\
        exts: {exts:?}\n---"
      )
    });
  Some(())
}

fn show_themes(map: &HighlightCfgMap) -> Option<()> {
  map
    .values()
    .next()
    .map(|v| {
      v.get_resource()
        .get_theme_set()
        .get_inner()
        .themes
        .keys()
    })?
    .for_each(|k| println!("{k}"));

  Some(())
}

pub(crate) fn init_highlight_cfg_map(args: &Cli) -> Option<HighlightCfgMap<'_>> {
  let raw = args.get_highlight();
  if raw.get_base_name().is_empty() || raw.get_suffix().is_empty() {
    if *raw.get_list_all_syntaxes() || *raw.get_list_all_themes() {
      error!("Both `--base-name` and `--suffix` must be specified")
    }
    None?
  }

  let keys = match collect_highlight_keys(raw) {
    x if x.is_empty() => None?,
    x => x,
  };
  let values = collect_highlight_values(raw, keys.len());

  let map = keys
    .iter()
    .zip(values)
    .map(|(k, v)| (k.clone(), v))
    .collect();
  trace!("highlight cfg map: {map:#?}");

  raw
    .get_list_all_syntaxes()
    .then(|| show_syntaxes(&map));
  raw
    .get_list_all_themes()
    .then(|| show_themes(&map));

  Some(map)
}

pub(crate) fn init_generator(args: &Cli) -> Generator<'_> {
  let raw = args.get_generator();
  log::trace!("raw generator args: {raw:#?}");

  Generator::default()
    .with_visibility(
      raw
        .get_visibility()
        .clone()
        .unwrap_or_default(),
    )
    .pipe(|data| match raw.get_outdir() {
      Some(outdir) => data.with_outdir(outdir),
      _ => data.with_outdir("tmp"),
    })
    .pipe(|data| match raw.get_bincode_suffix() {
      Some(suffix) => data.with_bincode_suffix(suffix.clone()),
      _ => data,
    })
    .pipe(|data| match raw.get_mod_prefix() {
      Some(prefix) => data.with_mod_prefix(prefix.clone()),
      _ => data,
    })
}

pub(crate) fn init_resources(args: &Cli) -> L10nResources {
  let raw = args.get_resources();
  trace!("raw resources args: {raw:#?}");

  raw
    .get_input()
    .pipe(L10nResources::new)
    .pipe(|res| raw.update_dsl_suffix(res))
    .pipe(|res| raw.update_include_languages(res))
    .pipe(|res| raw.update_exclude_languages(res))
    .pipe(|res| raw.update_include_map_names(res))
    .pipe(|res| raw.update_exclude_map_names(res))
}
