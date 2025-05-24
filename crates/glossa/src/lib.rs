#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

// /*!
// # glossa
// */
extern crate alloc;

pub use compact_str::CompactString as MiniStr;

mod cldr_fallback;
pub(crate) use cldr_fallback::cldr_fallback_mapping;

pub mod error;
pub use error::{GlossaError as Error, GlossaResult as Result};
//
/// Language Identifier
pub use lang_id::LangID;

/// Contains the FallbackChain Trait implementation
pub mod fallback;
pub use fallback::{try_init_chain, try_init_chain_from_slice};

#[cfg(feature = "std")]
pub mod sys; // old_name: lazy_values

pub mod traits;

mod context;
#[cfg(feature = "std")]
pub use context::LocaleContext;

pub mod misc {
  use lang_id::LangID;

  /// `language.split(['.', '@', ':']).next().into().parse()`
  pub fn normalize_glossa_lang(language: Option<&str>) -> Option<LangID> {
    language.and_then(|v| {
      v.split(['.', '@', ':'])
        .next()
        .and_then(|x| x.parse().ok())
    })
  }
}
