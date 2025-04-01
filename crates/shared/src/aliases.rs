// #![cfg(feature = "type-aliases")]

use crate::{
  MiniStr as Key, MiniStr as Value, MiniStr as Language, MiniStr as MapName,
  TemplateResolver,
};

#[cfg(feature = "std")]
pub mod type_aliases {
  pub use ahash;
  use ahash::HashMap;
  pub use kstring::KString;

  use super::*;

  pub type L10nFlattenMap = HashMap<(MapName, Key), Value>;
  pub type L10nMaps = HashMap<Language, L10nFlattenMap>;

  pub type L10nTemplateMap = HashMap<MapName, TemplateResolver>;
  pub type TemplateMaps = HashMap<Language, L10nTemplateMap>;
}

#[cfg(not(feature = "std"))]
pub mod type_aliases {
  use alloc::collections::BTreeMap;

  use super::*;

  pub type L10nFlattenMap = BTreeMap<(MapName, Key), Value>;

  pub type L10nMaps = BTreeMap<Language, L10nFlattenMap>;

  pub type L10nTemplateMap = BTreeMap<MapName, TemplateResolver>;

  pub type TemplateMaps = BTreeMap<Language, L10nTemplateMap>;
}
