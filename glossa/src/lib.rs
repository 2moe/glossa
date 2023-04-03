//! # Glossa
//!
//! Glossa is a language localisation library.
//!
//! ## Features
//!
//! - log: Enables logging of messages and errors.
//! - fluent: Used to manage localised resources at runtime.
//! - ahash: When enabled, it uses [ahash::HashMap](::ahash) by default, not [std::collections::HashMap](::std::collections::HashMap).
//!
//! ## Functionality
//!
//! By functional type, it can be divided into two categories.
//!
//! - Compile-time: Converts the configuration file into constant (`const fn`) Rust code to achieve efficient localisation.
//!   - Pros: High efficiency
//!   - Cons:
//!     - Requires `codegen`, which may result in some redundant code expansion.
//!     - Currently only supports simple key-value (K-V) pairs.
//! - Runtime: Manages `fluent` resources.
//!   - Pros: Fluent syntax may be more suitable for localisation.
//!   - Cons: Occupies more resources than `const fn`.
//!
//! The former is just the simple K-V pair that uses some const maps from phf to store data. Because it's simple, it's efficient.
//!
//! The two types of functionalities are independent of each other.
//!
//! There are some functions that need to be used with `glossa-codegen`, which will not be described in detail here.
//! If you want to find out more, go to the git repository for this project.
//!
//! ## Example
//!
//! In fact, you should use your own localisation resources rather than glossa's built-in `locales()`.
//!
//! It displays different content for different languages. If no relevant text is found, then it will automatically use the fallback chain.
//!
//! ```rust
//!     use glossa::{locales, GetText};
//!     let msg = locales().get_or_default("error", "text-not-found");
//!     println!("{msg}");
//!     let hello = locales()
//!         .get("test", "👋🌐")
//!         .expect("No localised text found. (map: test, key: 👋🌐)");
//!     println!("{hello}");
//! ```
//!

pub mod error;

/// The default error type is `GlossaError<'map>`
pub type Result<'map, T> = ::core::result::Result<T, error::GlossaError<'map>>;

pub(crate) mod log;

#[cfg(feature = "fluent")]
pub mod fluent;
#[cfg(feature = "fluent")]
pub use fluent::LangResource as LangRes;

pub mod assets;
mod l10n;

mod map_loader;
/// Get text from localised resources.
pub use map_loader::get::GetText;

/// Load the localised resources of the HashMap.
pub use map_loader::MapLoader;

/// Language Identifier
pub use lang_id::LangID;

/// Contains the FallbackChain Trait implementation
pub mod fallback;

pub use l10n::locales;