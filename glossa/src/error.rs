use crate::{
    assets::OnceCell,
    l10n::locales,
    log::{err, trace, warning},
    map_loader::get::GetText,
};
use std::io;
pub use thiserror::Error;

/// A custom error type for Glossa
#[derive(Error, Debug)]
pub enum GlossaError<'map> {
    MapTextNotFound(&'map str, &'map str), // Error when MapLoader text is not found
    TextNotFound(String),                  // Error when text is not found
    CreateArcLoader(#[source] io::Error),  // Error when creating arc loader fails
}

impl std::fmt::Display for GlossaError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GlossaError::*;
        let msg = static_text_not_found_msg;

        match self {
            TextNotFound(key) => {
                warning!("{} (key: `{key}`)", msg());
                write!(f, r#"{} (key: "{key}")"#, msg())
            }
            CreateArcLoader(e) => {
                let err_msg = "Failed to create arc loader.";
                err!("{err_msg}\nError: {e}");
                write!(f, "{err_msg}\n{e}")
            }
            MapTextNotFound(map, key) => {
                warning!("{} (map: `{map}`, key: `{key}`)", msg());
                write!(f, r#"{} (map: "{map}", key: "{key}")"#, msg())
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

/// Helper function to retrieve localised error message
fn static_text_not_found_msg() -> &'static str {
    static MSG: OnceCell<&str> = OnceCell::new();
    trace!("Retrieving static localised error message...");
    MSG.get_or_init(|| {
        locales()
            .get("error", "text-not-found")
            .unwrap_or("No localised text found.")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fallback::FallbackChain;

    #[test]
    fn text_not_found() {
        let err = GlossaError::text_not_found("hello");
        println!("{err}");
    }

    #[test]
    fn get_or_default_msg() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        locales().show_chain();
        let err = locales().get_or_default("error", "test-err-msg");
        eprintln!("{err}");
        let err = locales().get_or_default("errors", "test-err2");
        eprintln!("{err}");
    }

    #[test]
    fn any_error() -> crate::Result<'static, ()> {
        use crate::log::err;
        std::env::set_var("RUST_LOG", "trace");

        env_logger::init();
        err!("hello");
        let err = locales().get("err", "test-err-msg")?;
        eprintln!("{err}");
        Ok(())
    }
}
