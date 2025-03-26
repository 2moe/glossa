use std::{
  ffi::OsStr,
  fs,
  path::{Path, PathBuf},
  sync::OnceLock,
};

pub type L10nResMap = HashMap<KString, Vec<L10nMapEntry>>;

use ahash::HashMap;
use anyhow::bail;
use dashmap::DashSet;
use getset::{Getters, WithSetters};
use glossa_shared::ToCompactString;
use kstring::KString;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use tap::{Pipe, TapFallible, TryConv};
use tmpl_resolver::TemplateResolver;
use walkdir::{DirEntry, WalkDir};

use crate::{AnyResult, MiniStr, to_kstr};

#[derive(Getters, WithSetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct L10nResources<'i> {
  dir: PathBuf,
  tmpl_suffix: MiniStr,
  include_languages: &'i [&'i str],
  include_map_names: &'i [&'i str],
  exclude: &'i [&'i str],
  #[getset(get)]
  /// get data: [Self::get_or_init_data]
  lazy_data: OnceLock<L10nResMap>,
}

impl Default for L10nResources<'_> {
  /// Default:
  ///
  /// ```ignore
  /// {
  ///   tmpl_suffix: ".tmpl"
  ///   ..Default::default()
  /// }
  /// ```
  fn default() -> Self {
    Self {
      tmpl_suffix: ".tmpl".into(),
      dir: Default::default(),
      include_languages: Default::default(),
      include_map_names: Default::default(),
      exclude: Default::default(),
      lazy_data: Default::default(),
    }
  }
}

fn walk_file<P: AsRef<Path>>(dir: P) -> Option<impl Iterator<Item = DirEntry>> {
  dir
    .pipe(WalkDir::new)
    .follow_links(true)
    .into_iter()
    .filter_map(Result::ok)
    .filter(is_supported_config_file)
    .pipe(Some)
}

fn is_supported_config_file(e: &DirEntry) -> bool {
  let f = e.path();
  f.is_file()
    && f
      .extension()
      .is_some_and(is_supported_cfg_format)
}

fn is_supported_cfg_format<S: AsRef<OsStr>>(o: S) -> bool {
  ["toml", "ron", "json", "json5", "yml", "yaml"]
    .iter()
    .map(OsStr::new)
    .any(|a| o.as_ref() == a)
}

fn dir_name_to_opt_lang(dir: &Path) -> Option<KString> {
  dir
    .file_name()?
    .to_str()?
    .pipe(KString::from_ref)
    .pipe(Some)
}

impl L10nResources<'_> {
  /// Constructs a new `L10nResources` instance with localization directory.
  ///
  /// The provided path should point to a directory containing
  /// localization files.
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_codegen::L10nResources;
  ///
  /// let _res = L10nResources::new("../../locales/");
  /// ```
  pub fn new<P: Into<PathBuf>>(dir: P) -> Self {
    Self {
      dir: dir.into(),
      ..Default::default()
    }
  }

  pub fn get_or_init_data(&self) -> &L10nResMap {
    self
      .get_lazy_data()
      .get_or_init(|| {
        self
          .collect_localized_files()
          .expect("Failed to init L10nResources Data")
      })
  }

  fn walk_dir(&self) -> Option<impl ParallelIterator<Item = PathBuf>> {
    self
      .get_dir()
      .pipe(fs::read_dir)
      .ok()?
      .par_bridge()
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|d| d.is_dir())
      .into()
  }

  /// Processes localization file and converts to L10nMapEntry
  ///
  /// ## Implementation Notes
  ///
  /// 1. Deserialization Precedence: File contents must be deserialized and
  ///    validated BEFORE registering the file stem in the tracking set. This
  ///    ensures only files with valid data claim priority.
  /// 2. Empty Data Handling: HashMap data with empty content after
  ///    deserialization are intentionally excluded to prevent empty entries
  ///    from shadowing valid data files.
  /// 3. File Stem Collisions: The first valid file for each stem establishes
  ///    priority. Subsequent files with the same stem (different extensions)
  ///    will be ignored, even if they contain data.
  ///
  /// ## Edge Case Example
  ///
  /// When both `test.json` (empty) and `test.ron` (valid data) exist:
  /// - Bad order: Processing `test.json` first would permanently block
  ///   `test.ron`
  /// - Correct order: Deserialize first, then `test.json` gets filtered out by
  ///   empty check, allowing `test.ron` to claim the stem when processed.
  fn process_file<P: AsRef<Path> + core::fmt::Debug>(
    &self,
    file: P,
    file_stem: &MiniStr,
    stem_set: &DashSet<MiniStr>,
  ) -> Option<L10nMapEntry> {
    let data = deser_config_file(&file)
      .tap_err(|e| eprintln!("[WARN] Deserialization error for {file:?}: {e}"))
      .ok()?;

    // Reject empty datasets to prevent empty entries from reserving file stems
    (!data.is_empty()).then_some(())?;

    // let file_stem = get_file_stem(file)?;

    stem_set
      .insert(file_stem.clone())
      .then_some(())?;

    let suffix = self.get_tmpl_suffix().as_str();

    let (tmpl_data, data) = match file_stem.ends_with(suffix) && !suffix.is_empty() {
      true => (
        data
          .try_conv::<TemplateResolver>()
          .ok(),
        None,
      ),
      _ => (None, Some(data)),
    };

    let map_name = file_stem
      .trim_end_matches(suffix)
      .into();

    L10nMapEntry {
      map_name,
      data,
      tmpl_data,
    }
    .pipe(Some)
  }

  fn collect_localized_files(&self) -> Option<L10nResMap> {
    self
      .walk_dir()?
      .filter(|dir| self.filter_include_languages(dir))
      .filter(|dir| self.filter_exclude_languages(dir))
      .filter_map(|ref dir| {
        let entries = self.parallel_collect_l10n_entries(dir)?;
        let lang = dir_name_to_opt_lang(dir)?;
        (!entries.is_empty()).then_some((lang, entries))
      })
      .collect::<HashMap<_, _>>()
      .into()
  }

  fn parallel_collect_l10n_entries(&self, dir: &Path) -> Option<Vec<L10nMapEntry>> {
    let stem_set = DashSet::with_capacity(64);

    dir
      .pipe(walk_file)?
      .par_bridge()
      .filter_map(annotate_entry_with_stem)
      .filter(|(_, map_name)| self.filter_include_map_names(map_name))
      .filter_map(|(file, file_stem)| {
        self.process_file(file.path(), &file_stem, &stem_set)
      })
      .collect::<Vec<_>>()
      .into()
  }

  fn filter_include_map_names(&self, map_name: &MiniStr) -> bool {
    match self.include_map_names {
      [] => true,
      list => list
        .iter()
        .any(|item| map_name.eq_ignore_ascii_case(item)),
    }
  }

  fn filter_exclude_languages(&self, dir: &Path) -> bool {
    match self.exclude {
      [] => true,
      list => match dir.file_name() {
        Some(dirname) => !list
          .iter()
          .any(|item| dirname.eq_ignore_ascii_case(item)),
        _ => true,
      },
    }
  }

  fn filter_include_languages(&self, dir: &Path) -> bool {
    match self.include_languages {
      [] => true,
      list => dir
        .file_name()
        .is_some_and(|dirname| {
          list
            .iter()
            .any(|item| dirname.eq_ignore_ascii_case(item))
        }),
    }
  }
}

fn annotate_entry_with_stem(p: DirEntry) -> Option<(DirEntry, MiniStr)> {
  p.path()
    .pipe(get_file_stem)
    .map(|stem| (p, stem))
}

fn get_file_stem<P: AsRef<Path>>(file: P) -> Option<MiniStr> {
  file
    .as_ref()
    .file_stem()?
    .to_str()?
    .to_compact_string()
    .pipe(Some)
}

fn deser_config_file<P: AsRef<Path>>(
  file: P,
) -> AnyResult<HashMap<KString, MiniStr>> {
  let cfg_text = file.pipe_ref(fs::read_to_string)?;

  if cfg_text.trim().is_empty() {
    bail!("Empty File Content")
  }

  let new_err = || "Failed to deserialize config file.".pipe(anyhow::Error::msg);

  let data = match file
    .as_ref()
    .extension()
    .map(|x| x.to_string_lossy())
    .ok_or_else(new_err)?
    .as_ref()
  {
    #[cfg(feature = "json")]
    "json" => match serde_json::from_str(&cfg_text) {
      Ok(m) => m,
      #[cfg(not(feature = "json5"))]
      e => e?,
      #[cfg(feature = "json5")]
      _ => serde_json5::from_str(&cfg_text)?,
    },
    #[cfg(feature = "json5")]
    "json5" => serde_json5::from_str(&cfg_text)?,
    #[cfg(feature = "ron")]
    "ron" => ron::from_str(&cfg_text)?,
    #[cfg(feature = "toml")]
    "toml" => toml::from_str(&cfg_text)?,
    #[cfg(feature = "yaml")]
    "yaml" | "yml" => serde_yml::from_str(&cfg_text)?,
    _ => bail!("Skip unsupported file"),
  };

  Ok(data)
}

#[derive(Getters, WithSetters, Debug, Clone, Default, Serialize, Deserialize)]
#[getset(get = "pub(crate) with_prefix", set_with = "pub(crate)")]
pub struct L10nMapEntry {
  map_name: MiniStr,
  data: Option<HashMap<KString, MiniStr>>,
  tmpl_data: Option<TemplateResolver>,
}

impl L10nMapEntry {
  pub(crate) fn map_name_to_kstring(&self) -> KString {
    self
      .get_map_name()
      .pipe(to_kstr)
  }
}

#[cfg(test)]
pub(crate) mod dbg_shared {
  use crate::L10nResources;

  pub(crate) const DIR: &str = "../../locales/";

  pub(crate) fn new_resources<'i>() -> L10nResources<'i> {
    // L10nResources::default().with_dir(DIR.into())
    L10nResources::new(DIR)
    // .with_tmpl_suffix(".tmpl".into())
  }
}

#[cfg(test)]
mod tests {
  use std::{collections::BTreeMap, fs, io};

  use testutils::simple_benchmark;

  use super::*;
  use crate::resources::dbg_shared::new_resources;

  #[ignore]
  #[test]
  fn test_read_dir() -> io::Result<()> {
    for (idx, path) in dbg_shared::DIR
      .pipe(fs::read_dir)?
      .filter_map(Result::ok)
      .map(|x| x.path())
      .filter(|e| e.is_dir())
      .enumerate()
    {
      dbg!(path.file_name(), idx);
    }
    Ok(())
  }

  #[ignore]
  #[test]
  fn bench_init_res_data() {
    simple_benchmark(|| {
      dbg_shared::new_resources();
    });
  }

  #[ignore]
  #[test]
  fn test_init_res_data() {
    let res = dbg_shared::new_resources();
    let map = res
      .get_or_init_data()
      .iter()
      .collect::<BTreeMap<_, _>>();
    dbg!(map);
    // HashMap<("en", "map_name", map)>
  }

  #[ignore]
  #[test]
  fn test_only_includes_en() {
    let res = new_resources()
      .with_include_languages(&["zh", "en"])
      // .with_include_map_names(&["hi.tmpl"])
      .with_exclude(&["zh"]);
    let map = res.get_or_init_data();
    // println!("{map:?}")
    dbg!(map);
  }

  #[ignore]
  #[test]
  fn test_only_includes_de_and_und() {
    let res = new_resources()
      .with_include_languages(&["de", "und", "es"])
      .with_exclude(&["es"]);
    let map = res.get_or_init_data();
    // println!("{map:?}")
    dbg!(map);
  }
}
