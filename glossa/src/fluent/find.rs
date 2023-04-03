use crate::{
    error::GlossaError,
    fallback::FallbackChain,
    fluent::{loader::FluentLoader, LangResource},
};
use fluent_templates::fluent_bundle::FluentValue;
use lang_id::LangID;
use std::collections::HashMap;

/// A type alias for a map of Fluent values. The lifetime 'm represents the lifetime of the HashMap values.
type FluentMap<'m, K> = Option<&'m HashMap<K, FluentValue<'m>>>;

impl<'a> LangResource<'a> {
    /// Helper function that looks up a single text string with the given language, ID, and Fluent map.
    fn get_text<K: AsRef<str>>(
        &self,
        lang: &LangID,
        id: &str,
        map: FluentMap<K>,
    ) -> crate::Result<String> {
        use FluentLoader::*; // Import the variants of the FluentLoader enum into scope.
        let or_glossa_err =
            |opt: Option<String>| opt.ok_or_else(|| GlossaError::text_not_found(id));

        match self.loader {
            // Match on the `FluentLoader` instance inside the `LangResource`.
            Static(s) => or_glossa_err(s.lookup_single_language(lang, id, map)),
            Arc(s) => or_glossa_err(s.lookup_single_language(lang, id, map)),
            ArcOwned(ref s) => {
                or_glossa_err(s.lookup_single_language(lang, id, map))
            }
        }
    }

    fn get_from_fallback_chain<K: AsRef<str>>(
        &self,
        id: &str,
        map: FluentMap<K>,
    ) -> crate::Result<String> {
        // First, try to look up the text in the current language using the `get_text` helper function.
        if let Ok(x) = self.get_text(self, id, map) {
            // Call `get_text` on the current `LangResource`.
            return Ok(x); // If the lookup succeeds, return the text string.
        }

        self.set_chain_once(None)
            .iter() // Iterate over the fallback languages.
            .find_map(|l| self.get_text(l, id, map).ok()) // Call `get_text` on each language until a lookup succeeds.
            .ok_or_else(|| GlossaError::text_not_found(id)) // If no lookup succeeds, return an error.
    }

    /// A function to find a text string with the given ID.
    ///
    /// In fact, it calls `get_from_fallback_chain()` in the same way as `get_with_map()`, but the map is None.
    /// This is useful for simple cases. For example, if `hi = hello world`, we only need the id (i.e. hi), not the map.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let res = LangRes::from(&LOADER);
    /// // locales/en/test.ftl: welcome = Welcome to glossa🥳
    /// // locales/de/test.ftl: welcome = Willkommen bei glossa😚
    /// // locales/zh/test.ftl: welcome = 欢迎使用 glossa🥰
    ///
    /// let text = res
    ///     .get("welcome")
    ///     .expect(r#"Failed to get the value of "welcome" from locales/[lang-id]/test.ftl."#);
    ///
    /// // if the res.language is en, then
    /// assert_eq!(text, "Welcome to glossa🥳")
    /// // de:
    /// assert_eq!(text, "Willkommen bei glossa😚")
    /// // zh:
    /// assert_eq!(text, "欢迎使用 glossa🥰")
    /// ```
    pub fn get(&self, id: &str) -> crate::Result<String> {
        self.get_from_fallback_chain::<&str>(id, None) // Call `get_from_fallback_chain` with an empty Fluent map (i.e., `None`) and a type parameter of `&str`.
    }

    /// Similar to `get()`. If it succeeds, it returns a string; If it fails, it returns an error message of type String instead of `Result`.
    pub fn get_or_default(&self, id: &str) -> String {
        self.get(id)
            .unwrap_or_else(|e| e.to_string())
    }

    /// Public function to find a text string with the given ID and key-value pairs.
    ///
    /// # Examples
    ///
    /// locales/en-GB/test.ftl:
    ///
    ///  ```ftl
    /// time-period = { $period ->
    ///     *[morning] Good morning
    ///     [evening] Good evening
    /// }
    ///
    /// appellation = { $gender ->
    ///     [male] Mr.
    ///     *[female] Ms.
    /// }
    ///
    /// greetings = { time-period }! { appellation }{ $name }
    ///
    /// ```
    ///
    /// Here we will assume that your language is en-GB.
    /// This will give priority to parsing `locales/en-GB/test.ftl`
    ///
    /// ```no_run，should_panic
    /// let res = LangRes::from(&LOADER);
    ///
    /// let text = res
    ///   .get_with_kv(
    ///     "greetings",
    ///       [
    ///         ("period", "evening"),
    ///         ("name", "Alice"),
    ///         ("gender", "unknown"),
    ///       ],
    /// ).expect(r#"Failed to get "greetings"! "#);
    ///
    /// assert_eq!(text, "Good evening! Ms.Alice");
    /// ```
    pub fn get_with_kv<'v, K, V, T>(&self, id: &str, kv: T) -> crate::Result<String>
    where
        K: AsRef<str> + Eq + ::core::hash::Hash,
        V: Into<FluentValue<'v>>,
        T: IntoIterator<Item = (K, V)>,
    {
        // Convert the key-value pair iterator into a HashMap of Fluent values.
        let map = HashMap::from_iter(
            kv.into_iter()
                .map(|(k, v)| (k, v.into())),
        );
        // Search for the given ID in the fallback chain using the `get_from_fallback_chain` helper function.
        self.get_from_fallback_chain(id, Some(&map))
    }

    /// Similar to `get_with_kv()`. If it succeeds, it returns a string; If it fails, it returns an error message of type String instead of `Result`.
    pub fn get_with_kv_or_default<'v, K, V, T>(&self, id: &str, kv: T) -> String
    where
        K: AsRef<str> + Eq + ::core::hash::Hash,
        V: Into<FluentValue<'v>>,
        T: IntoIterator<Item = (K, V)>,
    {
        self.get_with_kv(id, kv)
            .unwrap_or_else(|e| e.to_string())
    }

    /// Similar to `get_with_kv()`, both are used to find localised text, but the difference is that this function passes in `&HashMap<k, v>`, instead of `[(k1, v1), (k2, v2)]`
    pub fn get_with_map<K: AsRef<str>>(
        &self,
        id: &str,
        map: &HashMap<K, FluentValue>,
    ) -> crate::Result<String> {
        self.get_from_fallback_chain(id, Some(map)) // Call `get_from_fallback_chain` with the provided Fluent map and the ID to search for.
    }

    /// Similar to `get_with_map()`. but always returns String, not Result.
    /// This is equivalent to `.get_with_map(id, map).unwrap_or_else(|e| e.to_string())`
    pub fn get_with_map_or_default<K: AsRef<str>>(
        &self,
        id: &str,
        map: &HashMap<K, FluentValue>,
    ) -> String {
        self.get_with_map(id, map)
            .unwrap_or_else(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get() {
        use crate::{
            fluent::loader::{new_arc_loader, ArcLoader},
            LangRes,
        };
        use once_cell::sync::Lazy;
        use std::{
            fs::{self, File},
            io::{self, Write},
            path::{Path, PathBuf},
        };

        /// This function is used to create a directory and write the contents to the relevant file.
        fn create_l10n_text<P: AsRef<Path>>(
            path: P,
            append: bool,
            contents: &str,
        ) -> io::Result<()> {
            let path = path.as_ref();

            fs::create_dir_all(
                path.parent()
                    .expect(r#"Please bring your parent to meet me😅.
                    Just joke, this path doesn't seem to contain the previous level, please double check.
                    For example: for `main.ftl`, you need to store it in "en/main.ftl" or "en-GB/main.ftl". 
                    (The lang-id can be changed at will)"#),
            )?;

            let mut file = File::options()
                .create(true)
                .append(append)
                .write(true)
                .open(path)?;

            writeln!(file, "{contents}")
        }

        const IO_MSG: &str = "I/O Error, failed to create/write the file";

        // Specify the fluent file for en-Latn-US
        // On Windows, it's "locales\en\test.ftl". On Unix, it's "locales/en/test.ftl".
        let mut arr = ["locales", "en", "test.ftl"];
        let mut file = PathBuf::from_iter(&arr);
        create_l10n_text(file, false, "welcome = Welcome to glossa🥳")
            .expect(IO_MSG);

        // We change the second element from "en" to "de" to start our German localisation.
        if let Some(p) = arr.iter_mut().nth(1) {
            *p = "de"
        }

        file = PathBuf::from_iter(&arr);
        create_l10n_text(file, false, "welcome = Willkommen bei glossa😚")
            .expect(IO_MSG);

        // We change the second element to "zh"
        if let Some(p) = arr.iter_mut().nth(1) {
            *p = "zh"
        }

        file = PathBuf::from_iter(arr);
        create_l10n_text(file, false, "welcome = 欢迎使用 glossa🥰").expect(IO_MSG);

        const ERR_MSG: &str = "Failed to create arc loader";
        static LOADER: Lazy<ArcLoader> =
            Lazy::new(|| new_arc_loader(Path::new("locales"), None).expect(ERR_MSG));

        // Although there is only Loader, not Lang, `from()` will automatically set your system language to the language of `LangRes`.
        let res = LangRes::from(&LOADER);

        let text = res.get("welcome").expect(
        r#"Failed to get the value of "welcome" from locales/[lang-id]/test.ftl."#,
    );

        // Since I'm not sure what language your system is in, I'm using match to determine the language, and then asset.
        // In fact, this step is not needed at all.
        // When you call `get()`, the text will already be the localised text you want.
        // If it can't be found, then it's probably not what you want, but it will automatically use fallback. e.g. zh-Hant-HK -> zh-Hant -> zh -> en
        match res.language.as_str() {
            "zh" => assert_eq!(text, "欢迎使用 glossa🥰"),
            "de" => assert_eq!(text, "Willkommen bei glossa😚"),
            _ => assert_eq!(text, "Welcome to glossa🥳"),
        }

        let text = res
            .get_with_kv(
                "greetings",
                [
                    ("period", "evening"),
                    ("name", "Alice"),
                    ("gender", "unknown"),
                ],
            )
            .expect(r#"Failed to get "greetings"! "#);

        assert_eq!(text, "Good evening! Ms.Alice");

        let text = res.get_with_kv(
            "greeting",
            [
                ("period", "morning"),
                ("name", "Alice"),
                ("person", "noble-young-female"),
            ],
        );

        dbg!(text.unwrap());
    }
}
