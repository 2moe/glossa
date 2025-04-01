use lang_id::error::LangidError;
use thiserror::Error;

use crate::MiniStr;

/// A custom error type for Glossa
// #[derive(Error, Debug, PartialEq)]
#[derive(Debug, Error, PartialEq)]
pub enum GlossaError<'map> {
  /// (map_name, key)
  #[error("{msg} (map: '{0}', key: '{1}')", msg = text_not_found())]
  MapTextNotFound(&'map str, &'map str),
  /// (key)
  #[error("{msg} (key: '{0}')", msg = text_not_found())]
  TextNotFound(MiniStr),
  #[error("LangID Error: {0}")]
  LangIDError(#[from] LangidError),
}

impl Default for GlossaError<'_> {
  fn default() -> Self {
    LangidError::Unknown.into()
  }
}

impl<'map> GlossaError<'map> {
  /// Constructor function for TextNotFound error
  pub fn new_text_not_found<S: Into<MiniStr>>(v: S) -> Self {
    Self::TextNotFound(v.into())
  }

  /// Constructor function for MapTextNotFound error
  pub fn new_map_text_not_found(map: &'map str, key: &'map str) -> Self {
    Self::MapTextNotFound(map, key)
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
  match glossa_l10n::error::matches::map(language, b"error", b"text-not-found") {
    "" => None,
    s => Some(s),
  }
}

#[cfg(feature = "std")]
pub(crate) fn text_not_found<'a>() -> &'a str {
  use glossa_l10n::error::locale_registry;

  crate::lazy_values::get_or_init_str_language_chain(Some(
    &locale_registry::all_locales(),
  ))
  .iter()
  .map(|id| id.as_bytes())
  .find_map(get_error_text)
  .unwrap_or_default()
}

#[cfg(not(feature = "std"))]
pub(crate) fn text_not_found<'a>() -> &'a str {
  "No localized text found"
  // get_error_text(b"en").unwrap_or_default()
}

#[cfg(test)]
mod tests {
  use testutils::simple_benchmark;

  use super::*;

  #[ignore]
  #[test]
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
