use lang_id::{LangID, sys_locale};
use tap::{Pipe, Tap};
use testutils::new_once_lock;

use crate::{
  MiniStr,
  fallback::{append_en, conv_language_chain_to_str_chain},
  init_language_chain,
};

/// Gets the static value of system language.
pub fn get_static_lang() -> &'static LangID {
  testutils::new_once_lock!(LANG: LangID);

  LANG.get_or_init(|| match () {
    #[cfg(not(target_os = "macos"))]
    () => sys_locale::fetch_sys_or_env_lang(),
    #[cfg(target_os = "macos")]
    () => sys_locale::fetch_env_lang_or_sys_locale(),
  })
}

pub fn get_or_init_sys_language_chain(
  all_locales: Option<&[LangID]>,
) -> &'static [LangID] {
  new_once_lock!(CHAIN: Box<[LangID]>);

  CHAIN.get_or_init(|| {
    init_language_chain(get_static_lang(), all_locales.unwrap_or_default())
      .expect("Failed to init language chain")
      .tap_mut(|x| {
        append_en(x);
      })
      .into_boxed_slice()
  })
}

pub fn get_or_init_str_language_chain(
  all_locales: Option<&[LangID]>,
) -> &'static [MiniStr] {
  new_once_lock!(CHAIN: Box<[MiniStr]>);

  CHAIN.get_or_init(|| {
    get_or_init_sys_language_chain(all_locales)
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
