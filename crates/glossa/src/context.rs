use std::{collections::HashSet, sync::OnceLock};

use compact_str::ToCompactString;
use getset::Getters;
use lang_id::LangID;
use log::warn;
use tap::Pipe;

use crate::{MiniStr, fallback::LocaleStrChain, sys::get_static_locale};

/// A context holder for locale-related information and fallback chains.
/// Manages current locale, supported locales, and cached fallback chains.
#[derive(Default, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct LocaleContext {
  /// Current active locale (initialized lazily)
  pub(crate) current_locale: OnceLock<LangID>,

  /// Cached locale fallback chain (e.g., ["en-NZ", "en-GB" "en"])
  #[getset(skip)]
  pub(crate) chain: OnceLock<LocaleStrChain>,

  /// All available locales in the application
  pub(crate) all_locales: Option<Box<[LangID]>>,
}

impl LocaleContext {
  /// Configures all supported locales and resets cached chain
  pub fn with_all_locales<I: Into<Box<[LangID]>>>(mut self, locales: I) -> Self {
    self.chain.take();
    self.init_static_locale_if_uninitialized();
    self.all_locales = Some(locales.into());
    self
  }

  /// - all_locales:
  ///   - `Some(Box<[LangID]>)` => `Vec<MiniStr>`
  ///   - None => `vec![]`
  pub fn collect_all_locales_to_vec(&self) -> Vec<MiniStr> {
    match self.get_all_locales() {
      Some(v) => v
        .iter()
        .map(|x| x.to_compact_string())
        .collect(),
      _ => Default::default(),
    }
  }

  /// By default, LocaleStrChain is automatically generated.
  /// By invoking the `try_push_front_with_custom_chain()` method,
  /// you can prioritize your custom chain over the automatically generated
  /// chain.
  ///
  /// > This method automatically removes duplicate elements.
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa::LocaleContext;
  ///
  /// let mut ctx = LocaleContext::default()
  ///   .with_all_locales(glossa_l10n::error::locale_registry::all_locales())
  ///   .with_current_locale("gsw".parse().ok());
  ///
  /// let old = ctx.get_or_try_init_chain();
  /// assert_eq!(
  ///   old,
  ///   Some(
  ///     ["de", "en"]
  ///       .map(Into::into)
  ///       .as_ref()
  ///   )
  /// );
  ///
  /// const LANGUAGE: &str = "es:fr:pt:en";
  /// // let custom = ["es", "fr", "pt", "en"].map(Into::into);
  /// let custom = LANGUAGE
  ///   .split(':')
  ///   .map(Into::into)
  ///   .collect::<Vec<_>>();
  ///
  /// let _ = ctx.try_push_front_with_custom_chain(&custom);
  /// let new = ctx.get_or_try_init_chain();
  ///
  /// assert_eq!(
  ///   new,
  ///   Some(
  ///     ["es", "fr", "pt", "en", "de"]
  ///       .map(Into::into)
  ///       .as_ref()
  ///   )
  /// )
  /// ```
  pub fn try_push_front_with_custom_chain(
    &mut self,
    custom: &[MiniStr],
  ) -> Result<(), LocaleStrChain> {
    let new = {
      let old = self
        .get_or_try_init_chain()
        .unwrap_or_default();
      let mut seen = HashSet::new();

      custom
        .iter()
        .chain(old.iter())
        .filter(|&x| seen.insert(x))
        .cloned()
        .collect()
    };

    self.chain.take();
    self.chain.set(new)
  }

  /// Initializes static locale if not already set
  pub(crate) fn init_static_locale_if_uninitialized(&self) {
    if self.is_current_locale_initialized() {
      return;
    }

    self
      .current_locale
      .get_or_init(|| get_static_locale().clone());
  }

  /// Checks if current locale has been initialized
  pub(crate) fn is_current_locale_initialized(&self) -> bool {
    self
      .current_locale
      .get()
      .is_some()
  }

  /// Checks if locale chain has been computed
  pub fn is_chain_initialized(&self) -> bool {
    self.chain.get().is_some()
  }

  /// Updates current locale and resets cached chain
  ///
  /// > If the `current` value is None, `self.current_locale` will use the
  /// > system default language.
  pub fn with_current_locale(mut self, current: Option<LangID>) -> Self {
    self.current_locale.take();
    self
      .current_locale
      .get_or_init(|| match current {
        Some(x) => x,
        _ => get_static_locale().clone(),
      });
    self.chain.take();
    self
  }

  /// Gets cached chain or initializes it
  pub fn get_or_try_init_chain(&self) -> Option<&[MiniStr]> {
    let all_locales = match self.is_chain_initialized() {
      true => Default::default(),
      _ => match self
        .get_all_locales()
        .as_deref()
        .filter(|x| !x.is_empty())
      {
        Some(x) => x,
        _ => {
          warn!("all_locales is empty");
          None?
        }
      },
    };

    self
      .chain
      .get_or_init(|| {
        self.init_static_locale_if_uninitialized();
        let current = self
          .current_locale
          .get()
          .expect("current_locale: Empty");

        crate::fallback::init_str_chain(current, all_locales)
      })
      .as_ref()
      .pipe(Some)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fallback::dbg_shared::init_logger;

  #[ignore]
  #[test]
  fn test_push_front_chain() {
    init_logger(false);
    let mut ctx = LocaleContext::default()
      .with_all_locales(glossa_l10n::error::locale_registry::all_locales())
      .with_current_locale("gsw".parse().ok());
    let old = ctx.get_or_try_init_chain();
    assert_eq!(
      old,
      Some(
        ["de", "en"]
          .map(Into::into)
          .as_ref()
      )
    );
    log::info!("old: {old:?}"); // => Some(["de", "en"])

    const LANGUAGE: &str = "es:fr:pt:en";
    let custom = LANGUAGE
      .split(':')
      .map(Into::into)
      .collect::<Vec<_>>(); // let custom = ["es", "fr", "pt", "en"].map(Into::into);

    let _ = ctx.try_push_front_with_custom_chain(&custom);
    let new = ctx.get_or_try_init_chain();
    log::info!("new: {new:?}"); // => Some(["es", "fr", "pt", "de", "en"])

    assert_eq!(
      new,
      Some(
        ["es", "fr", "pt", "en", "de"]
          .map(Into::into)
          .as_ref()
      )
    )
  }
}
