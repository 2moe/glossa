use std::io::{self, Write};

use anyhow::bail;
use glossa_shared::{
  ToCompactString, fmt_compact,
  tap::{Pipe, Tap},
};
// use compact_str::{ToCompactString, fmt_compact};
use itertools::Itertools;
use lang_id::{LangID, RawID};

use crate::{
  AnyResult, MiniStr,
  generator::{Generator, MapType},
};

impl<'h> Generator<'h> {
  /// Generates a consolidated match function containing all localization
  /// mappings
  ///
  /// Generates:
  ///
  /// ```ignore
  /// const fn map(lang: &[u8], map_name: &[u8], key: &[u8])
  ///   -> &'static str {
  ///   match (lang, map_name, key) {...}
  /// }
  /// ```
  ///
  /// ## Parameter
  ///
  /// - `non_dsl`
  ///   - Specifies the type of mapping to process.
  ///   - Note: Does not support DSL MapType
  ///
  /// ## Example
  ///
  /// ```ignore
  /// use glossa_codegen::{L10nResources, Generator, generator::MapType};
  ///
  /// const L10N_DIR: &str = "../../locales/";
  ///
  /// let data = L10nResources::new(L10N_DIR)
  ///   .with_include_languages(["en-GB", "de", "es", "pt",
  /// "zh-pinyin"]);
  ///
  /// let function_data = Generator::default()
  ///   .with_resources(data)
  ///   .output_match_fn_all_in_one(MapType::Regular)?;
  ///
  /// assert_eq!(function_data.trim(), r#######"
  ///   pub(crate) const fn map(lang: &[u8], map_name: &[u8], key: &[u8]) ->
  ///   &'static str {
  ///     match (lang, map_name, key) {
  ///     (b"de", b"error", b"text-not-found") => r#####"Kein lokalisierter Text
  /// gefunden"#####,
  ///     (b"en-GB", b"error", b"text-not-found") => r#####"No
  /// localised text found"#####,
  ///     (b"es", b"error", b"text-not-found") =>
  /// r#####"No se encontró texto localizado"#####,
  ///     (b"pt", b"error",
  /// b"text-not-found") => r#####"Nenhum texto localizado encontrado"#####,
  ///     (b"zh-Latn-CN", b"error", b"text-not-found") => r#####"MeiYou ZhaoDao
  /// BenDiHua WenBen"#####,
  ///     _ => "",
  /// }}
  ///     "#######.trim());
  /// ```
  pub fn output_match_fn_all_in_one(
    &'h self,
    non_dsl: MapType,
  ) -> io::Result<String> {
    const S_HEADER: &str = r##"const fn map(lang: &[u8], map_name: &[u8], key: &[u8])
    -> &'static str {
    match (lang, map_name, key) {
    "##;

    let new_header = || self.new_match_fn_header(S_HEADER);

    non_dsl
      .get_non_dsl_maps(self)?
      .iter()
      // .filter(|(_, data)| !data.is_empty())
      .flat_map(|(lang, map_entry)| {
        map_entry
          .iter()
          .map(move |e| (lang, e))
      })
      .fold(
        new_header(), //
        |mut acc, (lang, ((name, key), value))| {
          // Build match arms for each localization entry
          [
            "(b\"",
            lang
              .to_compact_string()
              .as_str(),
            "\", ",
            key_as_bytes(name).as_str(),
            ", ",
            key_as_bytes(key).as_str(),
            ") => r#####",
            "\"",
            value.as_str(),
            "\"",
            "#####,",
            "\n",
          ]
          .map(|s| acc.push_str(s));
          acc
        },
      )
      .tap_mut(|buf| buf.push_str("    _ => \"\",\n}}"))
      .pipe(Ok)
  }

  /// Generates a function:
  ///   `const fn map(language: &[u8]) -> &'static str { match language {...} }`
  ///
  /// Note: This function is for performance optimization.
  /// **Only** invoke it to generate a new function when both `map_name` and
  /// `key` are guaranteed to be unique.
  /// Otherwise, use [`Self::output_match_fn_all_in_one`].
  pub fn output_match_fn_all_in_one_by_language(
    &'h self,
    non_dsl: MapType,
  ) -> io::Result<String> {
    const S_HEADER: &str = r##"const fn map(language: &[u8]) -> &'static str {
    match language {
    "##;

    let new_header = || self.new_match_fn_header(S_HEADER);

    non_dsl
      .get_non_dsl_maps(self)?
      .iter()
      .flat_map(|(lang, map_entry)| {
        map_entry
          .iter()
          .map(move |(_ks, v)| (lang, v))
      })
      .fold(
        new_header(), //
        |mut acc, (lang, value)| {
          [
            "b\"",
            lang
              .to_compact_string()
              .as_str(),
            "\" => r#####",
            "\"",
            value.as_str(),
            "\"",
            "#####,",
            "\n",
          ]
          .map(|s| acc.push_str(s));
          acc
        },
      )
      .tap_mut(|buf| buf.push_str("    _ => \"\",\n}}"))
      .pipe(Ok)
  }

  /// Generates a function:
  ///   `const fn map(language: &[u8], key: &[u8]) -> &'static str { match
  /// (language, key) {...} }`
  ///
  /// # Note
  ///
  /// You can invoke this function to generate a new function **only** when
  /// `map_name` is unique.
  ///
  /// **Example**:
  ///
  /// - `en/yes-no { yes: "Yes", no: "No"}`
  /// - `de/yes-no { yes: "Ja", no: "Nein" }`
  ///
  /// Here, `map_name` is unique (per language), so it can be omitted:
  ///
  /// ```ignore
  /// match (language, key) {
  ///   (b"en", b"yes") => r#####"Yes"#####,
  ///   (b"en", b"no") => r#####"No"#####,
  ///   (b"de", b"yes") => r#####"Ja"#####,
  ///   (b"de", b"no") => r#####"Nein"#####,
  /// }
  /// ```
  ///
  /// If `map_names` are not unique, use [`Self::output_match_fn_all_in_one`]
  /// instead.
  ///
  /// For example, adding a new map: `en/yes-no2 { yes: "YES", no: "NO"}`
  /// would create conflicting keys ("yes", "no") if `map_name` is omitted.
  pub fn output_match_fn_all_in_one_by_language_and_key(
    &'h self,
    non_dsl: MapType,
  ) -> io::Result<String> {
    const S_HEADER: &str = r##"const fn map(language: &[u8], key: &[u8])
    -> &'static str {
    match (language, key) {
    "##;

    let new_header = || self.new_match_fn_header(S_HEADER);

    non_dsl
      .get_non_dsl_maps(self)?
      .iter()
      .flat_map(|(lang, map_entry)| {
        map_entry
          .iter()
          .map(move |((_name, key), value)| (lang, key, value))
      })
      .fold(
        new_header(), //
        |mut acc, (lang, key, value)| {
          [
            "(b\"",
            lang
              .to_compact_string()
              .as_str(),
            "\", ",
            key_as_bytes(key).as_str(),
            ") => r#####",
            "\"",
            value.as_str(),
            "\"",
            "#####,",
            "\n",
          ]
          .map(|s| acc.push_str(s));
          acc
        },
      )
      .tap_mut(|buf| buf.push_str("    _ => \"\",\n}}"))
      .pipe(Ok)
  }

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
    &'h self,
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

  fn collect_raw_locales(&'h self, map_type: MapType) -> io::Result<Vec<MiniStr>> {
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
  fn new_locales_fn_header(
    &'h self,
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
    self.new_match_fn_header(&s_header)
  }

  /// Creates header for generated match functions
  fn new_match_fn_header(&'h self, header: &str) -> String {
    let vis_fn = self.get_visibility().as_str();
    String::with_capacity(8192).tap_mut(|buf| {
      [vis_fn, " ", header].map(|s| buf.push_str(s));
    })
  }

  /// Generates individual match functions per locale
  pub fn output_match_fn(&'h self, non_dsl: MapType) -> io::Result<()> {
    const HEADER: &str = r##"const fn map(map_name: &[u8], key: &[u8]) -> &'static str {
    match (map_name, key) {
    "##;
    let new_header = || self.new_match_fn_header(HEADER);

    // Process non-DSL maps only (DSL MapType not supported)
    non_dsl
      .get_non_dsl_maps(self)?
      .iter()
      // .filter(|(_, data)| !data.is_empty())
      .map(|(lang, map_entry)| {
        let match_fn_string = map_entry
          .iter()
          .fold(
            new_header(), //
            |mut acc, ((map_name, data_k), data_v)| {
              // Build match arms for each entry
              [
                "(",
                key_as_bytes(map_name).as_str(),
                ", ",
                key_as_bytes(data_k).as_str(),
                r#") => r#####"#,
                "\"", // "
                data_v.as_str(),
                "\"",                            // "
                r###########"#####,"###########, // ###,
                "\n",
              ]
              .map(|s| acc.push_str(s));
              acc
            },
          )
          .tap_mut(|buf| buf.push_str("    _ => \"\",\n}}"));
        (lang, match_fn_string)
      })
      .try_for_each(|(lang, s)| {
        // Write generated content to module files
        self
          .create_rs_mod_file(lang)?
          .write_all(s.as_bytes())
      })
  }
}

fn try_conv_const_id(id: &MiniStr) -> Result<MiniStr, anyhow::Error> {
  use lang_id::matches::{get_fn_name, match_id};

  match match_id(id.as_bytes())
    .to_compact_string()
    .as_str()
  {
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

/// Helper function to format keys as byte string literals
fn key_as_bytes(key: &str) -> MiniStr {
  use fmt_compact as fmt;

  match key.is_ascii() {
    true => fmt!("b{key:?}"),
    _ => fmt!("{:?}", key.as_bytes()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    AnyResult,
    generator::dbg_generator::{en_generator, new_generator},
  };

  #[ignore]
  #[test]
  fn test_output_match_fn() -> AnyResult<()> {
    new_generator().output_match_fn(MapType::Regular)?;
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_output_aio_match_fn() -> AnyResult<()> {
    en_generator()
      .output_match_fn_all_in_one(MapType::Regular)?
      .tap(|s| println!("{s}"));
    Ok(())
  }

  const fn map(language: &[u8], map_name: &[u8], key: &[u8]) -> &'static str {
    match (language, map_name, key) {
      (b"de", b"error", b"text-not-found") => {
        r###"Kein lokalisierter Text gefunden"###
      }
      (b"el", b"error", b"text-not-found") => {
        r###"Δεν βρέθηκε κανένα τοπικό κείμενο"###
      }
      (b"en", b"error", b"text-not-found") => r###"No localized text found"###,
      (b"en", b"test", [240, 159, 145, 139, 240, 159, 140, 144]) => {
        r###"hello world"###
      }
      (b"en", b"test", b"hello") => r###"world"###,
      (b"en-GB", b"error", b"text-not-found") => r###"No localised text found"###,
      (b"zh", b"error", b"text-not-found") => r###"未找到本地化文本"###,
      (b"zh", b"test", b"quote") => r###"""no"''""###,
      (b"zh-Hant", b"error", b"text-not-found") => r###"沒有找到本地化文本"###,
      _ => "",
    }
  }

  #[ignore]
  #[test]
  fn test_get_match_map() {
    const S: &str = map(b"de", b"error", "text-not-found".as_bytes());
    assert!(!S.is_empty());
    println!("{S}");
  }

  #[ignore]
  #[test]
  fn test_show_all_locales() -> AnyResult<()> {
    let s = new_generator()
      .with_visibility(crate::Visibility::Pub)
      .output_locales_fn(
        MapType::Regular, //
        // false,
        true,
      )?;
    println!("{s}");

    Ok(())
  }

  #[ignore]
  #[test]
  fn doc_test_all_in_one_match_fn() -> io::Result<()> {
    use crate::L10nResources;
    const L10N_DIR: &str = "../../locales/";

    let data = L10nResources::new(L10N_DIR).with_include_languages([
      "en-GB",
      "de",
      "es",
      "pt",
      "zh-pinyin",
    ]);

    let function_data = Generator::default()
      .with_resources(data)
      .output_match_fn_all_in_one(MapType::Regular)?;

    assert_eq!(function_data, r#######"
    pub(crate) const fn map(lang: &[u8], map_name: &[u8], key: &[u8]) ->
  &'static str {
    match (lang, map_name, key) {
    (b"de", b"error", b"text-not-found") => r#####"Kein lokalisierter Text gefunden"#####,
(b"en-GB", b"error", b"text-not-found") => r#####"No localised text found"#####,
(b"es", b"error", b"text-not-found") => r#####"No se encontró texto localizado"#####,
(b"pt", b"error", b"text-not-found") => r#####"Nenhum texto localizado encontrado"#####,
(b"zh-Latn-CN", b"error", b"text-not-found") => r#####"MeiYou ZhaoDao BenDiHua WenBen"#####,
    _ => "",
}}
    "#######.trim());

    println!("{function_data}");
    Ok(())
  }
}
