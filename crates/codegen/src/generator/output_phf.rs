use std::{
  fs::File,
  io::{self, BufWriter, Write},
};

use compact_str::{ToCompactString, format_compact};
use glossa_shared::{PhfTupleKey, phf_triple_key::RawTripleKey};
use phf_codegen::OrderedMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tap::{Pipe, Tap};

use crate::{
  MiniStr,
  generator::{
    Generator, MapType, flattening::L10nBTreeMap, output_bincode::create_buf_writer,
  },
};

impl<'h> Generator<'_, 'h> {
  /// Collect all localized resources into a **`const phf::OrderedMap`**
  /// function, i.e., a single table can accommodate different `language`,
  /// `map_name`, and `map_key`.
  ///
  ///
  /// ## Example
  ///
  /// ```no_run
  /// use glossa_codegen::{L10nResources, Generator, generator::MapType};
  ///
  /// let data = L10nResources::new("../../locales/")
  ///   .with_include_map_names(&["error"])
  ///   .with_include_languages(&[
  ///     "de",
  ///     "zh-pinyin",
  ///     "zh",
  ///     "pt",
  ///     "es",
  ///     "en",
  ///     "en-GB",
  /// ]);
  ///
  /// let function_data = Generator::default()
  ///   .with_resources(data)
  ///   .output_match_fn_all_in_one(MapType::Regular)?;
  ///
  /// # Ok::<(), std::io::Error>(())
  /// ```
  ///
  /// ### function data:
  ///
  /// ```ignore
  /// // glossa_shared::{phf, PhfL10nAllInOneMap, PhfTripleKey};
  ///
  /// pub(crate) const fn map() -> super::PhfL10nAllInOneMap {
  ///     use super::PhfTripleKey as Key;
  ///     super::phf::OrderedMap {
  ///       key: 12913932095322966823,
  ///       disps: &[(2, 3), (2, 0)],
  ///       idxs: &[5, 4, 0, 6, 3, 2, 1],
  ///       entries: &[
  ///         (
  ///           Key(r#"de"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"Kein lokalisierter Text gefunden"#####,
  ///         ),
  ///         (
  ///           Key(r#"en"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"No localized text found"#####,
  ///         ),
  ///         (
  ///           Key(r#"en-GB"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"No localised text found"#####,
  ///         ),
  ///         (
  ///           Key(r#"es"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"No se encontró texto localizado"#####,
  ///         ),
  ///         (
  ///           Key(r#"pt"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"Nenhum texto localizado encontrado"#####,
  ///         ),
  ///         (
  ///           Key(r#"zh"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"未找到本地化文本"#####,
  ///         ),
  ///         (
  ///           Key(r#"zh-Latn-CN"#, r##"error"##, r###"text-not-found"###),
  ///           r#####"MeiYou ZhaoDao BenDiHua WenBen"#####,
  ///         ),
  ///       ],
  ///     }
  /// }
  /// ```
  ///
  /// ### Get Text
  ///
  /// ```ignore
  /// use glossa_shared::PhfTripleKey;
  ///
  /// fn test_get_text() {
  ///     let map = map();
  ///     let get_text =
  ///       |language| map.get(&PhfTripleKey(language, "error", "text-not-found"));
  ///
  ///     let zh_text = get_text("zh");
  ///     assert_eq!(zh_text, Some(&"未找到本地化文本"));
  ///
  ///     let language_chain = ["gsw", "de-CH", "de", "en"];
  ///
  ///     let text = language_chain
  ///       .into_iter()
  ///       .find_map(get_text);
  ///     assert_eq!(text, Some(&"Kein lokalisierter Text gefunden"));
  /// }
  /// ```
  pub fn output_phf_all_in_one(&'h self, non_tmpl: MapType) -> io::Result<String> {
    let vis_fn = self.get_visibility().as_str();

    non_tmpl
      .get_non_template_maps(self)?
      .iter()
      .filter(|(_, data)| !data.is_empty())
      .flat_map(|(lang, map_entry)| {
        map_entry
          .iter()
          .map(|((name, k), v)| {
            let new_key =
              RawTripleKey(lang.to_compact_string(), name.as_str(), k.as_str());
            let value = format_compact!(r##########"r#####"{v}"#####"##########);
            (new_key, value)
          })
      })
      .fold(OrderedMap::new(), |mut acc, (k, v)| {
        acc.entry(k, &v);
        acc
      })
      .pipe_ref_mut(|ordered_map| {
        format!(
          r#"{vis_fn} const fn map() -> super::PhfL10nAllInOneMap {{
      use super::PhfTripleKey as Key;
        {code}  }}"#,
          code = ordered_map
            .phf_path("super::phf")
            .build()
        )
      })
      .pipe(Ok)
  }

  /// Generates Perfect Hash Function (PHF) maps for localization data
  ///
  /// # Behavior
  ///
  /// - Processes non-template maps in parallel
  /// - Filters out empty localization datasets
  /// - Generates PHF maps preserving insertion order
  /// - Creates individual Rust module files per language
  ///
  /// # Errors
  ///
  /// Returns [`io::Result`] for file I/O operations failures
  pub fn output_phf(&'h self, non_tmpl: MapType) -> io::Result<()> {
    let vis_fn = self.get_visibility().as_str();

    non_tmpl
      .get_non_template_maps(self)?
      .par_iter()
      .filter(|(_, data)| !data.is_empty())
      .map(|(lang, map_entry)| {
        let new_phf_map = assemble_phf_map(map_entry);
        (lang, new_phf_map)
      })
      .try_for_each(|(lang, mut map)| {
        writeln!(
          &mut self.create_rs_mod_file(lang)?,
          r##"{vis_fn} const fn map() -> super::PhfL10nOrderedMap {{
          use super::PhfTupleKey as Key;
          {code}  }}"##,
          code = map
            .phf_path("super::phf")
            .build()
        )
      })

    // fn l10n_maps() -> Box<[(lang_id::LangID, PhfL10nOrderedMap)]> {
    //  use lang_id::consts::*;
    //  vec![
    //    #[cfg(feature = "l10n_en")]
    //    (lang_id_en(), l10n_en::maps),
    //
    //    #[cfg(feature = "l10n_zh")]
    //    (lang_id_zh(), l10n_zh::maps),
    // ].into_boxed_slice()
    // };
  }

  /// Creates Rust module file writer with standardized naming
  ///
  /// # File Naming
  ///
  /// Generates filenames following format:
  /// `{mod_prefix}{language_snake_case}.rs`
  /// - Converts language ID to snake_case (e.g., "en-US" → "en_us")
  /// - Applies module prefix from generator configuration
  pub(crate) fn create_rs_mod_file<D: core::fmt::Display>(
    &self,
    language: &D,
  ) -> io::Result<BufWriter<File>> {
    let mod_prefix = self.get_mod_prefix();
    let rs_file_name =
      format_compact!("{mod_prefix}{}.rs", to_lower_snake_case(language));

    eprintln!(
      "#[cfg(feature = \"{mod_prefix}{language}\")]\n\
      mod {rs_file_name}\n"
    );

    let out_dir = self.get_outdir().as_deref();

    create_buf_writer(out_dir, rs_file_name)
  }
}

/// Constructs ordered PHF map from localization entries
///
/// # Parameter
///
/// - `map_entry`
///   - Localization data in BTreeMap format
///
/// # Note
/// Preserves insertion order using [`phf_codegen::OrderedMap`]
fn assemble_phf_map(map_entry: &L10nBTreeMap) -> OrderedMap<PhfTupleKey<'_>> {
  map_entry
    .iter()
    .map(|((name, k), v)| {
      let tuple_key = PhfTupleKey(name.as_str(), k.as_str());
      let value = format_compact!(r##########"r#####"{v}"#####"##########);
      (tuple_key, value)
    })
    .fold(OrderedMap::new(), |mut acc, x| {
      acc.entry(x.0, &x.1);
      acc
    })
}

/// Normalizes snake_case format
///
/// - en.US => en_us
/// - en-US => en_us
/// - en-Latn-US => en_latn_us
/// - zh-Hans-CN => zh_hans_cn
///
/// ## Conversion Rules
///
/// 1. Convert to ASCII lowercase
/// 2. Replace hyphens/dots with underscores
pub fn to_lower_snake_case<D: core::fmt::Display>(id: D) -> MiniStr {
  format_compact!("{id}")
    .tap_mut(|s| s.make_ascii_lowercase())
    .chars()
    .map(|c| match c {
      '-' | '.' => '_',
      c => c,
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use anyhow::Result as AnyResult;
  use glossa_shared::{PhfL10nAllInOneMap, PhfL10nOrderedMap, PhfTripleKey};

  use super::*;
  use crate::generator::dbg_generator::{
    de_en_es_pt_zh_generator, en_gb_generator, new_generator,
  };

  #[ignore]
  #[test]
  fn test_build_en_gb_phf() -> AnyResult<()> {
    en_gb_generator().output_phf(MapType::Regular)?;
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_build_all_phf() -> AnyResult<()> {
    new_generator().output_phf(MapType::Regular)?;
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_build_all_in_one_phf() -> AnyResult<()> {
    let function_data = new_generator().output_phf_all_in_one(MapType::Regular)?;
    println!("{function_data}");
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_build_de_zh_es_pt_phf_all_in_one() -> AnyResult<()> {
    let function_data =
      de_en_es_pt_zh_generator().output_phf_all_in_one(MapType::Regular)?;
    println!("{function_data}");
    Ok(())
  }

  pub(crate) const fn en_gb_map() -> PhfL10nOrderedMap {
    use PhfTupleKey as Key;
    phf::OrderedMap {
      key: 12913932095322966823,
      disps: &[(0, 0)],
      idxs: &[0],
      entries: &[(
        Key(r#"error"#, r##"text-not-found"##),
        r#####"No localised text found"#####,
      )],
    }
  }

  pub(crate) const fn all_in_one_map() -> PhfL10nAllInOneMap {
    use PhfTripleKey as Key;
    phf::OrderedMap {
      key: 12913932095322966823,
      disps: &[(2, 3), (2, 0)],
      idxs: &[5, 4, 0, 6, 3, 2, 1],
      entries: &[
        (
          Key(r#"de"#, r##"error"##, r###"text-not-found"###),
          r#####"Kein lokalisierter Text gefunden"#####,
        ),
        (
          Key(r#"en"#, r##"error"##, r###"text-not-found"###),
          r#####"No localized text found"#####,
        ),
        (
          Key(r#"en-GB"#, r##"error"##, r###"text-not-found"###),
          r#####"No localised text found"#####,
        ),
        (
          Key(r#"es"#, r##"error"##, r###"text-not-found"###),
          r#####"No se encontró texto localizado"#####,
        ),
        (
          Key(r#"pt"#, r##"error"##, r###"text-not-found"###),
          r#####"Nenhum texto localizado encontrado"#####,
        ),
        (
          Key(r#"zh"#, r##"error"##, r###"text-not-found"###),
          r#####"未找到本地化文本"#####,
        ),
        (
          Key(r#"zh-Latn-CN"#, r##"error"##, r###"text-not-found"###),
          r#####"MeiYou ZhaoDao BenDiHua WenBen"#####,
        ),
      ],
    }
  }

  #[ignore]
  #[test]
  fn test_get_phf_en_map() {
    let map = en_gb_map();
    let v = map.get(&PhfTupleKey("error", "text-not-found"));
    dbg!(v);
  }

  #[ignore]
  #[test]
  fn doc_test_get_all_in_one_map() {
    let map = all_in_one_map();
    let get_text =
      |language| map.get(&PhfTripleKey(language, "error", "text-not-found"));

    let zh_text = get_text("zh");
    assert_eq!(zh_text, Some(&"未找到本地化文本"));

    let language_chain = ["gsw", "de-CH", "de", "en"];

    let text = language_chain
      .into_iter()
      .find_map(get_text);
    assert_eq!(text, Some(&"Kein lokalisierter Text gefunden"));
  }
}
