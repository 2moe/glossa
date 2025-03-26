#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
// -----

pub use compact_str::{
  CompactString as MiniStr, ToCompactString, format_compact as fmt_compact,
};

mod phf_tuple_key;
pub use phf_tuple_key::PhfTupleKey;
pub mod phf_triple_key;
pub use phf_triple_key::PhfTripleKey;

// -----
mod aliases;

#[cfg(feature = "type-aliases")]
pub use aliases::type_aliases;
#[cfg(feature = "type-aliases")]
pub use lang_id::LangID;
// -----
#[cfg(feature = "phf")]
pub use phf;
// -----
#[cfg(feature = "type-aliases")]
pub use tmpl_resolver::resolver::{TemplateAST, TemplateResolver};

// -----

#[cfg(feature = "phf")]
pub type PhfL10nOrderedMap = phf::OrderedMap<PhfTupleKey<'static>, &'static str>;

#[cfg(feature = "phf")]
pub type PhfL10nAllInOneMap = phf::OrderedMap<PhfTripleKey<'static>, &'static str>;
// -----

#[cfg(feature = "bincode")]
pub mod decode;
