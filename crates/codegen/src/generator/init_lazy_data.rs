use crate::{
  Generator,
  generator::flattening::{L10nMaps, L10nTemplateMaps},
};

impl Generator<'_, '_> {
  /// Initializes and retrieves template-based localization maps
  ///
  /// # Behavior
  /// - Performs lazy initialization of template maps on first access
  /// - Subsequent calls return cached reference
  pub fn get_or_init_template_maps(&self) -> &L10nTemplateMaps {
    self
      .lazy_maps
      .template
      .get_or_init(|| self.flatten_template_maps())
  }

  /// Initializes and retrieves primary localization maps
  ///
  /// # Behavior
  /// - Lazily loads core localization data on first access
  /// - Caches results for subsequent requests
  pub fn get_or_init_maps(&self) -> &L10nMaps {
    self
      .lazy_maps
      .regular
      .get_or_init(|| self.flatten_l10n_maps())
  }
}

impl<'h> Generator<'_, 'h> {
  /// Initializes and retrieves highlighted localization maps
  ///
  /// # Behavior
  /// - Lazy initialization of special highlight entries
  /// - Only initializes if highlight configuration exists
  pub fn get_or_init_highlight_maps(&'h self) -> Option<&'h L10nMaps> {
    self
      .lazy_maps
      .highlight
      .get_or_init(|| self.flatten_highlight_maps())
      .as_ref()
  }

  /// Provides merged view of localization data
  ///
  /// > Merges [Self::get_or_init_maps()] & [Self::get_or_init_highlight_maps()]
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
