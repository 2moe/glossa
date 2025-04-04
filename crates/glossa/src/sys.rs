use lang_id::{LangID, sys_locale};
use tap::{Pipe, Tap};
use testutils::new_once_lock;

use crate::{
  MiniStr,
  fallback::{append_en, conv_language_chain_to_str_chain},
  try_init_chain,
};

/// Retrieves the system's primary language identifier with thread-safe
/// initialization
///
/// Implements platform-specific detection strategies:
/// - macOS: Prioritizes environment variables before system settings
/// - Other OS: Uses system locale APIs with environment fallback
pub fn get_static_lang() -> &'static LangID {
  testutils::new_once_lock!(LANG: LangID);

  LANG.get_or_init(|| match () {
    #[cfg(not(target_os = "macos"))]
    () => sys_locale::fetch_sys_or_env_lang(),
    #[cfg(target_os = "macos")]
    () => sys_locale::fetch_env_lang_or_sys_locale(),
  })
}

/// Builds a prioritized language chain with resilient fallback handling
///
/// # Stages:
///
/// 1. current = [get_static_lang()]
/// 2. Language chain [initialization](crate::fallback::try_init_chain)
/// 3. [crate::fallback::append_en()]
/// 4. Return EN-only chain on failure
pub fn init_sys_language_chain(all_locales: Option<&[LangID]>) -> Box<[LangID]> {
  let try_init = |locales| try_init_chain(get_static_lang(), locales).ok();

  match all_locales.and_then(try_init) {
    Some(v) => v
      .tap_mut(|x| {
        append_en(x);
      })
      .into_boxed_slice(),
    _ => [lang_id::common::lang_id_en()].into(),
  }
}

/// Constructs an optimized string-based language priority chain
///
/// Transforms language identifiers into space-efficient string representations
/// while preserving ordering.
///
/// # Stages:
///
/// 1. current = [get_static_lang()]
/// 2. Language chain initialization:
///   - [try_init_chain(current, _)](crate::fallback::try_init_chain)
/// 3. [crate::fallback::append_en()]
/// 4. Compact string [conversion](conv_language_chain_to_str_chain)
pub fn init_str_chain(all_locales: Option<&[LangID]>) -> Box<[MiniStr]> {
  all_locales
    .pipe(init_sys_language_chain)
    .as_ref()
    .pipe(conv_language_chain_to_str_chain)
}

pub(crate) fn get_or_init_str_language_chain(
  all_locales: Option<&[LangID]>,
) -> &'static [MiniStr] {
  new_once_lock!(CHAIN: Box<[MiniStr]>);

  CHAIN.get_or_init(|| all_locales.pipe(init_str_chain))
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
    dbg_ref!(get_static_lang());

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

    let chain = init_str_chain(Some(&all_locales));

    dbg!(chain);
  }
}
