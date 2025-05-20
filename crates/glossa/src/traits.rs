use crate::MiniStr;

pub trait ChainProvider {
  fn provide_chain(&self) -> Option<&[MiniStr]>;
}

#[cfg(feature = "std")]
impl ChainProvider for crate::LocaleContext {
  fn provide_chain(&self) -> Option<&[MiniStr]> {
    self.get_or_try_init_chain()
  }
}
