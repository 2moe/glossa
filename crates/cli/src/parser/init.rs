use glossa_codegen::{
  Generator, L10nResources, glossa_shared::tap::Pipe, highlight::HighlightCfgMap,
};

use crate::{
  options::Cli,
  parser::{collect_highlight_keys, collect_highlight_values},
};

pub(crate) fn init_highlight_cfg_map(args: &Cli) -> HighlightCfgMap<'_> {
  let raw = args.get_highlight();
  let keys = collect_highlight_keys(raw);
  let values = collect_highlight_values(raw, keys.len());

  keys
    .iter()
    .zip(values)
    .map(|(k, v)| (k.clone(), v))
    .collect()
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
      // _ => data,
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
  log::trace!("raw resources args: {raw:#?}");

  raw
    .get_input()
    .pipe(L10nResources::new)
    .pipe(|res| match raw.get_dsl_suffix() {
      Some(suffix) => res.with_dsl_suffix(suffix.clone()),
      _ => res,
    })
    .pipe(|res| {
      match raw
        .get_include_languages()
        .as_slice()
      {
        [] => res,
        langs => res.with_include_languages(langs.iter().cloned()),
      }
    })
    .pipe(|res| {
      match raw
        .get_exclude_languages()
        .as_slice()
      {
        [] => res,
        langs => res.with_exclude_languages(langs.iter().cloned()),
      }
    })
    .pipe(|res| {
      match raw
        .get_include_map_names()
        .as_slice()
      {
        [] => res,
        names => res.with_include_map_names(names.iter().cloned()),
      }
    })
    .pipe(|res| {
      match raw
        .get_exclude_map_names()
        .as_slice()
      {
        [] => res,
        names => res.with_exclude_map_names(names.iter().cloned()),
      }
    })
}
