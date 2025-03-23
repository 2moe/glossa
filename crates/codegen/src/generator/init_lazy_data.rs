use crate::{
  Generator,
  generator::flattening::{L10nMaps, L10nTemplateMaps},
};

impl Generator<'_, '_> {
  pub fn get_or_init_template_maps(&self) -> &L10nTemplateMaps {
    self
      .lazy_maps
      .template
      .get_or_init(|| self.flatten_template_maps())
  }

  pub fn get_or_init_maps(&self) -> &L10nMaps {
    self
      .lazy_maps
      .regular
      .get_or_init(|| self.flatten_l10n_maps())
  }
}

impl<'h> Generator<'_, 'h> {
  pub fn get_or_init_highlight_maps(&'h self) -> Option<&'h L10nMaps> {
    self
      .lazy_maps
      .highlight
      .get_or_init(|| self.flatten_highlight_maps())
      .as_ref()
  }

  /// Merges [Self::get_or_init_maps()] & [Self::get_or_init_highlight_maps()]
  pub fn get_or_init_merged_maps(&'h self) -> &'h L10nMaps {
    if self.get_highlight().is_none() {
      return self.get_or_init_maps();
    }

    self
      .lazy_maps
      .merged
      .get_or_init(|| self.merge_l10n_and_highlight_maps())
  }
}
