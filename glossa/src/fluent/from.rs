use fluent_templates::{ArcLoader, StaticLoader};
use lang_id::LangID;
use once_cell::sync::{Lazy, OnceCell};

use crate::fluent::{loader::FluentLoader, LangResource};

/// Implementation of the `From` trait for `StaticLoader`.
impl<'a> From<&'static StaticLoader> for LangResource<'a> {
    /// Converts a static loader to a LangResource.
    fn from(loader: &'static StaticLoader) -> Self {
        LangResource::from_static_loader(loader)
    }
}

type OnceStaticLoader = Lazy<StaticLoader>;
impl<'a> From<&'static OnceStaticLoader> for LangResource<'a> {
    /// This implementation is very similar to `From<&'static StaticLoader>`,
    /// but the type passed in is different.
    fn from(loader: &'static OnceStaticLoader) -> Self {
        LangResource::from_static_loader(loader)
    }
}

/// Implementation of the `From` trait for `&'a ArcLoader`.
impl<'a> From<&'a ArcLoader> for LangResource<'a> {
    /// Converts an arc loader to a LangResource.
    ///
    /// This does not require ownership of the ArcLoader.
    fn from(loader: &'a ArcLoader) -> Self {
        LangResource::from_arc_loader(loader)
    }
}

/// Implementation of the `From` trait for `ArcLoader`.
impl<'a> From<ArcLoader> for LangResource<'a> {
    /// Converts an arc loader to a LangResource.
    ///
    /// This requires ownership of the ArcLoader.
    fn from(loader: ArcLoader) -> Self {
        Self::with_arc_owned(Self::get_current_locale().to_owned(), loader)
    }
}

type OnceArcLoader = Lazy<ArcLoader>;
impl<'a> From<&'a OnceArcLoader> for LangResource<'a> {
    /// Converts an static arc loader to a LangResource.
    fn from(loader: &'a OnceArcLoader) -> Self {
        LangResource::from_arc_loader(loader)
    }
}

/// Implementation of the LangResource.
impl<'a> LangResource<'a> {
    /// Creates a new LangResource from a static loader.
    ///
    /// It will use `get_current_locale()` to get your system language, you just need to pass in a StaticLoader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glossa::fluent::{loader::static_loader, LangResource};
    ///
    /// static_loader! {
    ///     static L10N_LOADER = {
    ///         locales: "locales",
    ///         fallback_language: "en",
    ///         customise: |bundle| bundle.set_use_isolating(false),
    ///     };
    /// }
    ///
    /// let res = LangResource::from_static_loader(&L10N_LOADER);
    /// ```
    ///
    /// Since it implements `From` Trait for `Lazy<StaticLoader>`, you can use `from()` directly
    pub fn from_static_loader(loader: &'static StaticLoader) -> Self {
        Self::new(Self::get_current_locale().to_owned(), loader)
    }

    /// Creates a new LangResource instance from an arc loader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glossa::fluent::{
    ///     loader::{new_arc_loader, ArcLoader},
    ///     LangResource,
    /// };
    /// use once_cell::sync::Lazy;
    /// use std::path::Path;
    ///
    /// static LOADER: Lazy<ArcLoader> = Lazy::new(|| {
    ///     new_arc_loader(Path::new("locales"), None)
    ///     .expect("Failed to create arc loader")
    /// });
    ///
    /// // `from_arc_loader()` can be simplified to `from()`
    /// let res = LangResource::from(&LOADER);
    /// dbg!(&res);
    /// ```
    ///
    /// See also [`new_arc_loader()`](crate::fluent::loader::new_arc_loader)
    pub fn from_arc_loader(loader: &'a ArcLoader) -> Self {
        Self::with_arc(Self::get_current_locale().to_owned(), loader)
    }

    /// Creates a new LangResource with the specified locale and static loader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glossa::fluent::{loader::static_loader, LangResource};
    /// use lang_id::consts::get_en_gb;
    ///
    /// static_loader! {
    ///     static L10N_LOADER = {
    ///         locales: "locales",
    ///         fallback_language: "en",
    ///         customise: |bundle| bundle.set_use_isolating(false),
    ///     };
    /// }
    ///
    /// let lang = unsafe { get_en_gb() };
    /// let res = LangResource::new(lang, &L10N_LOADER);
    ///
    /// dbg!(res);
    /// ```
    pub fn new(id: LangID, loader: &'static StaticLoader) -> Self {
        Self {
            id,
            loader: FluentLoader::Static(loader),
            chain: OnceCell::new(),
        }
    }

    /// Creates a new LangResource with the specified locale and arc loader.
    ///
    /// # Examples
    ///
    /// You can pass in a regular reference ArcLoader.
    ///
    /// ```no_run
    /// use glossa::{
    ///     fluent::loader::{new_arc_loader, ArcLoader},
    ///     LangRes,
    /// };
    /// use once_cell::sync::Lazy;
    /// use std::path::Path;
    ///
    /// const ERR_MSG: &str = "Failed to create arc loader";
    ///
    /// let lang = LangRes::get_current_locale();
    ///
    /// let loader = new_arc_loader(Path::new("locales"), None).expect(ERR_MSG);
    /// let res = LangRes::with_arc(lang.to_owned(), &loader);
    /// dbg!(&res);
    /// ```
    ///
    /// You can also pass in a 'static ArcLoader.
    ///
    /// ```no_run
    /// static LOADER: Lazy<ArcLoader> =
    ///     Lazy::new(|| new_arc_loader(Path::new("locales"), None).expect(ERR_MSG));
    ///
    /// let res = LangRes::with_arc(lang.to_owned(), &LOADER);
    ///
    /// dbg!(&res);
    /// ```
    ///
    /// If you don't need to customise the language, but use the system language(`get_current_locale()`), then use `from()`.
    ///
    /// See also `from_arc_loader()`
    pub fn with_arc(id: LangID, loader: &'a ArcLoader) -> Self {
        Self {
            id,
            loader: FluentLoader::Arc(loader),
            chain: OnceCell::new(),
        }
    }

    /// Similar to `with_arc()`, but requires ownership of the ArcLoader.
    pub fn with_arc_owned(id: LangID, loader: ArcLoader) -> Self {
        Self {
            id,
            loader: FluentLoader::ArcOwned(loader),
            chain: OnceCell::new(),
        }
    }

    /// Returns the current locale.
    ///
    /// # Examples
    ///
    /// ```
    /// let lang = glossa::LangRes::get_current_locale();
    ///
    /// dbg!(&lang);
    /// ```
    pub fn get_current_locale() -> &'static LangID {
        static ID: OnceCell<LangID> = OnceCell::new();
        ID.get_or_init(|| {
            let mut loc = lang_id::sys_lang::current();
            loc.maximize();
            loc
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_with_ref_arc() {
        use crate::{
            fluent::loader::{new_arc_loader, ArcLoader},
            LangRes,
        };
        use once_cell::sync::Lazy;
        use std::path::Path;

        const ERR_MSG: &str = "Failed to create arc loader";

        let lang = LangRes::get_current_locale();

        let loader = new_arc_loader(Path::new("locales"), None).expect(ERR_MSG);
        let res = LangRes::with_arc(lang.to_owned(), &loader);
        dbg!(&res);
        // You can also pass in a 'static ArcLoader.

        static LOADER: Lazy<ArcLoader> =
            Lazy::new(|| new_arc_loader(Path::new("locales"), None).expect(ERR_MSG));
        let res = LangRes::with_arc(lang.to_owned(), &LOADER);

        dbg!(&res);
    }
    #[test]
    fn new_language_resource() -> crate::Result<'static, ()> {
        use crate::{
            fluent::loader::{new_arc_loader, ArcLoader},
            LangRes,
        };
        use once_cell::sync::Lazy;
        use std::path::Path;

        // use glossa::{fluent::loader::new_arc_loader, LangRes};

        let lang = LangRes::get_current_locale();

        let loader = new_arc_loader(Path::new("locales"), None)?;
        let res = LangRes::with_arc(lang.to_owned(), &loader);
        dbg!(&res);

        // Or
        static LOADER: Lazy<ArcLoader> = Lazy::new(|| {
            new_arc_loader(Path::new("locales"), None)
                .expect("Failed to create arc loader")
        });
        let res = LangRes::with_arc(lang.to_owned(), &LOADER);

        dbg!(&res);

        Ok(())
    }

    #[test]
    fn test_from_static_loader() {
        use crate::fluent::{loader::static_loader, LangResource};

        static_loader! {
            static L10N_LOADER = {
                locales: "../locales",
                fallback_language: "en",
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }

        let res = LangResource::from(&L10N_LOADER);

        dbg!(res
            .loader
            .get_locales()
            .collect::<Vec<_>>());
    }

    #[test]
    fn test_from_arc() {
        use crate::fluent::{
            loader::{new_arc_loader, ArcLoader},
            LangResource,
        };
        use once_cell::sync::Lazy;
        use std::path::Path;

        static LOADER: Lazy<ArcLoader> = Lazy::new(|| {
            new_arc_loader(Path::new("locales"), None)
                .expect("Failed to create arc loader")
        });
        let res = LangResource::from(&LOADER);
        dbg!(res);
    }
}
