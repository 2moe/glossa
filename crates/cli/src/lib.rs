// #![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

pub mod options;
mod parser;

pub(crate) mod l10n;

pub use options::Cli;
