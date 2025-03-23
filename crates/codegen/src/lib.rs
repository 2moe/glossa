pub mod generator;
use kstring::KString;
// mod tmp__test_en;
// pub(crate) use glossa_shared::PhfTupleKey;
// pub use phf;
// pub(crate) type PhfL10nOrderedMap<'k> =
//   phf::OrderedMap<PhfTupleKey<'k>, &'static str>;

#[cfg(feature = "highlight")]
pub mod highlight;

pub(crate) mod internal_aliases;

//
mod resources;
mod visibility;

pub use anyhow::Result as AnyResult;
//
pub use generator::Generator;
pub use glossa_shared::MiniStr;
pub use resources::L10nResources;
use tap::Pipe;
pub use visibility::Visibility;

fn to_kstr(s: impl AsRef<str>) -> KString {
  s.as_ref()
    .pipe(KString::from_ref)
}
