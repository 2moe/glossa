use super::*;

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

/// decodes single L10n DSL file (e.g., en-GB.ast.bincode)
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

/// decodes **all_in_one** DSL L10n file (e.g., all.ast.bincode)
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
