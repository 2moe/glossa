// #![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

mod parser;

pub(crate) mod l10n;
pub use l10n::locale_registry::all_locales;

pub mod options;
pub use options::Cli;

pub mod envs;

pub mod static_data;

pub mod context;
