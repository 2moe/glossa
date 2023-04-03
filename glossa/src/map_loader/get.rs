use crate::{
    assets::localisation::SubLocaleMap,
    error::GlossaError,
    fallback::FallbackChain,
    log::{debug, trace},
    map_loader::MapLoader,
    LangID,
};
use std::{borrow::Cow, hash::BuildHasher};

pub trait GetText<S: BuildHasher>: FallbackChain {
    fn get_map(&self) -> &super::Map<S>;

    /// Gets a localised string for a given map name and key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use crate::assets::localisation::locale_hashmap;
    /// use glossa::{MapLoader, GetText};
    ///
    /// let loader = MapLoader::new(locale_hashmap());
    /// let err_msg = loader.get("test", "greetings");
    /// ```
    fn get<'map>(
        &'map self,
        map_name: &'map str,
        key: &'map str,
    ) -> crate::Result<&str> {
        // Define a fallback closure that will be used if the lookup fails in the primary map.
        let fb = || self.get_from_fallback_chain(map_name, key);

        trace!("Try to get the sub-map for the current language from the main map.");
        match self
            .get_map()
            .get(self.get_id())
        {
            // If the sub-map exists, try to get the localised string from it.
            Some(sub) => {
                trace!(
                    "Found localised string for ('map_name: {}, key: {}')",
                    map_name,
                    key
                );
                Self::get_from_sub_map(sub, map_name, key).map_or_else(fb, Ok)
            }
            _ => {
                trace!("Fall back to the fallback chain.");
                fb()
            }
        }
    }

    /// Much like `get()`, but returns the error message as a default value when it fails.
    /// # Example
    ///
    /// ```no_run
    /// use glossa::{MapLoader, GetText};
    ///
    /// let loader = MapLoader::new(locale_hashmap());
    /// let msg = loader.get_or_default("error", "text-not-found");
    ///
    /// dbg!(msg);
    /// ```
    fn get_or_default<'map>(
        &'map self,
        map_name: &'map str,
        key: &'map str,
    ) -> Cow<str> {
        self.get(map_name, key)
            .map(Cow::from)
            // If the lookup fails, return the error message as a default value.
            .unwrap_or_else(|e| {
                debug!(
                    "Failed to find localised string for ('map: {map_name}, key: {key}')\n Err: {e}",
                );
                Cow::from(e.to_string())
            })
    }

    /// Try to get a localised string from the fallback chain for a given map-name and key.
    fn get_from_fallback_chain<'map>(
        &'map self,
        map_name: &'map str,
        key: &'map str,
    ) -> crate::Result<&str> {
        trace!("Iterate over the languages in the fallback chain until we find one that contains the localised string");
        self.set_chain_once(None)
            .iter()
            // If we find the localised string in a language, return it.
            .find_map(|lang| {
                trace!(
                    "Found localised string for '{}:{}' in language {:?}",
                    map_name,
                    key,
                    lang
                );
                self.get_text(lang, map_name, key)
            }) // Call `get_text` on each language until a lookup succeeds.
            .ok_or_else(|| GlossaError::map_text_not_found(map_name, key))
    }

    /// Gets a localised string for a given language, map-name and key.
    fn get_text(&self, lang: &LangID, map_name: &str, key: &str) -> Option<&str> {
        self.get_map()
            .get(lang)
            .and_then(|m| {
                trace!("Try to get the localised string from the sub-map for the given map-name.");
                Self::get_from_sub_map(m, map_name, key)
            })
    }

    /// Try to get a localised string from a sub-map for a given map-name and key. To be more precise, get the text from a separate sub-map.
    fn get_from_sub_map<'a>(
        sub_map: &SubLocaleMap,
        map_name: &str,
        key: &str,
    ) -> Option<&'a str> {
        sub_map
            .get(map_name)
            .and_then(|f| f().get(key).copied())
    }
}

impl<S: BuildHasher> GetText<S> for MapLoader<S> {
    fn get_map(&self) -> &super::Map<S> {
        self.get_map()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{
        assets::localisation::locale_hashmap, map_loader::MapLoader, GetText,
    };

    #[test]
    fn get_error_msg() -> anyhow::Result<()> {
        let loader = MapLoader::new(locale_hashmap());
        // loader.show_chain();

        let s = loader.get_or_default("error", "not-found");
        println!("{s}");

        Ok(())
    }

    #[test]
    #[cfg(feature = "ahash")]
    fn use_ahash_and_std_maps() {
        // ahash
        let map1 = ahash::HashMap::from_iter(locale_hashmap().into_iter());
        // std hash:
        let map2 =
            std::collections::HashMap::from_iter(locale_hashmap().into_iter());

        let mut ld1 = MapLoader::new(map1);
        let str1 = ld1.get_or_default("error", "text-not-found 1");
        println!("{str1}");

        *ld1.get_map_mut() = map2;
        let str2 = ld1.get_or_default("error", "text-not-found 2");
        println!("{str2}");
    }
}
