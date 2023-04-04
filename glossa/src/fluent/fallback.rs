use crate::{fallback::Chain, fluent::LangResource};
use lang_id::LangID;

impl<'a> crate::fallback::FallbackChain for LangResource<'a> {
    fn get_id(&self) -> &LangID {
        self.get_id()
    }

    /// Generates the initial fallback chain by filtering locales with the same language code but different variants.
    fn get_locale_list(&self) -> Chain {
        self.loader
            .get_locales()
            .filter(|&x| self.locale_list_filter(x))
            .cloned()
            .collect()
    }

    fn set_chain_once(&self, custom: Option<Chain>) -> &Chain {
        self.get_chain()
            .get_or_init(|| self.new_chain(custom))
    }
}

#[cfg(test)]
mod tests {
    use crate::fluent::{loader, loader::ArcLoader};
    use once_cell::sync::Lazy;
    use std::{env, fs, path::Path};

    #[test]
    #[cfg(target_os = "linux")]
    fn test_fb_list() -> anyhow::Result<()> {
        use crate::fallback::FallbackChain;

        env::set_current_dir("/tmp")?;

        let ch = Path::new("../locales/de-CH");
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
}
