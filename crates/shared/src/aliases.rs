// #![cfg(feature = "type-aliases")]

use crate::{LangID, MiniStr, TemplateResolver};

#[cfg(feature = "std")]
pub mod type_aliases {

  use ahash::HashMap;
  use kstring::KString;

  use super::*;

  pub type L10nFlattenMap = HashMap<(KString, KString), MiniStr>;

  // // [(lang, <(map_name, key), value>)]
  pub type L10nMaps = HashMap<LangID, L10nFlattenMap>;

  pub type L10nTemplateMap = HashMap<KString, TemplateResolver>;

  // // [(lang, <map_name, map>)]
  pub type TemplateMaps = HashMap<LangID, L10nTemplateMap>;
}

#[cfg(not(feature = "std"))]
pub mod type_aliases {
  use alloc::collections::BTreeMap;

  use super::*;

  pub type L10nFlattenMap = BTreeMap<(MiniStr, MiniStr), MiniStr>;

  pub type L10nMaps = BTreeMap<LangID, L10nFlattenMap>;

  pub type L10nTemplateMap = BTreeMap<MiniStr, TemplateResolver>;

  pub type TemplateMaps = BTreeMap<LangID, L10nTemplateMap>;
}
