use getset::{Getters, MutGetters};
use std::{borrow::Cow, path::PathBuf};

#[cfg(not(feature = "highlight"))]
use std::marker::PhantomData;

mod from;

#[derive(Getters, MutGetters, Debug, Default)]
#[getset(
    get = "pub with_prefix",
    get_mut = "pub with_prefix",
    // set = "pub with_prefix"
)]
pub struct Generator<'ver, 'p, 'res> {
    l10n_path: PathBuf,
    version: Cow<'ver, str>,
    #[cfg(feature = "highlight")]
    highlight: Option<crate::highlight::HighLight<'res, 'p>>,
    #[cfg(not(feature = "highlight"))]
    highlight: Option<PhantomData<(&'res String, &'p PathBuf)>>,
}
