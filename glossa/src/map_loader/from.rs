use crate::{assets::OnceCell, debug, map_loader::Map, LangID, MapLoader};
use std::hash::BuildHasher;

impl<S: BuildHasher> MapLoader<S> {
    /// Creates a new "MapLoader" instance with the current system language ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use glossa::fallback::FallbackChain;
    /// use crate::assets::localisation::locale_hashmap;
    ///
    /// let loader = glossa::MapLoader::new(locale_hashmap());
    /// let msg = loader.get_or_default("test", "hello");
    /// dbg!(msg);
    /// loader.show_chain();
    /// ```
    pub fn new(map: Map<S>) -> Self {
        let mut cur = lang_id::sys_lang::current();

        debug!("Current lang id: {}", cur);

        let id = match &map {
            m if m.contains_key(&cur) => cur,
            m => {
                cur.maximize();

                match m.contains_key(&cur) {
                    true => cur,
                    _ => {
                        let max = cur.to_owned();
                        cur.minimize();
                        match m.contains_key(&cur) {
                            true => cur,
                            _ => max,
                        }
                    }
                }
            }
        };

        debug!("lang id: {}", id);

        Self {
            id,
            map,
            chain: OnceCell::new(),
        }
    }

    /// Similar to `new()`, but you need to pass in a specific language ID instead of letting it detect it automatically.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use glossa::{
    ///     assets::lang_id_consts,
    ///     MapLoader,
    /// };
    /// use crate::assets::localisation::locale_hashmap;
    ///
    /// let id = unsafe { lang_id_consts::get_en_gb() };
    /// let loader = MapLoader::with_language_id(id, locale_hashmap());
    /// let msg = loader.get_or_default("test", "hello");
    /// dbg!(msg);
    /// ```
    pub fn with_language_id(id: LangID, map: Map<S>) -> Self {
        Self {
            id,
            map,
            chain: OnceCell::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{fallback::FallbackChain, GetText};

    // use super::*;
    #[test]
    fn new_map_loader() {
        use crate::{
            assets::localisation::locale_hashmap, fallback::FallbackChain,
            map_loader::MapLoader,
        };
        let loader = MapLoader::new(locale_hashmap());
        let _err_msg = loader.get("error", "text-not-found");

        loader.show_chain();

        let zh = unsafe { lang_id::consts::get_zh() };
        let map = &loader.get_map().get(&zh);

        map.map(|s| println!("{s:?}"));
    }

    #[test]
    fn from_lang_id_and_map() {
        use crate::{
            assets::{lang_id_consts, localisation::locale_hashmap},
            MapLoader,
        };

        let id = unsafe { lang_id_consts::get_el() };

        let loader = MapLoader::with_language_id(id, locale_hashmap());
        loader.show_chain();
        let msg = loader.get_or_default("test", "not-found");
        println!("{msg}");
    }
    #[test]
    fn new_loader() {
        use crate::assets::localisation::locale_hashmap;

        let loader = crate::MapLoader::new(locale_hashmap());
        loader.show_chain();
        let msg = loader.get_or_default("test", "hello");
        println!("{msg}");
    }
}
