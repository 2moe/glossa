use lang_id::{LangID, sys_locale};
pub use testutils::new_once_lock;

/// Retrieves system locale with platform-specific implementations.
///
/// If you don't need to modify the system's language during program execution,
/// you can use [get_static_locale()]. Since the [get_static_locale()] supports
/// locale caching, it's more efficient than [retrieve_locale()].
///
/// ## Example
///
/// ```
/// use glossa::sys::retrieve_locale;
///
/// let current_locale = retrieve_locale();
/// // assert_eq!(current_locale.to_string(), "en-US");
/// ```
pub fn retrieve_locale() -> LangID {
  match () {
    #[cfg(not(target_os = "macos"))]
    () => sys_locale::retrieve_sys_or_env_lang(),
    #[cfg(target_os = "macos")]
    () => sys_locale::retrieve_env_lang_or_sys_locale(),
  }
}

/// Gets the system's primary locale(language identifier) with thread-safe
/// initialization.
///
/// > It supports caching the locale.
/// >
/// > In essence, [get_static_locale()] persists the result of
/// > [retrieve_locale()]
/// > as a static variable.
///
/// Implements platform-specific detection strategies:
/// - macOS: Prioritizes environment variables before system settings
/// - Other OS: Uses system locale APIs with environment fallback
pub fn get_static_locale() -> &'static LangID {
  new_once_lock!(LANG: LangID);
  LANG.get_or_init(retrieve_locale)
}

#[cfg(test)]
mod tests {
  use testutils::dbg_ref;

  use super::*;
  use crate::{LocaleContext, fallback::dbg_shared::init_logger};

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
