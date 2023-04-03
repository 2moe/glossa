//! # glossa-codegen
//!
//! Use a code generator to generate code.
//!
//! It can generate const language localisation map for code at compile time
//!
//! ## features
//!
//! - yaml
//!   - Enabled by default.
//!   - The default file extension is ".yaml" or ".yml"
//! - ron
//!   - The default ext is ".ron"
//! - toml
//!   - The ext is ".toml"
//! - json
//!   - ext: ".json"
//!
//! This corresponds to different types of configuration files. You can enable all features or add them as needed.
//!
//! By default, the file type is determined based on the file name suffix, and the **map name** (table name) is set based on the file name. Whether deserialisation is needed at compile-time is determined by the enabled feature.
//!
//! ## Preparations
//!
//! Before writing `build.rs`, we need to prepare the localisation resource files (localised files).
//!
//! de (Deutsch, Lateinisch, Deutschland):
//!
//! - "assets/l10n/de/error.yaml"
//!
//! ```yaml
//! text-not-found: Kein lokalisierter Text gefunden
//! ```
//!
//! en (English, Latin, United States):
//!
//! - "assets/l10n/en/error.yaml"
//!
//! ```yaml
//! text-not-found: No localized text found
//! ```
//!
//! en-GB (English, Latin, Great Britain):
//!
//! - `assets/l10n/en-GB/error.yaml`
//!
//! ```yaml
//! text-not-found: No localised text found
//! ```
//!
//! es (español, latino, España):
//!
//! - `assets/l10n/es/error.yaml`
//!
//! ```yaml
//! text-not-found: No se encontró texto localizado
//! ```
//!
//! pt (português, latim, Brasil)
//!
//! - `assets/l10n/pt/error.yaml`
//!
//! ```yaml
//! text-not-found: Nenhum texto localizado encontrado
//! ```
//!
//! ---
//!
//! `build.rs`：
//!
//! ```rust
//! use glossa_codegen::{consts::*, prelude::*};
//! use std::{
//!     fs::File,
//!     io::{self, BufWriter},
//!     path::PathBuf,
//! };
//!
//! fn main() -> io::Result<()> {
//!     // Specify the version as the current package version to avoid repetitive compilation for the same version.
//!     let version = Some(get_pkg_version!());
//!     // During development, we can set it to None.
//!     // let version = None;
//!
//!     // This is a constant array: ["src", "assets", "localisation.rs"], which is converted into a path for storing automatically generated Rust code related to localisation.
//!     // On Windows, the path is 'src\assets\localisation.rs'.
//!     // On Unix, the path is "src/assets/localisation.rs".
//!     // Note: this is a relative path!
//!     let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());
//!
//!     // If it's the same version, then exit.
//!     if is_same_version(&path, version)? {
//!         return Ok(());
//!     }
//!
//!     // If the path is "src/assets/localisation.rs", then it will append `mod localisation;` and related `use` statements to "src/assets/mod.rs".
//!     append_to_l10n_mod(&path)?;
//!
//!     // This creates a new file: "src/assets/localisation.rs".
//!     // Unlike append, if only create is used, the file will be cleared.
//!     let mut file = BufWriter::new(File::create(&path)?);
//!
//!     // default_l10n_dir_arr() is also a constant array: ["assets", "l10n"].
//!     // If the current localisation resource path is at the parent level, then you can use `path = PathBuf::from_iter([".."].into_iter().chain(default_l10n_dir_arr()));`.
//!     path = PathBuf::from_iter(default_l10n_dir_arr());
//!
//!     // Here, the l10n file is deserialised into a map and written to the rs file.
//!     // file: "src/assets/localisation.rs"
//!     // path: "assets/l10n"
//!     // visibility: Used to set the visibility of the generated `fn`. If it is None, then Some("pub(crate)") is used. You can use `Some("pub(in path)")` or `Some("pub")`
//!     deser_cfg_to_map(&mut file, &mut path, Some("pub(crate)"), version)
//! }
//! ```

pub mod consts;
pub mod generator;
pub mod prelude;
mod version;
