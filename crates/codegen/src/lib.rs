pub mod generator;
use kstring::KString;

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
pub use visibility::Visibility;

fn to_kstr(s: impl AsRef<str>) -> KString {
  use glossa_shared::tap::Pipe;

  s.as_ref()
    .pipe(KString::from_ref)
}
