use std::io::{self, Write};

use glossa_shared::{
  ToCompactString, fmt_compact,
  tap::{Pipe, Tap},
};

use crate::{
  MiniStr,
  generator::{Generator, MapType},
};

impl<'h> Generator<'h> {
  /// Generates individual match functions per locale
  ///   => `const fn map(map_name: &[u8], key: &[u8]) -> &'static str`
  pub fn output_match_fn(&self, non_dsl: MapType) -> io::Result<()> {
    const HEADER: &str = r##"const fn map(map_name: &[u8], key: &[u8]) -> &'static str {
    match (map_name, key) {
    "##;
    let new_header = || self.new_fn_header(HEADER);

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

    let new_header = || self.new_fn_header(S_HEADER);

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

    let new_header = || self.new_fn_header(S_HEADER);

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
  pub fn output_match_fn_all_in_one_without_map_name(
    &'h self,
    non_dsl: MapType,
  ) -> io::Result<String> {
    const S_HEADER: &str = r##"const fn map(language: &[u8], key: &[u8])
    -> &'static str {
    match (language, key) {
    "##;

    let new_header = || self.new_fn_header(S_HEADER);

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

  /// Creates header for generated functions
  pub fn new_fn_header(&self, header: &str) -> String {
    let vis_fn = self.get_visibility().as_str();
    String::with_capacity(8192).tap_mut(|buf| {
      [vis_fn, " ", header].map(|s| buf.push_str(s));
    })
  }

  /// Generates individual match functions per locale
  /// => `const fn map(key: &[u8]) -> &'static str`
  ///
  /// Compared to `output_match_fn`, omits map_name. If you're unsure which one
  /// to use, then use [output_match_fn()](Self::output_match_fn)
  pub fn output_match_fn_without_map_name(
    &'h self,
    non_dsl: MapType,
  ) -> io::Result<()> {
    const HEADER: &str = r##"const fn map(key: &[u8]) -> &'static str {
    match key {
    "##;
    let new_header = || self.new_fn_header(HEADER);

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
            |mut acc, ((_, data_k), data_v)| {
              // Build match arms for each entry
              [
                key_as_bytes(data_k).as_str(),
                r#" => r#####"#,
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

/// Helper function to format keys as byte string literals
pub(crate) fn key_as_bytes(key: &str) -> MiniStr {
  use fmt_compact as fmt;

  match key.is_ascii() {
    true => fmt!("b{key:?}"),
    _ => fmt!("{:?}", key.as_bytes()),
  }
}

#[cfg(test)]
mod tests {
  use glossa_shared::display::puts;

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
      .pipe_ref(puts)
      .pipe(Ok)
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
