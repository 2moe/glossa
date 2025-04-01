#[cfg(feature = "error")]
pub mod locale_registry;

#[cfg(feature = "error")]
pub mod matches;

pub const fn default() -> &'static str {
  r#####"No localized text found"#####
}
