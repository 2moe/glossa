use std::io;

use anyhow::bail;
use glossa_shared::{
  ToCompactString, fmt_compact,
  tap::{Pipe, Tap},
};
use itertools::Itertools;
use lang_id::{LangID, RawID};

use crate::{
  AnyResult, Generator, MiniStr,
  generator::{MapType, to_lower_snake_case},
};

impl Generator<'_> {
  /// Generates a function listing all available locales
  ///
  /// ## Parameters
  ///
  /// - `map_type`
  ///   - Mapping type to process
  /// - `const_lang_id`
  ///   - Whether to generate lang_id constants
  ///
  /// ## Example
  ///
  /// ```ignore
  /// let function_data = new_generator()
  ///   .with_visibility(crate::Visibility::Pub)
  ///   .output_locales_fn(
  ///     MapType::Regular,
  ///     false,
  /// )?;
  /// ```
  ///
  /// function_data:
  ///
  /// ```ignore
  /// pub const fn all_locales() -> [lang_id::LangID; 107] {
  /// use lang_id::consts::*;
  /// [
  ///   lang_id_af(),
  ///   lang_id_am(),
  ///   lang_id_ar(),
  ///   lang_id_az(),
  ///   lang_id_be(),
  ///   lang_id_bg(),
  ///   lang_id_bn(),
  ///   lang_id_bs(),
  ///   lang_id_ca(),
  ///   lang_id_ceb(),
  ///   lang_id_co(),
  ///   ...
  /// ]}
  /// ```
  pub fn output_locales_fn(
    &self,
    map_type: MapType,
    const_lang_id: bool,
  ) -> AnyResult<String> {
    let raw_locales = self.collect_raw_locales(map_type)?;
    let locales_len = raw_locales.len();
    let new_header = || self.new_locales_fn_header(&locales_len, &const_lang_id);

    if !const_lang_id {
      return new_header()
        .tap_mut(|buf| {
          let push_str = |s| buf.push_str(s);
          [&format!("{raw_locales:#?}"), "}\n"].map(push_str);
        })
        .pipe(Ok);
    }

    // Process constant lang IDs
    raw_locales
      .iter()
      .map(try_conv_const_id)
      .try_fold(
        new_header(), //
        |mut acc, fn_name| {
          let push_str = |s| acc.push_str(s);
          ["\n    ", &fn_name?, ","].map(push_str);
          Ok::<_, anyhow::Error>(acc)
        },
      )?
      .tap_mut(|buf| buf.push_str("  ]\n}"))
      .pipe(Ok)
  }

  /// -> e.g., `["en", "en-GB", "zh-Hant", "zh-Latn-CN"]`
  pub fn collect_raw_locales(&self, map_type: MapType) -> io::Result<Vec<MiniStr>> {
    match map_type.is_dsl() {
      true => match self.get_or_init_dsl_maps() {
        x if x.is_empty() => "// Error: Empty DSL Map"
          .pipe(io::Error::other)
          .pipe(Err),
        data => data
          .iter()
          .map(|(id, _)| id.to_compact_string())
          .collect_vec()
          .pipe(Ok),
      },
      _ => map_type
        .get_non_dsl_maps(self)?
        .iter()
        .map(|(id, _)| id.to_compact_string())
        .collect_vec()
        .pipe(Ok),
    }
  }

  pub(crate) fn new_locales_fn_header(
    &self,
    locales_len: &usize,
    const_lang_id: &bool,
    // this: &Generator<'_, 'h>,
  ) -> String {
    // Generate appropriate header based on const_lang_id flag
    let ret_type = {
      match *const_lang_id {
        true => fmt_compact!(
          r#"[super::lang_id::LangID; {locales_len}] {{
  #[allow(unused_imports)]
  use super::lang_id::RawID;
  use super::lang_id::consts::*;
  ["#
        ),
        _ => fmt_compact!("[&'static str; {locales_len}] {{\n  "),
      }
    };

    let s_header = format!("const fn all_locales() -> {ret_type}",);
    self.new_fn_header(&s_header)
  }

  /// Generates `cfg` features for different languages and mod file names.
  ///
  /// Output Sample:
  ///
  /// ```ignore
  /// #[cfg(feature = "l10n-en-GB")]
  /// mod l10n_en_gb;
  ///
  /// #[cfg(feature = "l10n-es")]
  /// mod l10n_es;
  /// ```
  ///
  /// You can use this in `mod.rs`.
  ///
  /// > Note: If you use `output_*_all_in_one`, you do not need to call this
  /// > method.
  ///
  /// - The feature prefix depends on [`Self::feature_prefix`]
  /// - The mod prefix name depends on [`Self::mod_prefix`]
  pub fn output_mod_rs(&self, map_type: MapType) -> io::Result<String> {
    let mod_vis = self.get_mod_visibility();

    let feature_names = self.collect_cargo_feature_names(map_type)?;
    let module_names = self.collect_rs_mod_names(map_type)?;
    ensure_length_equal(feature_names.len(), module_names.len())?;

    feature_names
      .iter()
      .zip(module_names)
      .map(|(feat_name, mod_name)| {
        fmt_compact!(
          r###"
#[cfg(feature = "{feat_name}")]
{mod_vis}mod {mod_name};
        "###
        )
      })
      .collect::<String>()
      .pipe(Ok)
  }

  /// -> e.g., `["l10n-en", "l10n-en-GB", "l10n-zh", "l10n-zh-Hant"]`
  pub fn collect_cargo_feature_names(
    &self,
    map_type: MapType,
  ) -> io::Result<Vec<MiniStr>> {
    let feat_prefix = self.get_feature_prefix();

    self
      .collect_raw_locales(map_type)?
      .into_iter()
      .map(|x| fmt_compact!("{feat_prefix}{x}"))
      .collect_vec()
      .pipe(Ok)
  }

  /// -> e.g., `["l10n_en", "l10n_en_gb", "l10n_zh"]`
  pub fn collect_rs_mod_names(&self, map_type: MapType) -> io::Result<Vec<MiniStr>> {
    let mod_prefix = self.get_mod_prefix();

    self
      .collect_raw_locales(map_type)?
      .into_iter()
      .map(|x| fmt_compact!("{mod_prefix}{id}", id = to_lower_snake_case(x)))
      .collect_vec()
      .pipe(Ok)
  }

  /// Generates Cargo features for different locales.
  ///
  /// Output Sample:
  ///
  /// ```toml,ignore
  /// l10n-en = []
  /// l10n-zh = []
  /// l10n-all = ["l10n-en","l10n-zh"]
  /// ```
  #[cfg(feature = "json")]
  pub fn output_cargo_features(&self, map_type: MapType) -> io::Result<String> {
    let feat_prefix = self.get_feature_prefix();
    let all_feat_locales = self.collect_cargo_feature_names(map_type)?;

    // If all elements in the array are of the same type, then TOML and JSON are the
    // same.
    let all_arr_json = serde_json::to_string(&all_feat_locales)?;

    format!(
      "{} = []\n{feat_prefix}all = {all_arr_json}",
      all_feat_locales.join(" = []\n")
    )
    .pipe(Ok)
  }
}

/// Validates whether the input language identifier `id` meets specific format
/// requirements and attempts to convert it into an identifier (function name).
///
/// > convert: lang_id::matches::get_fn_name(id) => function name
///
/// If the input identifier does not meet the requirements, it
/// attempts to construct a constant LangID using RawID.
fn try_conv_const_id(id: &MiniStr) -> AnyResult<MiniStr> {
  use lang_id::matches::{get_fn_name, match_id};

  match match_id(id.as_bytes())
    .to_compact_string()
    .as_str()
  {
    // Since multiple keys in match_id may correspond to one value, a check is
    // required here.
    x if x == id => id
      .as_bytes()
      .pipe(get_fn_name)
      .to_compact_string()
      .pipe(Ok),
    _ => {
      let id = id.parse::<LangID>()?;
      if id.variants().count() >= 1 {
        bail!("This ID ({id}) contains variants and cannot be converted to const.")
      }
      RawID::try_from_str(
        id.language.as_str(),
        id.script
          .map(|x| x.to_compact_string())
          .unwrap_or_default()
          .as_str(),
        id.region
          .map(|x| x.to_compact_string())
          .unwrap_or_default()
          .as_str(),
      )?
      .to_compact_string()
      .pipe(Ok)
    }
  }
}
pub(crate) fn ensure_length_equal(
  feat_names_len: usize,
  mod_names_len: usize,
) -> io::Result<()> {
  (mod_names_len == feat_names_len)
    .then_some(())
    .ok_or_else(|| {
      io::Error::other("feature_names and module_names are not equal in length")
    })
}

#[cfg(test)]
mod tests {
  use glossa_shared::display::puts;

  use super::*;
  use crate::generator::dbg_generator::new_generator;

  #[ignore]
  #[test]
  fn test_list_all_locales() -> AnyResult<()> {
    new_generator()
      .with_visibility(crate::Visibility::Pub)
      .output_locales_fn(
        MapType::Regular,
        // false,
        true,
      )?
      .pipe_ref(puts)
      .pipe(Ok)
  }

  #[ignore]
  #[test]
  fn test_gen_mod_rs_str() -> AnyResult<()> {
    let generator = new_generator().with_mod_visibility(crate::Visibility::Pub);

    let raw_locales = generator.collect_raw_locales(MapType::Regular)?;
    let mod_prefix = generator.get_mod_prefix();
    let feature_prefix = generator.get_feature_prefix();
    let mod_vis = generator.get_mod_visibility();

    let fn_content = raw_locales
      .iter()
      .map(|id| {
        let mod_name = to_lower_snake_case(id);
        fmt_compact!(
          r###"
#[cfg(feature = "{feature_prefix}{id}")]
{mod_vis}mod {mod_prefix}{mod_name};
        "###
        )
      })
      .collect::<String>();

    println!("{fn_content}");
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_output_mod_rs() -> AnyResult<()> {
    new_generator()
      .with_mod_visibility(crate::Visibility::PubCrate)
      .output_mod_rs(MapType::Regular)?
      .pipe_ref(puts)
      .pipe(Ok)
  }

  #[test]
  #[ignore]
  #[cfg(feature = "json")]
  fn test_gen_cargo_features() -> AnyResult<()> {
    use glossa_shared::display::puts;

    let generator = new_generator();
    let feat_prefix = generator.get_feature_prefix();

    let all_locales = generator
      .collect_raw_locales(MapType::Regular)?
      .into_iter()
      .map(|x| fmt_compact!("{feat_prefix}{x}"))
      .collect_vec();

    let all = serde_json::to_string(&all_locales)?;

    format!(
      "{} = []\n{feat_prefix}all = {all}",
      all_locales.join(" = []\n")
    )
    .pipe_ref(puts)
    .pipe(Ok)
  }

  #[ignore]
  #[test]
  fn test_output_cargo_features() -> AnyResult<()> {
    new_generator()
      .output_cargo_features(MapType::DSL)?
      .pipe_ref(puts)
      .pipe(Ok)
  }
}
