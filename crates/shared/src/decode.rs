pub(crate) use bincode::config::standard as bincode_std_cfg;
pub use tmpl_resolver::ResolverResult;

use crate::type_aliases::{L10nFlattenMap, L10nMaps, L10nTemplateMap, TemplateMaps};

// macro_rules! decode_slice {
//   ($slice:expr) => {
//     bincode::serde::decode_from_slice($slice,
// bincode::config::standard())?.pipe(Ok)   };
// }

pub fn decode_slice<T>(slice: &[u8]) -> ResolverResult<(T, usize)>
where
  T: serde::de::DeserializeOwned,
{
  bincode::serde::decode_from_slice(slice, bincode_std_cfg()) //
    .map_err(Into::into)
}

pub fn decode_slice_to_flatten_map(
  slice: &[u8],
) -> ResolverResult<(L10nFlattenMap, usize)> {
  decode_slice(slice)
}

pub fn decode_slice_to_maps(slice: &[u8]) -> ResolverResult<(L10nMaps, usize)> {
  decode_slice(slice)
}

pub fn decode_slice_to_tmpl_map(
  slice: &[u8],
) -> ResolverResult<(L10nTemplateMap, usize)> {
  decode_slice(slice)
}

pub fn decode_slice_to_template_maps(
  slice: &[u8],
) -> ResolverResult<(TemplateMaps, usize)> {
  decode_slice(slice)
}
// ----------------
#[cfg(feature = "std")]
pub fn decode_file<T, P>(src_file: P) -> ResolverResult<T>
where
  T: serde::de::DeserializeOwned,
  P: AsRef<std::path::Path>,
{
  use std::{fs::File, io::BufReader};

  use bincode::serde::decode_from_std_read;
  use tap::Pipe;

  let decode = |src| decode_from_std_read::<T, _, _>(src, bincode_std_cfg());

  src_file
    .pipe(File::open)?
    .pipe(BufReader::new)
    .pipe_ref_mut(decode)
    .map_err(Into::into)
}

#[cfg(feature = "std")]
pub fn decode_file_to_flatten_map<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nFlattenMap> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
pub fn decode_file_to_maps<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nMaps> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
pub fn decode_file_to_tmpl_map<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nTemplateMap> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
pub fn decode_file_to_template_maps<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<TemplateMaps> {
  decode_file(src_file)
}
