pub(crate) use bincode::config::standard as bincode_std_cfg;
pub use glossa_dsl::error::ResolverResult;

use crate::type_aliases::{DSLMaps, L10nDSLMap, L10nFlattenMap, L10nMaps};

#[cfg(feature = "std")]
pub mod file;

// ----------------
pub mod slice;
