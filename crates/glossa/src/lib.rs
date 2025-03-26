#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

/*!
# glossa

*/

pub use compact_str::CompactString as MiniStr;

mod cldr_fallback;
pub(crate) use cldr_fallback::cldr_fallback_mapping;

pub mod error;

/// The default error type is `GlossaError<'map>`
pub type Result<'map, T> = ::core::result::Result<T, error::GlossaError<'map>>;

/// Language Identifier
pub use lang_id::LangID;

/// Contains the FallbackChain Trait implementation
pub mod fallback;
pub use fallback::{init_language_chain, init_language_chain_from_slice};

#[cfg(feature = "std")]
pub mod lazy_values;
