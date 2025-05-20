use glossa_codegen::L10nResources;
use log::debug;

use crate::options::ResourcesOpt;

impl ResourcesOpt {
  pub(crate) fn update_dsl_suffix(&self, res: L10nResources) -> L10nResources {
    match self.get_dsl_suffix() {
      Some(suffix) => {
        debug!("DSL suffix: {suffix}");
        res.with_dsl_suffix(suffix.clone())
      }
      _ => res,
    }
  }
  pub(crate) fn update_include_languages(
    &self,
    res: L10nResources,
  ) -> L10nResources {
    match self
      .get_include_languages()
      .as_slice()
    {
      [] => res,
      langs => {
        debug!("Including languages: {langs:?}");
        res.with_include_languages(langs.iter().cloned())
      }
    }
  }
  pub(crate) fn update_exclude_languages(
    &self,
    res: L10nResources,
  ) -> L10nResources {
    match self
      .get_exclude_languages()
      .as_slice()
    {
      [] => res,
      langs => {
        debug!("Excluding languages: {langs:?}");
        res.with_exclude_languages(langs.iter().cloned())
      }
    }
  }

  pub(crate) fn update_include_map_names(
    &self,
    res: L10nResources,
  ) -> L10nResources {
    match self
      .get_include_map_names()
      .as_slice()
    {
      [] => res,
      names => {
        debug!("Including map names: {names:?}");
        res.with_include_map_names(names.iter().cloned())
      }
    }
  }
  pub(crate) fn update_exclude_map_names(
    &self,
    res: L10nResources,
  ) -> L10nResources {
    match self
      .get_exclude_map_names()
      .as_slice()
    {
      [] => res,
      names => {
        debug!("Excluding map names: {names:?}");
        res.with_exclude_map_names(names.iter().cloned())
      }
    }
  }
}
