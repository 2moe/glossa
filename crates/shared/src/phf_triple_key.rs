use core::hash::{Hash, Hasher};

use phf_shared::{FmtConst, PhfBorrow, PhfHash};

use crate::MiniStr;

#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct PhfTripleKey<'a>(pub &'a str, pub &'a str, pub &'a str);

impl PhfHash for PhfTripleKey<'_> {
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
    self.1.hash(state);
    self.2.hash(state);
  }
}

impl PhfBorrow<Self> for PhfTripleKey<'_> {
  fn borrow(&self) -> &Self {
    self
  }
}

impl FmtConst for PhfTripleKey<'_> {
  fn fmt_const(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      r#####"Key(r#"{}"#, r##"{}"##, r###"{}"###)"#####,
      self.0, self.1, self.2
    )
  }
}

// --------

#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct RawTripleKey<'a>(pub MiniStr, pub &'a str, pub &'a str);
impl PhfHash for RawTripleKey<'_> {
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    self.0.as_str().hash(state);
    self.1.hash(state);
    self.2.hash(state);
  }
}
impl PhfBorrow<Self> for RawTripleKey<'_> {
  fn borrow(&self) -> &Self {
    self
  }
}
impl FmtConst for RawTripleKey<'_> {
  fn fmt_const(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      r#####"Key(r#"{}"#, r##"{}"##, r###"{}"###)"#####,
      self.0, self.1, self.2
    )
  }
}
