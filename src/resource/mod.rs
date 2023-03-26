use crate::resource::{fallback::FallbackChain, loader::FluentLoader};
use getset::{Getters, MutGetters};
use lang_id::LangID;
use once_cell::sync::OnceCell;

// mods:
mod deref;
pub mod fallback;
mod find;
mod from;
pub mod loader;

/// The `LangResource` struct represents a language resource comprising a `LangID`, a `FluentLoader`, and a `FallbackChain` for alternative language resources. The fallback chain can be lazily initialised and is immutable once set, ensuring effective and reliable handling of language localisation in software applications.
///
/// ## Overview
///
/// | Member Variable | Type                      | Description                                                       |
/// | --------------- | ------------------------- | ----------------------------------------------------------------- |
/// | `id`            | `LangID`                  | Represents the ID of the language resource.                       |
/// | `loader`        | `FluentLoader<'a>`        | Represents the loader for Fluent resources.                       |
/// | `chain`         | `OnceCell<FallbackChain>` | Represents the fallback chain for alternative language resources. |
///
/// ### ID
///
/// ID refers to language identifier, such as `en`, `en-001`, and `en-Latn-001` are all valid lang-ids.
///
/// `en` refers to en-Latn-US, not en-GB.
///
/// `001` refers to the world.
///
/// > The related rules come from Unicode CLDR (Unicode Common Locale Data Repository) version 42.0.0.
///
/// If you use `from()` to create a struct, you don't need to worry about the ID field because it will automatically obtain your system's language.
///
/// > For a detailed list of lang-ids, you can refer to [lang-id](https://github.com/2moe/lang-id).
///
/// ### loader
///
/// `FluentLoader` is an enum type that currently supports `StaticLoader` and `ArcLoader`, both of which come from fluent-templates.
///
/// Perhaps more loaders can be added in the future.
///
/// | Variable   | Type                    | Description                                                                                                                                                         |
/// | ---------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
/// | `Static`   | `&'static StaticLoader` | Represents a static Fluent loader that loads Fluent resources at compile time.                                                                                      |
/// | `Arc`      | `&'a ArcLoader`         | Represents a reference-counted smart pointer-based Fluent loader that allows loading resources at runtime. Both `'static ArcLoader` and `'a ArcLoader` can be used. |
/// | `ArcOwned` | `Arc<ArcLoader>`        | Represents a reference-counted smart pointer-based Fluent loader with ownership. The main difference from `Arc` loader is that it has ownership.                    |
///
/// The `FluentLoader` enum represents two types of Fluent loaders: the `StaticLoader`, which loads Fluent resources (i18n/l10n resources) at compile time, and the `ArcLoader`, which allows loading resources at runtime using both `'static ArcLoader` and `'a ArcLoader`. The `ArcOwned` variant is another type of `ArcLoader` that has ownership.
///
/// This enum is used to specify the type of Fluent loader when initializing a `LangResource` struct.
///
/// Note: `static_loader` comes from fluent-templates, you can use the same syntax as it.
///
/// ```no_run
/// use glossa::resource::loader::static_loader;
///
/// static_loader! {
///     static LOADER = {
///         locales: "locales",
///         fallback_language: "en",
///         customise: |bundle| bundle.set_use_isolating(false),
///     };
/// }
/// ```
///
/// ### About chain
///
/// `chain` can be lazily initialised. By default, it won't be initialised automatically when not needed and will only be initialised once when needed.
///
/// For example, if your language ID is "en" and all the localisation resources match, then it won't automatically initialise the fallback chain.
///
/// On the other hand, if your language ID is "zh-Hant-MO" and there are no matches, then it will automatically generate a fallback chain based on the resource list.
///
/// - For languages with the same name, scripts have higher priority than regions.
///   - If the current resource list is `["zh", "zh-Hans", "zh-Hant-HK", "zh-Hans-MO"]`, then `zh-Hant-HK` has higher priority than `zh-Hans-MO`.
///
/// If you need to customise the fallback chain, use `set_chain_once()` before calling `get()` or `get_with_kv()`.
///
/// Once the fallback chain is initialised, you cannot modify its value, but you can replace it with a new `OnceCell` instance.
///
/// ```no_run
/// let mut res = LangResource::from(&LOADER);
/// let chain = res.get_chain_mut();
/// *chain = OnceCell::new();
/// ```
#[derive(Getters, MutGetters, Clone)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct LangResource<'a> {
    id: LangID,
    loader: FluentLoader<'a>,
    chain: OnceCell<FallbackChain>,
}

/// Implements the Debug trait for the LangResource struct, allowing it to be printed with dbg!() and other debugging tools.
impl<'a> std::fmt::Debug for LangResource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LangResource")
            .field("id", &self.id)
            .field(
                "loader",
                &self
                    .loader
                    .get_locales()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(), // Adds a Vec of locales supported by the Fluent loader
            )
            .field("chain", &self.chain)
            .finish() // Finishes formatting the struct
    }
}

impl<'a> PartialEq for LangResource<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.chain == other.chain && {
            self.loader
                .get_locales()
                .zip(other.loader.get_locales())
                .all(|(a, b)| a == b)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{
        loader::{new_arc_loader, ArcLoader},
        LangResource,
    };
    use once_cell::sync::{Lazy, OnceCell};
    use std::path::PathBuf;

    static LOADER: Lazy<ArcLoader> = Lazy::new(|| {
        new_arc_loader(&PathBuf::from_iter(["assets", "test", "i18n"]), None)
            .expect("Failed to create arc loader")
    });

    // #[test]
    fn test_arc_loader<'a>() -> anyhow::Result<LangResource<'a>> {
        let res = LangResource::from(&LOADER);
        dbg!(&res);
        dbg!(res
            .get_with_kv("greeting", [("name", "Alice")])
            .unwrap());
        Ok(res)
    }

    #[test]
    fn lang_res_getter() -> anyhow::Result<()> {
        let mut res = LangResource::from(&LOADER);
        dbg!(
            test_arc_loader()?.show_chain(),
            res.show_chain(),
            res.get_chain(),
        );
        let chain = res.get_chain_mut();
        *chain = OnceCell::new();

        dbg!(res.get_chain());
        Ok(())
    }
}
