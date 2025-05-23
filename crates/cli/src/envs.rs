use glossa::{LangID, sys::new_once_lock};

fn get_env(s: &str) -> Option<String> {
  std::env::var(s)
    .ok()
    .filter(|x| !x.trim_ascii().is_empty())
}

/// Gets the environment variable for the Glossa configuration directory
/// (GLOSSA_CFG_DIR)
pub fn static_glossa_cfg_dir() -> Option<&'static str> {
  new_once_lock!(V: Option<String>);
  V.get_or_init(|| get_env("GLOSSA_CFG_DIR"))
    .as_deref()
}

/// GLOSSA_L10N_DIR
pub fn static_glossa_l10n_dir() -> Option<&'static str> {
  new_once_lock!(V: Option<String>);
  V.get_or_init(|| get_env("GLOSSA_L10N_DIR"))
    .as_deref()
}

/// `GLOSSA_LANG.split(['.', '@', ':']).next().into().parse()`
pub fn normalize_glossa_lang() -> Option<LangID> {
  static_glossa_lang().and_then(|v| {
    v.split(['.', '@', ':'])
      .next()
      .and_then(|x| x.parse().ok())
  })
}

/// GLOSSA_LANG
pub fn static_glossa_lang() -> Option<&'static str> {
  new_once_lock!(L: Option<String>);

  L.get_or_init(|| get_env("GLOSSA_LANG"))
    .as_deref()
}

pub(crate) fn format_glossa_env() -> &'static str {
  let (lang, l10n_dir, cfg_dir) = (
    static_glossa_lang(),
    static_glossa_l10n_dir(),
    static_glossa_cfg_dir(),
  );

  log::info!(
    "env:
  GLOSSA_LANG: {lang:?}
  GLOSSA_L10N_DIR: {l10n_dir:?}
  GLOSSA_CFG_DIR: {cfg_dir:?}
  GLOSSA_LOG: {log_lv:?}
  ",
    log_lv = get_env("GLOSSA_LOG")
  );
  ""
}
