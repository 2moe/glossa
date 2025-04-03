#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

/*!
# glossa

*/
extern crate alloc;

pub use compact_str::CompactString as MiniStr;

mod cldr_fallback;
pub(crate) use cldr_fallback::cldr_fallback_mapping;

pub mod error;
pub use error::GlossaResult as Result;
//
/// Language Identifier
pub use lang_id::LangID;

/// Contains the FallbackChain Trait implementation
pub mod fallback;
pub use fallback::{try_init_chain, try_init_chain_from_slice};

#[cfg(feature = "std")]
pub mod lazy_values;
