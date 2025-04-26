use std::{borrow::Cow, fs, path::PathBuf};

use glossa::MiniStr;
use glossa_codegen::{
  glossa_shared::tap::Pipe,
  highlight::{
    DerivedMapKey, SyntaxHighlightConfig,
    hlight::{self, HighlightResource},
  },
};

use super::to_kstr;
use crate::options::HighlightOpt;

pub(crate) fn collect_highlight_keys(args: &HighlightOpt) -> Box<[DerivedMapKey]> {
  args
    .get_base_name()
    .iter()
    .zip(args.get_suffix())
    .map(|(base, suffix)| {
      DerivedMapKey::default()
        .with_base_name(to_kstr(base))
        .with_suffix(to_kstr(suffix))
    })
    .collect()
}

pub(crate) fn collect_highlight_values(
  args: &HighlightOpt,
  len: usize,
) -> Box<[SyntaxHighlightConfig<'_>]> {
  (0..len)
    .map(|i| {
      let get_or_false = |v: &[bool]| {
        v.get(i)
          .copied()
          .unwrap_or(false)
      };

      let true_color = args
        .get_true_color()
        .pipe_deref(get_or_false);

      let background = args
        .get_background()
        .pipe_deref(get_or_false);

      let syntax_name = args
        .get_syntax_name()
        .get(i)
        .cloned()
        .unwrap_or("md".into());

      let try_read_file = |v: &[PathBuf]| {
        v.get(i)
          .and_then(|x| fs::read(x).ok())
      };

      let syntax_set = args
        .get_custom_syntax_set()
        .pipe_deref(try_read_file);

      let theme_set = args
        .get_custom_theme_set()
        .pipe_deref(try_read_file);

      let resource = HighlightResource::default()
        .with_background(background)
        .pipe(|res| {
          match args
            .get_theme_name()
            .get(i)
            .filter(|x: &&MiniStr| !x.is_empty())
          {
            Some(s) => res.with_theme_name(s.clone()),
            _ => res,
          }
        })
        .pipe(|res| {
          syntax_set
            .as_deref()
            .pipe(hlight::syntax::load_syntax_set)
            .pipe(|set| res.with_syntax_set(Cow::Owned(set)))
        })
        .pipe(|res| {
          theme_set
            .as_deref()
            .pipe(hlight::theme::load_theme_set)
            .pipe(|set| res.with_theme_set(set.into()))
        });

      SyntaxHighlightConfig::default()
        .with_syntax_name(syntax_name)
        .with_true_color(true_color)
        .with_resource(resource)
    })
    .collect()
}
