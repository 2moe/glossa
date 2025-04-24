use lang_id::error::LangidError;
use thiserror::Error;

use crate::{MiniStr as Key, MiniStr as MapName};

/// `Ok(T)` | `Err(GlossaError)`
pub type GlossaResult<T> = ::core::result::Result<T, GlossaError>;

#[derive(Debug, Error, PartialEq)]
pub enum GlossaError {
  #[error("{msg} (map: '{0}', key: '{1}')", msg = text_not_found())]
  MapTextNotFound(MapName, Key),
  #[error("{msg} (key: '{0}')", msg = text_not_found())]
  TextNotFound(Key),

  #[error("LangID Error: {0}")]
  LangIDError(#[from] LangidError),

  #[error("Failed to parse TinyStr: {0}")]
  ParseTinyStr(#[from] lang_id::error::ParseStrError),
}

impl Default for GlossaError {
  fn default() -> Self {
    LangidError::Unknown.into()
  }
}

impl GlossaError {
  /// Constructor function for TextNotFound error
  pub fn new_text_not_found<K: Into<Key>>(key: K) -> Self {
    Self::TextNotFound(key.into())
  }

  /// Constructor function for MapTextNotFound error
  pub fn new_map_text_not_found<M: Into<MapName>, K: Into<Key>>(
    map: M,
    key: K,
  ) -> Self {
    Self::MapTextNotFound(map.into(), key.into())
  }

  /// Returns `true` if the glossa error is [`LangIDError`].
  ///
  /// [`LangIDError`]: GlossaError::LangIDError
  #[must_use]
  pub fn is_langid_error(&self) -> bool {
    matches!(self, Self::LangIDError(..))
  }
}

#[cfg(feature = "std")]
pub(crate) fn get_error_text<'a>(language: &[u8]) -> Option<&'a str> {
  match glossa_l10n::error::matches::map(language) {
    "" => None,
    s => Some(s),
  }
}

#[cfg(feature = "std")]
pub(crate) fn text_not_found<'a>() -> &'a str {
  use glossa_l10n::error::default as default_text;

  use crate::sys::LocaleContext;
  let all_locales = glossa_l10n::error::locale_registry::all_locales();

  match LocaleContext::default()
    .with_all_locales(all_locales)
    .get_or_try_init_chain()
  {
    Some(x) => x
      .iter()
      .map(|id| id.as_bytes())
      .find_map(get_error_text)
      .unwrap_or_else(default_text),
    _ => default_text(),
  }
}

#[cfg(not(feature = "std"))]
pub(crate) fn text_not_found<'a>() -> &'a str {
  glossa_l10n::error::default()
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
  use testutils::simple_benchmark;

  use super::*;

  #[ignore]
  #[test]
  /// - release: 250ns
  /// - debug: 2.833µs
  fn bench_get_err_text() {
    let _ = text_not_found();
    simple_benchmark(|| {
      let _text = text_not_found();
    });
  }

  #[ignore]
  #[test]
  fn test_err() {
    let err = GlossaError::new_text_not_found("greeting");
    println!("{err}");

    let err2 = GlossaError::new_map_text_not_found("abc.tmpl", "hello");
    println!("{err2}")
  }
}
