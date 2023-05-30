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
//! - highlight
//!
//! In addition to highlight, this corresponds to different types of configuration. You can enable all features or add them as needed.
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
//! ## build.rs
//!
//! ```rust
//! use glossa_codegen::{consts::*, prelude::*};
//! use std::{io, path::PathBuf};
//!
//! fn main() -> io::Result<()> {
//!     // Specify the version as the current package version to avoid repetitive compilation for the same version.
//!     let ver = get_pkg_version!();
//!
//!     // This is a constant array: ["src", "assets", "localisation.rs"], which is converted into a path for storing automatically generated Rust code related to localisation.
//!     // path: "src/assets/localisation.rs".
//!     let rs_path = PathBuf::from_iter(default_l10n_rs_file_arr());
//!
//!     // If it's the same version, then exit.
//!     if is_same_version(&rs_path, Some(ver))? {
//!         // When developing, we can comment out the `return` statement below so that every change will be recompiled and won't exit prematurely.
//!         return Ok(());
//!     }
//!
//!     // If the path is "src/assets/localisation.rs", then it will append `mod localisation;` and related `use` statements to "src/assets/mod.rs".
//!     append_to_l10n_mod(&rs_path)?;
//!
//!     // A new file will be created here:
//!     //    - Linux(non android): "/dev/shm/localisation.tmp"
//!     //    - Other："src/assets/localisation.tmp"
//!     // After the code generation is complete, rename(move) the file: "/dev/shm/localisation.tmp" -> "src/assets/localisation.rs".
//!     // Note: If written directly to `rs_path` and `cargo` is interrupted during building, it may result in incomplete generated code. Therefore, `tmp_path` is used as a temporary buffer file.
//!     let tmp_path = get_shmem_path(&rs_path)?;
//!     let writer = MapWriter::new(&tmp_path, &rs_path);
//!
//!     // default_l10n_dir_arr() is also a constant array: ["assets", "l10n"].
//!     // If the current localisation resource path is at the parent level, then you can use `path = PathBuf::from_iter([".."].into_iter().chain(default_l10n_dir_arr()));`.
//!     let l10n_path = PathBuf::from_iter(default_l10n_dir_arr());
//!
//!     let generator = Generator::new(l10n_path).with_version(ver);
//!     // Invoke the generator here to generate code.
//!     generator.run(writer)
//! }
//! ```

pub mod consts;
mod generator;

#[cfg(feature = "highlight")]
pub mod highlight;

pub(crate) mod conversion;
mod create;
pub(crate) mod deser;
pub(crate) mod header;
mod map_writer;
pub mod prelude;
mod version;
