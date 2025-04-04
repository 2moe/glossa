use super::*;

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

/// decodes single L10n DSL data (e.g., read("en-GB.ast.bincode") )
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

/// decodes **all_in_one** DSL L10n data (e.g., read("all.ast.bincode") )
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
