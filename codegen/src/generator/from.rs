use crate::generator::Generator;
use std::{borrow::Cow, path::PathBuf};

impl<'ver, 'p, 'res> Generator<'ver, 'p, 'res> {
    pub fn new(l10n_path: PathBuf) -> Self {
        Self {
            l10n_path,
            version: Cow::from("none"),
            // ..Default::default()
            highlight: None,
        }
    }

    pub fn with_version(self, ver: &'ver str) -> Self {
        Self {
            version: Cow::from(ver),
            ..self
        }
    }

    pub fn with_opt_version(self, ver: Option<&'ver str>) -> Self {
        match ver {
            Some(v) => self.with_version(v),
            _ => self,
        }
    }
}
