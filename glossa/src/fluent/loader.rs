use crate::{error::GlossaError, info, trace, Result};
pub use fluent_templates::{self, static_loader, ArcLoader};
use fluent_templates::{Loader, StaticLoader};
use lang_id::LangID;
use std::{
    io,
    path::{Path, PathBuf},
};

/// An enum representing types of Fluent loaders
/// - Static loader (which loads Fluent resources(i18n/l10n resources) at compile time)
/// - Ref Arc loader (which can load resources at runtime).
///   - In addition to `&'a ArcLoader`, you can also use `&'static ArcLoader`.
/// - Owned Arc loader (The main difference with the `Ref Arc loader` is that it has ownership).
///
/// | Variable   | Type                    | Description                                                                                     |
/// | ---------- | ----------------------- | ----------------------------------------------------------------------------------------------- |
/// | `Static`   | `&'static StaticLoader` | Allows loading Fluent resources at compile time.                                                |
/// | `Arc`      | `&'a ArcLoader`         | Allows loading resources at runtime. Both `&'static ArcLoader` and `&'a ArcLoader` can be used. |
/// | `ArcOwned` | `ArcLoader`             | The main difference from `&'a ArcLoader` is that it has ownership.                              |
pub enum FluentLoader<'a> {
    Static(&'static StaticLoader),
    Arc(&'a ArcLoader),
    ArcOwned(ArcLoader),
}

impl<'a> FluentLoader<'a> {
    /// Returns an iterator over the locales supported by the Fluent loader.
    pub(crate) fn get_locales(&self) -> Box<dyn Iterator<Item = &LangID> + '_> {
        trace!("Getting locales");

        info!("hello");
        match self {
            // If the loader is a static loader, return an iterator over its locales.
            Self::Static(x) => x.locales(),
            // If the loader is an Arc loader, return an iterator over its locales.
            Self::Arc(a) => a.locales(),
            Self::ArcOwned(o) => o.locales(),
        }
    }
}

/// Creates a new `ArcLoader` instance by configuring an `ArcLoaderBuilder`.
///
/// # Parameters
///
/// - `dir`: A reference to the directory path containing the translation files.
/// - `shared`: Some fluent resources that are shared with all locales. For example, if **core.ftl** has `-app-name = glossa`, then if `shared` contains the file **core.ftl**, then **-app-name** will be shared.
///
/// # Returns
///
/// Returns a `Result` containing the newly created `ArcLoader`,
/// or a `GlossaError` if there was a problem creating the loader.
///
/// # Examples
///
/// ```no_run
///     use glossa::fluent::{
///         loader::{new_arc_loader, ArcLoader},
///         LangResource,
///     };
///     use once_cell::sync::Lazy;
///     use std::path::PathBuf;
///
///     static LOADER: Lazy<ArcLoader> = Lazy::new(|| {
///         // "assets/test/i18n" is a resource dir.
///         // "assets/test/i18n/en/main.ftl", "assets/test/i18n/en-GB/main.ftl" are some fluent resources.
///         new_arc_loader(&PathBuf::from_iter(["assets", "test", "i18n"]), None)
///             .expect("Failed to create arc loader")
///         });
///     let res = LangResource::from_arc_loader(&LOADER);
/// ```
pub fn new_arc_loader<'a, P>(
    dir: &P,
    shared: Option<&[PathBuf]>,
) -> Result<'a, ArcLoader>
where
    P: AsRef<Path> + ?Sized,
{
    // Create a new ArcLoaderBuilder with the given directory and default options
    ArcLoader::builder(dir, unsafe { lang_id::consts::get_en() })
        // Set shared resources
        .shared_resources(shared)
        // Customize the fluent bundle by disabling message isolation
        .customize(|bundle| bundle.set_use_isolating(false))
        // Build the ArcLoader instance from the configured builder
        .build()
        // Map any error produced during creation to a GlossaError
        .map_err(|e| {
            GlossaError::CreateArcLoader(io::Error::new(
                io::ErrorKind::Other,
                e.to_string(),
            ))
        })
}

// #[cfg(test)]
// mod tests {
//     use crate::l10n::locales;

//     #[test]
//     fn get_log_resource_loader() {
//         const LOG_KEY: &str = "FluentLoader-get_locales";
//         dbg!("{}", locales().get_or_default(LOG_KEY));
//     }
// }
