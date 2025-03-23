#![cfg(feature = "type-aliases")]

pub use lang_id::LangID;
#[allow(unused_imports)]
pub use tmpl_resolver::resolver::{TemplateAST, TemplateResolver};

use crate::MiniStr;

#[cfg(feature = "std")]
pub mod type_aliases {
  use ahash::HashMap;
  use kstring::KString;

  use super::*;

  pub type L10nFlattenMap = HashMap<(KString, KString), MiniStr>;

  // // [(lang, <(map_name, key), value>)]
  pub type L10nMaps = Box<[(LangID, L10nFlattenMap)]>;

  pub type L10nTemplateMap = HashMap<KString, TemplateAST>;

  // // [(lang, <map_name, map>)]
  pub type TemplateMaps = Box<[(LangID, L10nTemplateMap)]>;
}

#[cfg(not(feature = "std"))]
pub mod type_aliases {
  use alloc::{boxed::Box, collections::BTreeMap};

  use super::*;

  pub type L10nFlattenMap = BTreeMap<(MiniStr, MiniStr), MiniStr>;

  pub type L10nMaps = Box<[(LangID, L10nFlattenMap)]>;

  pub type L10nTemplateMap = BTreeMap<MiniStr, TemplateAST>;

  pub type TemplateMaps = Box<[(LangID, L10nTemplateMap)]>;
}
