use crate::resource::LangResource;
use lang_id::LangID;
use std::{io::Write, ops::Deref};

/// Define a type alias for the fallback chain
pub type FallbackChain = Vec<LangID>;

/// Define the default language as English
pub const DEFAULT: LangID = unsafe { lang_id::consts::get_en() };

impl<'a> LangResource<'a> {
    /// Generates and returns the fallback chain as a static reference.
    /// The `custom` parameter is an optional custom fallback chain.
    pub fn set_chain_once(&self, custom: Option<FallbackChain>) -> &FallbackChain {
        // Use the `get_or_init` method of the `OnceCell` to generate or retrieve the fallback chain.
        self.chain.get_or_init(|| {
            if let Some(mut x) = custom {
                // Check if a custom fallback chain was provided.
                push_default_lang(&mut x); // If so, add the default language to the list (if it's not already present).
                return x; // Return the custom fallback chain.
            }

            #[cfg(debug_assertions)]
            println!(
                "(Note: This message will only appear at Debug time, not at release.)  Getting fb list...\ncurrent: {}, {:?}, {:?}",
                self.language, self.script, self.region
            );

            // Otherwise, generate the fallback chain from the language resource.
            let mut list = self.get_locale_list();
            self.sort_fallback_chain(&mut list);

            list
        })
    }

    /// Prints the generated fallback chain for debugging purposes.
    ///
    /// As the fallback_chain is a static variable and to reduce overhead, it is only generated on the first initialisation.
    ///
    /// If you need a custom chain, call `set_chain_once()` manually first, then call this function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let res = glossa::LangRes::new(unsafe { lang_id::consts::get_en_gb() }, loader);
    /// res.show_chain();
    /// ```
    pub fn show_chain(&self) {
        const OUT_ERR_MSG: &str = "Could not output to stdout";

        let mut out = std::io::stdout().lock();
        writeln!(out, "\ncurrent:  {} \nFallback:", **self).expect(OUT_ERR_MSG);

        for (lang, i) in self
            .set_chain_once(None)
            .iter()
            .zip(1usize..)
        {
            writeln!(out, "{i}:\t{lang}").expect(OUT_ERR_MSG)
        }

        out.flush()
            .expect("Failed to flush stdout")
    }

    /// Generates the initial fallback chain by filtering locales with the same language code but different variants.
    fn get_locale_list(&self) -> FallbackChain {
        self.loader
            .get_locales()
            .filter(|&x| x.language == self.language && x != self.deref())
            .cloned()
            .collect()
    }

    /// Sorts the fallback chain by comparing the script and region of each language ID.
    ///
    /// The primary sorting key is the script, followed by the region. If two language IDs have
    /// the same script, the one with a matching region is sorted first. Language IDs without
    /// a script or region are sorted last.
    fn sort_fallback_chain(&self, list: &mut FallbackChain) {
        let mut x = DEFAULT;

        list.sort_unstable_by_key(|id| {
            use std::cmp::Ordering::*;

            // Get a copy of the current LangID and maximize its subtags for comparison purposes.
            x = id.to_owned();
            x.maximize();

            // Compare the subtags of the current LangID to the subtags of the language resource.
            match (x.script, self.script, x.region, self.region) {
                (Some(xs), Some(ys), ..) if xs == ys => Less,
                (.., Some(xr), Some(yr)) if xr == yr => Less,
                _ => Greater,
            }
        });

        // Remove duplicate entries from the sorted fallback chain.
        list.dedup();

        push_default_lang(list);
    }
}

/// Add the default language to the provided fallback chain if it is not already present.
fn push_default_lang(x: &mut FallbackChain) {
    if !x.contains(&DEFAULT) {
        x.push(DEFAULT)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{
        fallback::{FallbackChain, DEFAULT},
        loader,
        loader::ArcLoader,
    };
    use lang_id::LangID;
    use once_cell::sync::{Lazy, OnceCell};
    use std::{collections::HashMap, env, fs, path::Path};

    #[test]
    #[cfg(target_os = "linux")]
    fn test_fb_list() -> anyhow::Result<()> {
        env::set_current_dir("/tmp")?;

        let ch = Path::new("locales/de-CH");
        if ch.exists() {
            fs::remove_dir_all(ch)?
        }

        static L10N_LOADER: Lazy<ArcLoader> = Lazy::new(|| {
            loader::new_arc_loader("locales", None)
                .expect("Failed to create arc loader")
        });

        {
            for i in 0..5 {
                let loader = if i == 3 {
                    fs::create_dir_all(ch)?;
                    fs::File::create(ch.join("main.ftl"))?;
                    static TEST_LOADER: Lazy<ArcLoader> = Lazy::new(|| {
                        loader::new_arc_loader("locales", None)
                            .expect("Failed to create arc loader")
                    });

                    &TEST_LOADER
                } else {
                    &L10N_LOADER
                };

                let res_de = crate::LangRes::with_arc(
                    unsafe { lang_id::consts::get_de() },
                    loader,
                );

                res_de.show_chain();

                std::thread::spawn(move || res_de.show_chain());
                // .join()
                // .expect("thread panic! Failed to show fb list.");
            }
        }
        fs::remove_dir_all(ch)?;

        let res_zh = crate::LangRes::with_arc(
            unsafe { lang_id::consts::get_zh_hant_mo() },
            &L10N_LOADER,
        );

        res_zh.set_chain_once(None);
        res_zh.show_chain();
        res_zh.show_chain();
        Ok(())
    }

    #[test]
    fn once_cell_static() {
        static A: OnceCell<FallbackChain> = OnceCell::new();
        test_once_cell(&A, DEFAULT);
        dbg!(&A);

        static B: OnceCell<FallbackChain> = OnceCell::new();
        test_once_cell(&B, unsafe { lang_id::consts::get_zh_hans_mo() });
        dbg!(&B);
        let mut map = HashMap::new();
        map.insert(DEFAULT, "v");
    }

    fn test_once_cell(
        a: &'static OnceCell<FallbackChain>,
        lang: LangID,
    ) -> &'static FallbackChain {
        a.get_or_init(|| vec![lang])
    }

    #[test]
    fn once_cell_struct() {
        #[derive(Debug)]
        struct A {
            u: OnceCell<u32>,
        }

        impl A {
            fn new(num: u32) -> Self {
                let cell = OnceCell::new();
                let u = num;
                cell.get_or_init(|| u);
                Self { u: cell }
            }
        }

        let mut a = A::new(3);
        dbg!(&a.u);
        a = A::new(6);
        dbg!(&a);
    }
}
