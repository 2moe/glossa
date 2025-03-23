#[derive(Debug, Clone)]
pub enum Visibility {
  Private,
  PubCrate,
  Pub,
  PubSuper,
  // Custom(MiniStr),
}

impl From<&str> for Visibility {
  fn from(value: &str) -> Self {
    use Visibility::*;
    match value {
      "" => Private,
      "pub" => Pub,
      "pub(super)" => PubSuper,
      "pub(crate)" => PubCrate,
      _ => PubCrate,
    }
  }
}

impl Visibility {
  pub const fn as_str(&self) -> &str {
    use Visibility::*;
    match self {
      Private => "",
      PubCrate => "pub(crate)",
      Pub => "pub",
      PubSuper => "pub(super)",
    }
  }
}

impl AsRef<str> for Visibility {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Default for Visibility {
  fn default() -> Self {
    Self::PubCrate
  }
}
