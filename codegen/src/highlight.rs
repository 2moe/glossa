use getset::{Getters, MutGetters};
pub use hlight::HighLightRes;
use std::{borrow::Cow, collections::HashMap, ffi::OsStr};

#[derive(Getters, MutGetters, Debug)]
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
