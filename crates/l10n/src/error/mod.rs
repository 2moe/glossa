pub const fn default() -> &'static str {
  r#####"No localized text found"#####
}

#[cfg(feature = "error")]
/// key: text-not-found
pub mod matches;

#[cfg(feature = "error")]
#[cfg(feature = "lang-id")]
pub mod locale_registry;

#[cfg(feature = "lang-id")]
use lang_id;
