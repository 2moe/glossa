pub const fn default(key: &[u8]) -> &'static str {
  match key {
    b"cancel" => r#####"Cancel"#####,
    b"no" => r#####"No"#####,
    b"ok" => r#####"OK"#####,
    b"yes" => r#####"Yes"#####,
    _ => "",
  }
}

#[cfg(feature = "yes_no")]
/// map_name: yes-no
pub mod matches;

#[cfg(feature = "yes_no")]
#[cfg(feature = "lang-id")]
pub mod locale_registry;

#[allow(unused_imports)]
#[cfg(feature = "lang-id")]
use lang_id;
