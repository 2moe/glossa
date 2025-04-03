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
/// # Fallback Strategy
/// 1. Attempt chain creation with detected locales
/// 2. Append English (en) as final fallback
/// 3. Return default EN-only chain on failure
pub fn init_sys_language_chain(all_locales: Option<&[LangID]>) -> Box<[LangID]> {
  let en_slice = || [lang_id::common::lang_id_en()].into();

  let Some(locales) = all_locales else {
    return en_slice();
  };

  match try_init_chain(get_static_lang(), locales) {
    Ok(v) => v
      .tap_mut(|x| {
        append_en(x);
      })
      .into_boxed_slice(),
    Err(_) => en_slice(),
  }
}

pub(crate) fn get_or_init_sys_language_chain(
  all_locales: Option<&[LangID]>,
) -> &'static [LangID] {
  new_once_lock!(CHAIN: Box<[LangID]>);

  CHAIN.get_or_init(|| init_sys_language_chain(all_locales))
}

pub(crate) fn get_or_init_str_language_chain(
  all_locales: Option<&[LangID]>,
) -> &'static [MiniStr] {
  new_once_lock!(CHAIN: Box<[MiniStr]>);

  CHAIN.get_or_init(|| {
    all_locales
      .pipe(get_or_init_sys_language_chain)
      .pipe(conv_language_chain_to_str_chain)
  })
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
    unsafe {
      std::env::set_var("LANG", "POSIX.UTF-8");
    };
    dbg_ref!(get_static_lang());
    // dbg_ref!("".parse::<LangID>());

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

    let chain = get_or_init_sys_language_chain(Some(&all_locales));

    dbg!(chain);
  }
}
