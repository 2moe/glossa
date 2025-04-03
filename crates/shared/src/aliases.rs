// #![cfg(feature = "type-aliases")]
use glossa_dsl::resolver::Resolver;

use crate::{
  MiniStr as Key, MiniStr as Value, MiniStr as Language, MiniStr as MapName,
};

#[cfg(feature = "std")]
pub mod type_aliases {
  pub use ahash;
  use ahash::HashMap;
  pub use kstring::KString;

  use super::*;

  pub type L10nFlattenMap = HashMap<(MapName, Key), Value>;
  pub type L10nMaps = HashMap<Language, L10nFlattenMap>;

  pub type L10nDSLMap = HashMap<MapName, Resolver>;
  pub type DSLMaps = HashMap<Language, L10nDSLMap>;
}

#[cfg(not(feature = "std"))]
pub mod type_aliases {
  use alloc::collections::BTreeMap;

  use super::*;

  pub type L10nFlattenMap = BTreeMap<(MapName, Key), Value>;

  pub type L10nMaps = BTreeMap<Language, L10nFlattenMap>;

  pub type L10nDSLMap = BTreeMap<MapName, Resolver>;

  pub type DSLMaps = BTreeMap<Language, L10nDSLMap>;
}
