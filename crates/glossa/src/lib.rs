#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

/*!
# glossa

*/

pub mod error;

/// The default error type is `GlossaError<'map>`
pub type Result<'map, T> = ::core::result::Result<T, error::GlossaError<'map>>;

/// Language Identifier
pub use lang_id::LangID;

/// Contains the FallbackChain Trait implementation
pub mod fallback;

/// Gets the static value of system language.
/// If it has not been initialised yet, initialise it by calling the
/// `lang_id::sys_lang::current()`.
pub fn get_static_lang() -> &'static LangID {
  use std::sync::OnceLock;

  static LANG: OnceLock<LangID> = OnceLock::new();
  LANG.get_or_init(lang_id::sys_locale::fetch_env_lang_or_sys_locale)
}
