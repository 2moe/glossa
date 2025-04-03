pub(crate) mod flattening;
mod init_lazy_data;
pub(crate) mod output_bincode;
pub(crate) mod output_match;
pub(crate) mod output_phf;
use std::{io, path::PathBuf, sync::OnceLock};

use getset::{Getters, MutGetters, WithSetters};
use glossa_shared::tap::Pipe;
pub use output_phf::to_lower_snake_case;

use crate::{
  L10nResources, MiniStr, Visibility,
  generator::flattening::{L10nDSLMaps, L10nMaps},
  internal_aliases::HighlightCfgMap,
};

#[derive(Getters, WithSetters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub", get_mut = "pub with_prefix")]
pub struct Generator<'h> {
  #[getset(skip)]
  #[getset(get = "pub")]
  resources: Box<L10nResources>,

  #[getset(get_mut)]
  visibility: Visibility,

  #[getset(skip)]
  #[getset(get = "pub", get_mut = "pub")]
  outdir: Option<PathBuf>,

  bincode_suffix: MiniStr,
  mod_prefix: MiniStr,

  #[getset(skip)]
  #[getset(get = "pub")]
  highlight: Option<Box<HighlightCfgMap<'h>>>,

  #[getset(skip)]
  /// get: `Self::get_or_init_*maps`
  lazy_maps: Box<LazyMaps>,
}

#[cfg(feature = "highlight")]
impl<'h> Generator<'h> {
  fn clear_highlight_cache(&mut self) {
    self.lazy_maps.highlight = Default::default();
    self.lazy_maps.merged = Default::default();
  }

  pub fn with_highlight(mut self, highlight: HighlightCfgMap<'h>) -> Self {
    self.highlight = Some(highlight.into());
    self.clear_highlight_cache();
    self
  }

  pub fn set_highlight(&mut self, highlight: Option<HighlightCfgMap<'h>>) {
    self.highlight = highlight.map(|data| data.into());
    self.clear_highlight_cache();
  }
}

impl Generator<'_> {
  pub fn with_outdir<P: Into<PathBuf>>(mut self, outdir: P) -> Self {
    self.outdir = Some(outdir.into());
    self
  }
}

impl Generator<'_> {
  /// Configures the generator with localization resources, resetting cached
  /// mappings.
  ///
  /// This method performs two primary actions:
  ///
  /// 1. Replaces the current [`L10nResources`] with the provided instance
  /// 2. Resets the internal `lazy_maps` to their default empty state
  ///
  /// # Side Effects
  ///
  /// The cache clearance ensures any existing lazy-loaded mappings won't
  /// conflict with new resources. Subsequent operations will rebuild mappings
  /// from the updated resources.
  ///
  /// # Example
  ///
  /// ```
  /// use glossa_codegen::{Generator, L10nResources};
  ///
  /// // Initialize resources from a localization directory
  /// let resources = L10nResources::new("../../locales/");
  ///
  /// // Create generator with configured resources
  /// let _generator = Generator::default()
  ///   .with_resources(resources);
  /// ```
  pub fn with_resources(mut self, resources: L10nResources) -> Self {
    self.resources = resources.into();
    self.lazy_maps = Default::default();
    self
  }
}

#[derive(Default, Debug, Clone)]
struct LazyMaps {
  /// get: [Generator::get_or_init_maps]
  regular: OnceLock<L10nMaps>,

  /// get: [Generator::get_or_init_highlight_maps]
  highlight: OnceLock<Option<L10nMaps>>,

  /// get: [Generator::get_or_init_dsl_maps]
  dsl: OnceLock<L10nDSLMaps>,

  /// get: [Generator::get_or_init_merged_maps]
  merged: OnceLock<L10nMaps>,
}

impl Default for Generator<'_> {
  /// Default:
  ///
  /// ```ignore
  /// {
  ///   bincode_suffix: ".bincode",
  ///   mod_prefix: "l10n_",
  ///   visibility: Visibility::PubCrate,
  ///   ..Default::default()
  /// }
  /// ```
  fn default() -> Self {
    Self {
      bincode_suffix: MiniStr::const_new(".bincode"),
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
  Highlight,
  RegularAndHighlight,
  DSL,
}

impl MapType {
  fn get_non_dsl_maps<'a>(
    &self,
    generator: &'a Generator<'a>,
  ) -> io::Result<&'a L10nMaps> {
    use MapType::*;
    match self {
      Regular => generator.get_or_init_maps(),
      Highlight => generator
        .get_or_init_highlight_maps()
        .ok_or_else(|| io::Error::other("Failed to get highlight maps"))?,
      RegularAndHighlight => generator.get_or_init_merged_maps(),
      _ => return io::Error::other("DSL Maps are not supported.").pipe(Err),
    }
    .pipe(Ok)
  }

  /// Returns `true` if the map type is [`DSL`].
  ///
  /// [`DSL`]: MapType::DSL
  #[must_use]
  pub fn is_dsl(&self) -> bool {
    matches!(self, Self::DSL)
  }
}

impl Default for MapType {
  fn default() -> Self {
    Self::DSL
  }
}

#[cfg(test)]
pub(crate) mod dbg_generator {

  use super::*;
  use crate::resources::dbg_shared;

  pub(crate) fn new_generator<'h>() -> Generator<'h> {
    let data = dbg_shared::new_resources();
    Generator::default()
      .with_resources(data)
      .with_outdir("tmp")
  }

  #[cfg(feature = "highlight")]
  pub(crate) fn highlight_generator<'h>() -> Generator<'h> {
    let hmap = crate::highlight::dbg_shared::new_highlight_map();
    new_generator().with_highlight(hmap)
  }

  pub(crate) fn en_generator<'h>() -> Generator<'h> {
    let data = dbg_shared::new_resources().with_include_languages(["en"]);
    new_generator().with_resources(data)
  }

  pub(crate) fn en_gb_generator<'h>() -> Generator<'h> {
    let data = dbg_shared::new_resources().with_include_languages(["en-GB"]);
    new_generator().with_resources(data)
  }

  pub(crate) fn es_generator<'h>() -> Generator<'h> {
    // let es = ["es"].into_iter().collect();
    let data = dbg_shared::new_resources().with_include_languages(["es"]);
    new_generator().with_resources(data)
  }

  pub(crate) fn de_en_fr_pt_zh_generator<'h>() -> Generator<'h> {
    let data = dbg_shared::new_resources()
      .with_include_languages([
        "de", // "zh-pinyin",
        "zh", "pt", "fr", "en", "en-GB",
      ])
      .with_include_map_names(["yes-no"]);
    new_generator().with_resources(data)
  }
}
