use core::fmt;

use log::warn;
use thiserror::Error;

/// A custom error type for Glossa
#[derive(Error, Debug)]
pub enum GlossaError<'map> {
  /// (map_name, key)
  MapTextNotFound(&'map str, &'map str), // Error when MapLoader text is not found
  /// (key)
  TextNotFound(String), // Error when text is not found
}

impl fmt::Display for GlossaError<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use GlossaError::*;

    match self {
      TextNotFound(key) => {
        warn!(r#"(key: "{key}")"#);
        write!(f, "(key: `{key}`)")
      }
      MapTextNotFound(map, key) => {
        warn!(r#"(map: "{map}", key: "{key}")"#);
        write!(f, "(map: `{map}`, key: `{key}`)")
      }
    }
  }
}

impl<'map> GlossaError<'map> {
  /// Constructor function for TextNotFound error
  pub fn text_not_found<S: Into<String>>(v: S) -> Self {
    Self::TextNotFound(v.into())
  }

  /// Constructor function for MapTextNotFound error
  pub fn map_text_not_found(map: &'map str, key: &'map str) -> Self {
    Self::MapTextNotFound(map, key)
  }
}
