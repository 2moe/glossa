pub(crate) mod flattening;
mod init_lazy_data;
pub(crate) mod output_bincode;
pub(crate) mod output_match;
pub(crate) mod output_phf;
use std::{io, path::PathBuf, sync::OnceLock};

use getset::{Getters, MutGetters, WithSetters};
pub use output_phf::to_lower_snake_case;
use tap::Pipe;

use crate::{
  L10nResources, MiniStr, Visibility,
  generator::flattening::{L10nMaps, L10nTemplateMaps},
};

#[derive(Getters, WithSetters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub", get_mut = "pub with_prefix")]
pub struct Generator<'i, 'h> {
  #[getset(skip)]
  #[getset(get = "pub")]
  resources: Box<L10nResources<'i>>,

  #[getset(get_mut)]
  visibility: Visibility,

  outdir: Option<PathBuf>,
  bincode_suffix: MiniStr,
  mod_prefix: MiniStr,
  highlight: Option<&'h crate::internal_aliases::HighlightCfgMap<'h>>,

  #[getset(skip)]
  /// get: `Self::get_or_init_*maps`
  lazy_maps: Box<LazyMaps>,
}

impl<'i> Generator<'i, '_> {
  pub fn with_resources(mut self, resources: L10nResources<'i>) -> Self {
    self.lazy_maps = Default::default();
    self.resources = resources.into();
    self
  }
}

#[derive(Default, Debug, Clone)]
struct LazyMaps {
  /// get: [Generator::get_or_init_maps]
  regular: OnceLock<L10nMaps>,

  /// get: [Generator::get_or_init_highlight_maps]
  highlight: OnceLock<Option<L10nMaps>>,

  /// get: [Generator::get_or_init_template_maps]
  template: OnceLock<L10nTemplateMaps>,

  /// get: [Generator::get_or_init_merged_maps]
  merged: OnceLock<L10nMaps>,
}

impl Default for Generator<'_, '_> {
  fn default() -> Self {
    Self {
      bincode_suffix: Default::default(),
      outdir: Default::default(),
      resources: Default::default(),
      // match_bound: 200,
      // doc: true,
      mod_prefix: MiniStr::const_new("l10n_"),
      // cargo_feature_prefix: "l10n-".into(),
      // overwrite: true,
      visibility: Default::default(),
      // visibility_mod: Default::default(),
      highlight: Default::default(),
      lazy_maps: Default::default(),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum MapType {
  Regular,
  Hightlight,
  RegularAndHighlight,
  Template,
}

impl MapType {
  fn get_non_template_maps<'a>(
    &self,
    generator: &'a Generator<'_, 'a>,
  ) -> io::Result<&'a L10nMaps> {
    use MapType::*;
    match self {
      Regular => generator.get_or_init_maps(),
      Hightlight => generator
        .get_or_init_highlight_maps()
        .ok_or_else(|| io::Error::other("Failed to get highlight maps"))?,
      RegularAndHighlight => generator.get_or_init_merged_maps(),
      _ => return io::Error::other("Template Maps are not supported.").pipe(Err),
    }
    .pipe(Ok)
  }

  /// Returns `true` if the map type is [`Template`].
  ///
  /// [`Template`]: MapType::Template
  #[must_use]
  pub fn is_template(&self) -> bool {
    matches!(self, Self::Template)
  }
}

impl Default for MapType {
  fn default() -> Self {
    Self::Template
  }
}

#[cfg(test)]
pub(crate) mod dbg_generator {
  use std::path::Path;

  use super::*;
  use crate::resources::dbg_shared;

  fn tmp_dir() -> Option<PathBuf> {
    Path::new("tmp")
      .to_owned()
      .into()
  }

  pub(crate) fn new_generator<'i, 'h>() -> Generator<'i, 'h> {
    let data = dbg_shared::new_resources();
    Generator::default()
      .with_resources(data)
      .with_outdir(tmp_dir())
  }

  pub(crate) fn en_generator<'i, 'h>() -> Generator<'i, 'h> {
    let data = dbg_shared::new_resources().with_include_languages(Some(&["en"]));

    Generator::default()
      .with_resources(data)
      .with_outdir(tmp_dir())
  }
}
