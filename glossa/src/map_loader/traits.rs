use crate::{
    // assets::OnceCell, // OnceCell module for lazy initialization
    fallback::{Chain, FallbackChain}, // FallbackChain trait for handling fallback chains
    log::{debug, trace},              // Language ID type
    map_loader::MapLoader, // MapLoader struct for loading Language Resource map
    LangID,
};
use std::{
    hash::BuildHasher,
    ops::{Deref, DerefMut},
};

/// Implementation of deref_mut method for mutable reference
/// Gets &mut MapLoader.id
impl<S: BuildHasher> DerefMut for MapLoader<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id // Returns mutable reference to the field 'id'
    }
}

/// Implementation of deref method for immutable reference
/// Gets &MapLoader.id
impl<S: BuildHasher> Deref for MapLoader<S> {
    type Target = LangID;

    fn deref(&self) -> &Self::Target {
        &self.id // Returns immutable reference to the field 'id'
    }
}

impl<S: BuildHasher> FallbackChain for MapLoader<S> {
    /// Implementation of get_id() method from FallbackChain trait
    /// There is nothing special about the getter of this Trait, it is the same as self.get_id().
    fn get_id(&self) -> &LangID {
        &self.id // Returns the current locale id
    }

    fn set_chain_once(&self, custom: Option<Chain>) -> &Chain {
        self.chain
            .get_or_init(|| self.new_chain(custom))
    }

    /// Generates the initial fallback chain by filtering locales with the same language code but different variants.
    fn get_locale_list(&self) -> Chain {
        // Create a list of locales with the same language and exclude the current locale
        trace!("About to get the locale list");
        debug!("{}", self.get_id());

        self.map
            .keys()
            .filter(|&x| self.locale_list_filter(x))
            .cloned()
            .collect()
    }
}
