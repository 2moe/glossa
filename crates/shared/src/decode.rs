pub(crate) use bincode::config::standard as bincode_std_cfg;
pub use glossa_dsl::error::ResolverResult;

use crate::type_aliases::{DSLMaps, L10nDSLMap, L10nFlattenMap, L10nMaps};

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

/// decodes single L10n data (e.g., read("en-GB.bincode") )
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map<(map_name, key), value>,
///   read_bytes_size
/// )
/// ```
pub fn decode_single_data_to_map(
  slice: &[u8],
) -> ResolverResult<(L10nFlattenMap, usize)> {
  decode_slice(slice)
}

/// decodes **all_in_one** L10n data (e.g., read("all.bincode") )
///
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map< LangID, Map<(map_name, key), value> >,
///   read_bytes_size
/// )
/// ```
pub fn decode_to_maps(slice: &[u8]) -> ResolverResult<(L10nMaps, usize)> {
  decode_slice(slice)
}

/// decodes single L10n DSL data (e.g., read("en-GB.tmpl.bincode") )
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map<map_name, Resolver>,
///   read_bytes_size
/// )
/// ```
pub fn decode_single_data_to_dsl_map(
  slice: &[u8],
) -> ResolverResult<(L10nDSLMap, usize)> {
  decode_slice(slice)
}

/// decodes **all_in_one** DSL L10n data (e.g., read("all.tmpl.bincode") )
///
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map< LangID, Map<map_name, Resolver> >,
///   read_bytes_size
/// )
/// ```
pub fn decode_to_dsl_maps(slice: &[u8]) -> ResolverResult<(DSLMaps, usize)> {
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
/// decodes single L10n file (e.g., en-GB.bincode)
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map<(map_name, key), value>,
/// )
/// ```
pub fn decode_single_file_to_flatten_map<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nFlattenMap> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
/// decodes **all_in_one** L10n file (e.g., all.bincode)
///
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map< LangID, Map<(map_name, key), value> >,
/// )
/// ```
pub fn decode_file_to_maps<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nMaps> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
/// decodes single L10n DSL file (e.g., en-GB.tmpl.bincode)
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map<map_name, Resolver>,
/// )
/// ```
pub fn decode_single_file_to_dsl_map<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<L10nDSLMap> {
  decode_file(src_file)
}

#[cfg(feature = "std")]
/// decodes **all_in_one** DSL L10n file (e.g., all.tmpl.bincode)
///
///
/// returns:
///
/// ```ignore
/// Ok(
///   Map< LangID, Map<map_name, Resolver> >,
/// )
/// ```
pub fn decode_file_to_dsl_maps<P: AsRef<std::path::Path>>(
  src_file: P,
) -> ResolverResult<DSLMaps> {
  decode_file(src_file)
}
