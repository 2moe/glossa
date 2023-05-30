use crate::highlight::syntax::static_syntax_set;
use getset::{Getters, MutGetters};
use once_cell::sync::OnceCell;
use std::{borrow::Cow, collections::HashMap, ffi::OsStr};
pub use syntect::{dumps, highlighting::ThemeSet};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
pub mod output;
mod syntax;
pub mod theme;

#[derive(Getters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLight<'res, 'p> {
    resource: HighLightRes<'res>,
    files: HashMap<Cow<'res, OsStr>, HighLightFmt<'p>>,
}

impl<'res, 'p> HighLight<'res, 'p> {
    pub fn new(
        resource: HighLightRes<'res>,
        files: HashMap<Cow<'res, OsStr>, HighLightFmt<'p>>,
    ) -> Self {
        Self { resource, files }
    }
}

#[derive(Getters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLightRes<'name> {
    name: Cow<'name, str>,
    theme: OnceCell<Theme>,
    syntax_set: &'static SyntaxSet,
    theme_set: &'name ThemeSet,
    background: bool,
}

#[derive(Getters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLightFmt<'f> {
    syntax: Cow<'f, str>,
    suffix: Option<Cow<'f, str>>,
    // key: suffix, value: (theme_name, background)
    extra: Option<HashMap<&'f str, (&'f str, bool)>>,
}

impl<'p> Default for HighLightFmt<'p> {
    fn default() -> Self {
        Self {
            syntax: Cow::from("markdown"),
            suffix: Some(Cow::from("_md")),
            extra: None,
        }
    }
}

impl<'p> HighLightFmt<'p> {
    pub fn new(fmt: Cow<'p, str>, prefix: Option<Cow<'p, str>>) -> Self {
        Self {
            syntax: fmt,
            suffix: prefix,
            ..Default::default()
        }
    }
}

impl<'name> HighLightRes<'name> {
    pub fn new(name: Cow<'name, str>, theme_set: &'name ThemeSet) -> Self {
        Self {
            name,
            theme_set,
            syntax_set: static_syntax_set(),
            ..Default::default()
        }
    }

    /// Enable or disable background
    pub fn with_background(self, switch: bool) -> Self {
        Self {
            background: switch,
            ..self
        }
    }
}

impl<'name> Default for HighLightRes<'name> {
    fn default() -> Self {
        Self {
            name: Cow::from(Self::monokai_theme_name()),
            theme: OnceCell::new(),
            syntax_set: static_syntax_set(),
            theme_set: Self::static_theme_set(),
            background: true,
        }
    }
}
