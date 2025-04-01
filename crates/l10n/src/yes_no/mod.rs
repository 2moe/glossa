#[cfg(feature = "yes_no")]
pub mod locale_registry;

#[cfg(feature = "yes_no")]
pub mod matches;

pub const fn default(key: &[u8]) -> &'static str {
  match key {
    b"cancel" => r#####"Cancel"#####,
    b"no" => r#####"No"#####,
    b"ok" => r#####"OK"#####,
    b"yes" => r#####"Yes"#####,
    _ => "",
  }
}
