use std::sync::OnceLock;

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
