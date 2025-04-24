use std::sync::OnceLock;

use getset::Getters;
use lang_id::{LangID, sys_locale};
use log::warn;
use tap::Pipe;
pub use testutils::new_once_lock;

use crate::{MiniStr, fallback::LocaleStrChain};

pub trait ChainProvider {
  fn provide_chain(&self) -> Option<&[MiniStr]>;
}

impl ChainProvider for LocaleContext {
  fn provide_chain(&self) -> Option<&[MiniStr]> {
    self.get_or_try_init_chain()
  }
}

/// A context holder for locale-related information and fallback chains.
/// Manages current locale, supported locales, and cached fallback chains.
#[derive(Default, Debug, Getters, Clone)]
#[getset(get = "pub with_prefix")]
pub struct LocaleContext {
  /// Current active locale (initialized lazily)
  current_locale: OnceLock<LangID>,

  /// Cached locale fallback chain (e.g., ["en-NZ", "en-GB" "en"])
  #[getset(skip)]
  chain: OnceLock<LocaleStrChain>,

  /// All available locales in the application
  all_locales: Option<Box<[LangID]>>,
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
  fn init_static_locale_if_uninitialized(&self) {
    if self.is_current_locale_initialized() {
      return;
    }

    self
      .current_locale
      .get_or_init(|| get_static_locale().clone());
  }

  /// Checks if current locale has been initialized
  fn is_current_locale_initialized(&self) -> bool {
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
  pub fn with_current_locale(mut self, current: Option<LangID>) -> Self {
    self.current_locale.take();
    self
      .current_locale
      .get_or_init(|| match current {
        Some(x) => x,
        _ => get_locale(),
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

/// Retrieves system locale with platform-specific implementations
fn get_locale() -> LangID {
  match () {
    #[cfg(not(target_os = "macos"))]
    () => sys_locale::fetch_sys_or_env_lang(),
    #[cfg(target_os = "macos")]
    () => sys_locale::fetch_env_lang_or_sys_locale(),
  }
}

/// Retrieves the system's primary locale(language identifier) with thread-safe
/// initialization
///
/// Implements platform-specific detection strategies:
/// - macOS: Prioritizes environment variables before system settings
/// - Other OS: Uses system locale APIs with environment fallback
pub fn get_static_locale() -> &'static LangID {
  testutils::new_once_lock!(LANG: LangID);
  LANG.get_or_init(get_locale)
}

#[cfg(test)]
mod tests {
  use testutils::dbg_ref;

  use super::*;
  use crate::fallback::dbg_shared::init_logger;

  #[ignore]
  #[test]
  fn test_get_or_init_posix_language_chain() {
    init_logger(true);
    // unsafe {
    //   std::env::set_var("LANG", "POSIX.UTF-8");
    // };
    dbg_ref!(get_static_locale());

    let all_locales = {
      use lang_id::consts::*;
      [
        lang_id_en(),
        lang_id_zh(),
        lang_id_ar(),
        lang_id_de(),
        lang_id_ru(),
        lang_id_es_419(),
      ]
    };

    let ctx = LocaleContext::default().with_all_locales(all_locales);

    let chain = ctx.get_or_try_init_chain();

    dbg!(chain);
  }
}
