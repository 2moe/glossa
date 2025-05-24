use std::{
  ffi::OsStr,
  path::{Path, PathBuf},
  sync::OnceLock,
};

use compact_str::ToCompactString;
use glossa_dsl::error::ResolverResult;
use lang_id::LangID;
use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tap::Pipe;

use crate::{
  decode::file::{
    decode_file_to_dsl_maps, decode_file_to_maps, decode_single_file_to_dsl_map,
    decode_single_file_to_flatten_map,
  },
  type_aliases::{DSLMaps, L10nMaps},
};

// pub trait LoadBincode {
//   fn get_bincode_dir() -> Option<PathBuf>;

pub fn list_bincode_files(bincode_dir: Option<&Path>) -> Option<Vec<PathBuf>> {
  bincode_dir?
    .read_dir()
    .ok()?
    .par_bridge()
    .flatten()
    .map(|x| x.path())
    .filter(|x| !x.is_dir())
    .collect::<Vec<_>>()
    .pipe(|x| (!x.is_empty()).then_some(x))
}

pub fn list_static_bincode_files(
  bincode_dir: Option<&Path>,
) -> Option<&'static [PathBuf]> {
  static V: OnceLock<Option<Vec<PathBuf>>> = OnceLock::new();
  V.get_or_init(|| list_bincode_files(bincode_dir))
    .as_deref()
}

pub fn try_load_files<P: AsRef<Path>>(files: &[P]) -> ResolverResult<L10nMaps> {
  let all = OsStr::new("all");
  log::debug!("Loading bincode data ...");

  let iter = files.iter().map(AsRef::as_ref);

  match iter
    .clone()
    .find(|x| x.file_stem() == Some(all))
  {
    Some(p) => {
      debug!("load: all.bincode");
      decode_file_to_maps(p)
    }
    _ => {
      // use `.collect().par_iter()` instead of `iter.par_bridge()`
      iter
        .collect::<Vec<_>>()
        .par_iter()
        .filter_map(|file| {
          decode_single_file_to_flatten_map(file)
            .ok()
            .and_then(|data| {
              file
                .file_stem()
                .inspect(|s| debug!("file-stem: {s:?}"))
                .and_then(|s| s.to_str())
                .map(|lang| (lang.to_compact_string(), data))
            })
        })
        .collect::<L10nMaps>()
        .pipe(Ok)
    }
  }
}

pub fn try_load_dsl_files<P: AsRef<Path>>(files: &[P]) -> ResolverResult<DSLMaps> {
  let all = OsStr::new("all");
  log::debug!("Loading bincode data ...");

  let iter = files.iter().map(AsRef::as_ref);

  match iter
    .clone()
    .find(|x| x.file_stem() == Some(all))
  {
    Some(p) => {
      debug!("load DSL: all.bincode");
      decode_file_to_dsl_maps(p)
    }
    _ => iter
      .collect::<Vec<_>>()
      .par_iter()
      // .par_bridge()
      .filter_map(|file| {
        decode_single_file_to_dsl_map(file)
          .ok()
          .and_then(|data| {
            file
              .file_stem()
              .inspect(|s| debug!("file-stem: {s:?}"))
              .and_then(|s| s.to_str())
              .map(|lang| (lang.to_compact_string(), data))
          })
      })
      .collect::<DSLMaps>()
      .pipe(Ok),
  }
}

pub fn merge_locales(
  lhs: Option<&L10nMaps>,
  rhs: Option<&[LangID]>,
) -> Option<Box<[LangID]>> {
  let builtin = rhs
    .into_iter()
    .flatten()
    .cloned();

  let locales = lhs?
    .keys()
    .inspect(|x| debug!("language: {x}"))
    .filter_map(|x| x.parse::<LangID>().ok())
    .chain(builtin)
    .collect::<ahash::HashSet<_>>() // dedup
    .into_iter()
    .collect();

  Some(locales)
}
