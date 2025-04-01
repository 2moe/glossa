use core::hash::{Hash, Hasher};

use phf_shared::{FmtConst, PhfBorrow, PhfHash};

#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct PhfTupleKey<'a>(pub &'a str, pub &'a str);

impl PhfHash for PhfTupleKey<'_> {
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
    self.1.hash(state);
  }
}

// impl<'a> PhfBorrow<PhfMapKey<'a>> for &'a PhfMapKey<'a> {
//   fn borrow(&self) -> &PhfMapKey<'a> {
//     self
//   }
// }
impl PhfBorrow<Self> for PhfTupleKey<'_> {
  fn borrow(&self) -> &Self {
    self
  }
}

impl FmtConst for PhfTupleKey<'_> {
  fn fmt_const(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let Self(map, key) = self;
    write!(f, r###"Key(r#"{map}"#, r##"{key}"##)"###)
  }
}
