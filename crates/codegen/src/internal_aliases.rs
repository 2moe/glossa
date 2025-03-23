/// `Box<[(Hightlight resources, Highlight Format)]>`
#[cfg(feature = "highlight")]
pub(crate) use crate::highlight::HighlightCfgMap;

#[cfg(not(feature = "highlight"))]
pub(crate) type HighlightCfgMap<'h> =
  [(bool, core::marker::PhantomData<&'h bool>); 0];
