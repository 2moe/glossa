use crate::{
    log::{debug, info, trace},
    LangID,
};
use std::io::Write;

/// Define a type alias for the fallback chain
pub type Chain = Vec<LangID>;

pub trait FallbackChain {
    /// Define the default language as English
    const DEFAULT: LangID = unsafe { lang_id::consts::get_en() };
    // type OnceChain = OnceCell<Self::Chain>;

    /// Gets the Language ID
    fn get_id(&self) -> &LangID;

    /// Creates a new fallback chain
    fn new_chain(&self, custom: Option<Chain>) -> Chain {
        trace!("Check if a custom fallback chain was provided");
        if let Some(mut x) = custom {
            info!("Custom FallBack Chain detected");
            debug!("{:?}", &x);
            Self::push_default_lang(&mut x); // If so, add the default language to the list (if it's not already present).
            debug!("Return the custom fallback chain");
            return x;
        }

        info!("Getting fallback chain ...");

        info!(
            "The fallback chain is being generated from the language resources ..."
        );

        debug!("Call `get_locale_list()` to generate a list of current similar languages");
        let mut chain = self.get_locale_list();
        info!("Locale list: {:?}", chain); // Logging the created list of locales
        info!("About to sort the fallback chain");
        self.sort_fallback_chain(&mut chain);

        chain
    }

    /// Generates and returns the fallback chain as a static reference.
    /// The `custom` parameter is an optional custom fallback chain.
    ///
    /// A possible implementation:
    ///
    /// ```no_run
    /// fn set_chain_once(&self, custom: Option<Chain>) -> &Chain {
    ///     // Use the `get_or_init` method of the `OnceCell` to generate or retrieve the fallback chain.
    ///     self.get_chain().get_or_init(|| self.new_chain(custom))
    /// }
    /// ```
    fn set_chain_once(&self, custom: Option<Chain>) -> &Chain;

    /// Prints the generated fallback chain for debugging purposes.
    ///
    /// The fallback_chain is only generated on the first initialisation.
    ///
    /// If you need a custom chain, call `set_chain_once()` manually first, then call this function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use crate::assets::localisation::locale_hashmap;
    /// use glossa::{
    ///     fallback::FallbackChain,
    ///     MapLoader,
    /// };
    /// let loader = MapLoader::new(locale_hashmap());
    /// loader.show_chain();
    /// ```
    fn show_chain(&self) {
        const OUT_ERR_MSG: &str = "Could not output to stdout";

        debug!("About to lock the standard output stream");
        let mut out = std::io::stdout().lock();

        trace!("current:  {} \nFallback:", self.get_id());
        writeln!(out, "\ncurrent:  {}\nFallback:", self.get_id())
            .expect(OUT_ERR_MSG);

        for (lang, i) in self
            .set_chain_once(None)
            .iter()
            .zip(1usize..)
        {
            info!("Chain iteration {}: {}", i, lang);
            writeln!(out, "{i}:\t{lang}").expect(OUT_ERR_MSG)
        }

        debug!("About to flush the standard output stream");
        out.flush()
            .expect("Failed to flush stdout")
    }

    /// You can filter locales with the same language code, but with different variants
    ///
    /// A possible implementation:
    ///
    /// ```no_run
    /// fn get_locale_list(&self) -> Chain {
    ///     iter.filter(|x| self.locale_list_filter(x))
    ///     .cloned()
    ///     .collect()
    /// };
    /// ```
    fn get_locale_list(&self) -> Chain;

    fn locale_list_filter(&self, id: &LangID) -> bool {
        id.language == self.get_id().language && id != self.get_id()
    }

    /// Sorts the fallback chain by comparing the script and region of each language ID.
    ///
    /// The primary sorting key is the script, followed by the region. If two language IDs have
    /// the same script, the one with a matching region is sorted first. Language IDs without
    /// a script or region are sorted last.
    fn sort_fallback_chain(&self, chain: &mut Chain) {
        let mut x = Self::DEFAULT;
        trace!("Set a mutable variable with the value `{x}`");

        let mut max = self.get_id().to_owned();
        max.maximize();

        info!(
            "current: {}, {:?}, {:?}",
            max.language, max.script, max.region,
        );

        chain.sort_unstable_by_key(|id| {
            use std::cmp::Ordering::*;

            trace!("Get a copy of the current LangID and maximize its subtags for comparison purposes.");
            x = id.to_owned();

            info!("Maximising the x variable, org-x: {x}");
            x.maximize();
            info!("maximised-x: {x}");

            trace!("Compare the subtags of the current LangID to the subtags of the language resource.");
            match (
                x.script,
                max.script,
                x.region,
                max.region,
            ) {
                (Some(xs), Some(ys), ..) if xs == ys => {
                    debug!("x.script = self.script,\nxs:{xs}, self.script: {ys}");
                    Less
                },
                (.., Some(xr), Some(yr)) if xr == yr => {
                    debug!("x.region = self.region,\nxr:{xr}, self.region: {yr}");
                    Less
                },
                _ => Greater,
            }
        });

        trace!("Remove duplicate entries from the sorted fallback chain.");
        chain.dedup();

        Self::push_default_lang(chain);
    }

    /// Adds the default language to the provided fallback chain if it is not already present.
    fn push_default_lang(x: &mut Chain) {
        if !x.contains(&Self::DEFAULT) {
            debug!(
                "Adding the default language:({}, {:?}, {:?}) to the provided fallback chain",
                Self::DEFAULT.language,
                Self::DEFAULT.script,
                Self::DEFAULT.region
            );
            x.push(Self::DEFAULT)
        }
    }
}
