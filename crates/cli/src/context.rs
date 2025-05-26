use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use glossa::{
  LangID, LocaleContext, fallback::dbg_ref, misc::normalize_glossa_lang,
  sys::new_once_lock, traits::ChainProvider,
};
use glossa_codegen::glossa_shared::{
  ToCompactString,
  load_bincode::{self, list_bincode_files, try_load_files},
  tap::{Pipe, Tap},
  type_aliases::L10nMaps,
};

use crate::{envs::static_glossa_lang, static_data::bincode_dir};

pub(crate) trait GetL10nText: ChainProvider {
  fn try_get_bincode<'t>(&self, map_name: &str, key: &str) -> Option<&'t str> {
    let key_bytes = key.as_bytes();
    let fallback_to_builtin =
      || static_locale_context().try_get_bulitin_cli_data(key_bytes);

    let lookup =
      |(language, map_name, key)| match static_bincode_resource()?.get(language) {
        Some(x) => x
          .get(&(map_name, key))
          .map(|x| x.as_str())
          .or_else(fallback_to_builtin),
        _ => {
          log::trace!("fallback to builtin data");
          fallback_to_builtin()
        }
      };

    self
      .provide_chain()?
      .iter()
      .map(|lang| (lang, map_name.to_compact_string(), key.to_compact_string()))
      .find_map(lookup)
  }

  fn try_get_bulitin_cli_data<'t>(&self, key: &[u8]) -> Option<&'t str> {
    let lookup = |language| match crate::l10n::router::map(language, key) {
      "" => None,
      s => Some(s),
    };
    self
      .provide_chain()?
      .iter()
      .map(|lang| lang.as_bytes())
      .find_map(lookup)
  }

  fn try_get<'t>(&self, map_name: &str, key: &str) -> Option<&'t str> {
    match static_bincode_resource() {
      Some(_) => self.try_get_bincode(map_name, key),
      _ => self.try_get_bulitin_cli_data(key.as_bytes()),
    }
  }
}

impl GetL10nText for LocaleContext {}

/// - exists => Some(())
/// - _ => None
pub fn bincode_exists() -> Option<()> {
  bincode_dir()
    .pipe(list_static_bincode_files)
    .map(|_| ())
}

fn list_static_bincode_files(
  bincode_dir: Option<&Path>,
) -> Option<&'static [PathBuf]> {
  new_once_lock!(V: Option<Vec<PathBuf>>);
  V.get_or_init(|| list_bincode_files(bincode_dir))
    .as_deref()
}

pub fn static_bincode_resource() -> Option<&'static Arc<L10nMaps>> {
  bincode_exists()?;

  new_once_lock!(L: Option<Arc<L10nMaps>>);

  L.get_or_init(|| {
    bincode_dir()
      .pipe(list_static_bincode_files)
      .map(try_load_files)
      .and_then(|x| x.ok())
      .map(Arc::new)
  })
  .as_ref()
}

fn compatible_with_gnu_language_colon_style(locale_context: &mut LocaleContext) {
  let _ =
    locale_context.try_push_front_with_colon_separated_str(static_glossa_lang());
}

pub fn static_bincode_context() -> Option<&'static LocaleContext> {
  bincode_exists()?;
  new_once_lock!(L: Option<LocaleContext>);

  L.get_or_init(|| {
    let locales = merge_locales()?;
    dbg_ref!(locales);

    static_glossa_lang()
      .pipe(normalize_glossa_lang)
      .pipe(|x| LocaleContext::default().with_current_locale(x))
      .with_all_locales(locales)
      .tap_mut(compatible_with_gnu_language_colon_style)
      .pipe(Some)
  })
  .as_ref()
}

/// Merges locales from builtin & bincode sources
pub(crate) fn merge_locales() -> Option<Box<[LangID]>> {
  load_bincode::merge_locales(
    static_bincode_resource().map(|v| v.as_ref()),
    static_locale_context()
      .get_all_locales()
      .as_deref(),
  )
}

pub(crate) fn static_locale_context() -> &'static LocaleContext {
  new_once_lock!(L: LocaleContext);
  L.get_or_init(|| {
    let new_ctx_with_cur_locale =
      |x| LocaleContext::default().with_current_locale(x);

    static_glossa_lang()
      .pipe(normalize_glossa_lang)
      .pipe(new_ctx_with_cur_locale)
      .with_all_locales(crate::l10n::locale_registry::all_locales())
      .tap_mut(compatible_with_gnu_language_colon_style)
  })
}

pub fn get_text<'a>(key: &str, map_name: Option<&str>) -> &'a str {
  let map_name = map_name.unwrap_or("cli");

  match static_bincode_context() {
    Some(ctx) => ctx,
    _ => static_locale_context(),
  }
  .try_get(map_name, key)
  .unwrap_or_else(|| {
    log::warn!("{}", glossa::Error::new_map_text_not_found(map_name, key));
    ""
  })
}
