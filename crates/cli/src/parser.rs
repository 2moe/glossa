use std::{borrow::Cow, fs};

use clap::Parser;
use glossa::MiniStr;
use glossa_codegen::{
  AnyResult,
  glossa_shared::tap::Pipe,
  highlight::{
    DerivedMapKey, KString, SyntaxHighlightConfig,
    hlight::{self, HighlightResource},
  },
};
mod init;
mod output;

use crate::{
  options::{Cli, HighlightOpt},
  parser::output::output_data,
};

impl Cli {
  pub fn run() -> AnyResult<()> {
    let args = Cli::parse();
    log::trace!("args: {:#?}", args);

    let generator = init::init_generator(&args)
      .with_resources(init::init_resources(&args))
      .with_highlight(init::init_highlight_cfg_map(&args));

    output_data(&args, &generator)?;

    Ok(())
  }
}

pub fn puts<T: core::fmt::Display>(msg: &T) {
  println!("{msg}")
}

fn to_kstr(s: &MiniStr) -> KString {
  KString::from_ref(s)
}

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
  let true_color_list = args.get_true_color();
  let syntax_names = args.get_syntax_name();
  let theme_names = args.get_theme_name();
  let background_list = args.get_background();
  let syntaxes = args.get_custom_syntax_set();
  let themes = args.get_custom_theme_set();

  (0..len)
    .map(|i| {
      let true_color = true_color_list
        .get(i)
        .copied()
        .unwrap_or(false);

      let syntax_name = syntax_names
        .get(i)
        .cloned()
        .unwrap_or("md".into());

      let background = background_list
        .get(i)
        .copied()
        .unwrap_or(false);

      let theme_name = theme_names.get(i);

      let syntax_set = syntaxes
        .get(i)
        .and_then(|x| fs::read(x).ok());
      let theme_set = themes
        .get(i)
        .and_then(|x| fs::read(x).ok());

      let resource = HighlightResource::default()
        .with_background(background)
        .pipe(
          |res| match theme_name.filter(|x: &&MiniStr| !x.is_empty()) {
            Some(s) => res.with_theme_name(s.clone()),
            _ => res,
          },
        )
        .pipe(|res| {
          syntax_set
            .as_ref()
            .map(|data| data.as_ref())
            .pipe(hlight::syntax::load_syntax_set)
            .pipe(|x| res.with_syntax_set(Cow::Owned(x)));

          //
          todo!()
        });

      SyntaxHighlightConfig::default()
        .with_syntax_name(syntax_name)
        .with_true_color(true_color)
        .with_resource(resource)
    })
    .collect()
}
