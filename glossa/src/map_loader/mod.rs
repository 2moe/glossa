use crate::{
    assets::{localisation::SubLocaleMap, OnceCell},
    fallback::Chain,
    LangID,
};
use getset::{Getters, MutGetters};
use std::{collections::HashMap, hash::BuildHasher};
type Map<S> = HashMap<LangID, SubLocaleMap, S>;

mod from;
pub mod get;
mod traits;

#[derive(Getters, MutGetters, Debug, Default)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct MapLoader<
    #[cfg(not(feature = "ahash"))] S = std::collections::hash_map::RandomState,
    #[cfg(feature = "ahash")] S = ahash::RandomState,
> where
    S: BuildHasher,
{
    id: LangID,
    map: Map<S>,
    chain: OnceCell<Chain>,
}
