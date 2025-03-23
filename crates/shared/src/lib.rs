#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
// -----

pub use compact_str::{CompactString as MiniStr, format_compact as fmt_compact};

mod phf_tuple_key;
pub use phf_tuple_key::PhfTupleKey;

// -----
mod aliases;
//
#[cfg(feature = "type-aliases")]
pub use aliases::type_aliases;
// -----
#[cfg(feature = "phf")]
pub use phf;
//
#[cfg(feature = "phf")]
pub type PhfL10nOrderedMap<'k> = phf::OrderedMap<PhfTupleKey<'k>, &'static str>;
// -----

#[cfg(feature = "bincode")]
pub mod decode;
