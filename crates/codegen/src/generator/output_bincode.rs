use std::{
  fs::File,
  io::{self, BufWriter},
  path::Path,
};

use glossa_shared::{MiniStr, fmt_compact, tap::Pipe};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
  AnyResult,
  generator::{Generator, MapType},
  resources::L10nResMap,
};

impl<'h> Generator<'_, 'h> {
  /// Retrieves the localization resource map containing all translation data
  pub(crate) fn get_l10n_res_map(&self) -> &L10nResMap {
    self
      .get_resources()
      .get_or_init_data()
  }

  /// Serializes data to binary format (bincode) with parallel file operations
  ///
  /// # Errors
  ///
  /// Returns [`AnyResult`] with error details for:
  /// - File creation failures
  /// - Serialization errors
  fn encode_bincode<T, D>(&self, lang_id: D, data: T) -> AnyResult<()>
  where
    T: serde::Serialize,
    D: core::fmt::Display,
  {
    bincode::serde::encode_into_std_write(
      data,
      &mut self.create_bincode_file(lang_id)?,
      bincode::config::standard(),
    )?;
    Ok(())
  }

  /// Generates consolidated binary file containing all localization data
  ///
  /// > `>` "outdir()/all{bincode_suffix}"
  ///
  /// # Behavior
  /// - For template maps: Creates "all{bincode_suffix}" containing template
  ///   data
  /// - For regular maps: Creates "all{bincode_suffix}" containing all language
  ///   data
  ///
  /// # Errors
  /// Returns [`AnyResult`] with error details for:
  /// - File I/O failures
  /// - Serialization errors
  pub fn output_bincode_all_in_one(&'h self, map_type: MapType) -> AnyResult<()> {
    let all = "all";

    if map_type.is_template() {
      return match self.get_or_init_template_maps() {
        x if x.is_empty() => Ok(()),
        data => self.encode_bincode(all, data),
      };
    }

    let data = map_type.get_non_template_maps(self)?;
    self.encode_bincode(all, data)
  }

  /// Generates individual binary files per language
  ///
  /// > `>` "outdir()/{language}{bincode_suffix}"
  ///
  /// # Behavior
  /// - For template maps: Creates separate files for each template
  /// - For regular maps: Creates separate files for each language
  /// - Skips empty datasets
  ///
  ///
  /// ## Example
  ///
  /// "../../locales/en/unread.tmpl.toml":
  ///
  /// ```toml
  /// num-to-en = """
  /// $num ->
  ///   [0] zero
  ///   [1] one
  ///   [2] two
  ///   [3] three
  ///   *[other] {$num}
  /// """
  ///
  /// unread = "unread message"
  ///
  /// unread-count = """
  /// $num ->
  ///   [0] No {unread}s.
  ///   [1] You have { num-to-en } {unread}.
  ///   *[other] You have { num-to-en } {unread}s.
  /// """
  ///
  /// show-unread-messages-count = "{unread-count}"
  /// ```
  ///
  /// rs_code:
  ///
  /// ```no_run
  /// use glossa_shared::TemplateResolver;
  /// use glossa_codegen::{L10nResources, Generator, generator::MapType};
  /// use std::path::Path;
  ///
  /// let resources = L10nResources::new("../../locales/");
  ///
  /// // Output to tmp/{language}.tmpl.bincode
  /// Generator::default()
  ///   .with_resources(resources)
  ///   .with_outdir("tmp")
  ///   .with_bincode_suffix(".tmpl.bincode".into())
  ///   .output_bincode(MapType::Template)?;
  ///
  /// let file = Path::new("tmp").join("en.tmpl.bincode");
  /// let tmpl_maps = glossa_shared::decode::decode_single_file_to_template_map(file)?;
  /// let unread_tmpl = tmpl_maps
  ///   .get("unread")
  ///   .expect("Failed to get TemplateAST (map_name: unread)");
  ///
  /// let get_text = |num_str| {
  ///   unread_tmpl.get_with_context("show-unread-messages-count", &[("num", num_str)])
  /// };
  ///
  /// let one = get_text("1")?;
  /// assert_eq!(one, "You have one unread message.");
  ///
  /// let zero = get_text("0")?;
  /// assert_eq!(zero, "No unread messages.");
  ///
  /// # Ok::<(), anyhow::Error>(())
  /// ```
  pub fn output_bincode(&'h self, map_type: MapType) -> AnyResult<()> {
    if map_type.is_template() {
      return match self.get_or_init_template_maps() {
        x if x.is_empty() => Ok(()),
        iter => iter
          .par_iter()
          .try_for_each(|(lang, data)| self.encode_bincode(lang, data)),
      };
    }

    map_type
      .get_non_template_maps(self)?
      .par_iter()
      .try_for_each(|(lang, data)| self.encode_bincode(lang, data))
  }

  /// Creates buffered bincode file writer with standardized naming
  ///
  /// # File Naming
  /// Follows format: `{language}{suffix}`
  /// - `suffix` configured via [`Generator::get_bincode_suffix`]
  ///
  /// # Errors
  /// Returns [`io::Result`] for file creation failures
  pub(crate) fn create_bincode_file<D: core::fmt::Display>(
    &self,
    language: D,
  ) -> io::Result<BufWriter<File>> {
    let suffix = self.get_bincode_suffix();
    let bincode_name = fmt_compact!("{language}{suffix}");
    let out_dir = self.get_outdir().as_deref();

    create_buf_writer(out_dir, bincode_name)
  }
}

/// Creates buffered file writer with error handling
///
/// # Errors
/// Returns [`io::Result`] with error details for:
///
/// - Missing output directory
/// - File creation failures
pub(crate) fn create_buf_writer(
  out_dir: Option<&Path>,
  bincode_name: MiniStr,
) -> io::Result<BufWriter<File>> {
  out_dir
    .ok_or_else(|| io::Error::other("Invalid outdir"))?
    .join(bincode_name)
    .pipe(File::create)?
    .pipe(BufWriter::new)
    .pipe(Ok)
}

#[cfg(test)]
mod tests {
  use testutils::simple_benchmark;

  use super::*;
  use crate::generator::dbg_generator::new_generator;

  #[ignore]
  #[test]
  fn test_output_tmpl_maps_to_bincode_files() -> AnyResult<()> {
    new_generator()
      .with_bincode_suffix(".tmpl.bincode".into())
      .output_bincode(MapType::Template)
  }

  #[ignore]
  #[test]
  fn doc_test_encode_and_decode_tmpl_bincode() -> AnyResult<()> {
    let resources = crate::L10nResources::new("../../locales/");

    // Output to tmp/{language}.tmpl.bincode
    Generator::default()
      .with_resources(resources)
      .with_outdir("tmp")
      .with_bincode_suffix(".tmpl.bincode".into())
      .output_bincode(MapType::Template)?;

    let file = Path::new("tmp").join("en.tmpl.bincode");
    let tmpl_maps = glossa_shared::decode::decode_single_file_to_template_map(file)?;
    let unread_tmpl = tmpl_maps
      .get("unread")
      .expect("Failed to get TemplateAST (map_name: unread)");

    let get_text = |num_str| {
      unread_tmpl.get_with_context("show-unread-messages-count", &[("num", num_str)])
    };

    let one = get_text("1")?;
    assert_eq!(one, "You have one unread message.");

    let zero = get_text("0")?;
    assert_eq!(zero, "No unread messages.");

    Ok(())
  }

  #[ignore]
  #[test]
  fn test_encode_regular_aio_bincode() -> AnyResult<()> {
    new_generator()
      .with_bincode_suffix("_regular.bincode".into())
      .output_bincode_all_in_one(MapType::Regular)
  }

  #[ignore]
  #[test]
  #[cfg(feature = "highlight")]
  fn test_encode_highlight_aio_bincode() -> AnyResult<()> {
    use crate::generator::dbg_generator;

    dbg_generator::highlight_generator()
      .with_bincode_suffix(".highlight.bincode".into())
      .output_bincode_all_in_one(MapType::Highlight)
  }

  #[ignore]
  #[test]
  #[cfg(feature = "highlight")]
  fn test_decode_highlight_aio() -> glossa_shared::decode::ResolverResult<()> {
    let data =
      glossa_shared::decode::decode_file_to_maps("tmp/all.highlight.bincode")?;

    let en_maps = data.get("en").unwrap();
    let value = en_maps
      .get(&("md_md".into(), "pwsh".into()))
      .unwrap();
    println!("{value}");
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_encode_tmpl_aio_bincode() -> AnyResult<()> {
    new_generator()
      .with_bincode_suffix("_tmpl.bincode".into())
      .output_bincode_all_in_one(MapType::Template)
  }

  #[ignore]
  #[test]
  fn test_decode_tmpl_aio_bincode() -> AnyResult<()> {
    let raw_map =
      glossa_shared::decode::decode_file_to_template_maps("tmp/all_tmpl.bincode")?;

    let zh_maps = raw_map.get("zh").unwrap();
    let zh_unread_map = zh_maps.get("unread").unwrap();

    let text = zh_unread_map
      .get_with_context("show-unread-messages-count", &[("num", "2")])?;
    dbg!(text);

    Ok(())
  }

  /// Debug:
  /// - decode from file
  ///   - Time taken: 422.708µs
  /// - decode from slice
  ///   - Time taken: 335.333µs
  ///
  /// Release:
  /// - decode from file
  ///   - Time taken: 190.209µs
  /// - decode from slice
  ///   - Time taken: 63.25µs
  #[ignore]
  #[test]
  fn bench_decode_regular_file() -> AnyResult<()> {
    let file = Path::new("tmp").join("all_regular.bincode");

    eprintln!("decode from file");
    simple_benchmark(|| {
      let _ = glossa_shared::decode::decode_file_to_maps(&file);
    });

    let bytes = std::fs::read(&file)?;
    let decode_slice = || glossa_shared::decode::decode_to_maps(&bytes);

    eprintln!("decode from slice");
    simple_benchmark(|| {
      let _ = decode_slice();
    });

    Ok(())
  }
}
