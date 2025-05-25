use std::io;

use glossa_shared::{
  fmt_compact,
  tap::{Pipe, Tap},
};

use crate::{
  Generator,
  generator::{MapType, locales::ensure_length_equal},
};

impl Generator<'_> {
  /// Generates a function that routes to the appropriate module and extracts
  /// the corresponding value.
  ///
  /// Example: `self.output_router_for_match_fns(MapType::Regular, false)?`
  ///
  /// Output String Sample:
  ///
  /// ```ignore
  /// pub const fn map(language: &[u8], key: &[u8]) -> &'static str {
  ///   use super::*;
  ///   match language {
  ///     #[cfg(feature = "l10n-de")]
  ///     b"de" => l10n_de::map(key),
  ///     #[cfg(feature = "l10n-en-GB")]
  ///     b"en-GB" => l10n_en_gb::map(key),
  ///     _ => "",
  ///   }
  /// }
  /// ```
  pub fn output_router_for_match_fns(
    &self,
    map_type: MapType,
    contains_map_name: bool,
  ) -> io::Result<String> {
    let header = format!(
      "const fn map(language: &[u8], {opt_parm}key: &[u8]) -> &'static str {{
      use super::*;
      match language {{
    ",
      opt_parm = if contains_map_name { "map_name: &[u8], " } else { "" }
    );

    let new_header = || self.new_fn_header(&header);

    let locales = self.collect_raw_locales(map_type)?;
    let features = self.collect_cargo_feature_names(map_type)?;
    let modules = self.collect_rs_mod_names(map_type)?;
    ensure_length_equal(features.len(), modules.len())?;

    features
      .iter()
      .zip(modules)
      .zip(locales)
      .fold(
        new_header(), //
        |mut acc, ((feat, mod_name), locale)| {
          let cfg_line = fmt_compact!("  #[cfg(feature = \"{feat}\")]\n  ");
          let match_line = match contains_map_name {
            true => {
              fmt_compact!(r##"b"{locale}" => {mod_name}::map(map_name, key),"##)
            }
            _ => fmt_compact!(r##"b"{locale}" => {mod_name}::map(key),"##),
          };
          [cfg_line, match_line].map(|s| acc.push_str(&s));
          acc.push('\n');
          acc
        },
      )
      .tap_mut(|buf| buf.push_str("  _ => \"\",\n}}"))
      .pipe(Ok)
  }

  /// Generates a function that routes to the appropriate phf map.
  ///
  /// Example: `self.output_router_for_phf_maps(MapType::Regular, true)?`
  ///
  /// Output String Sample:
  ///
  /// ```ignore
  /// // super: use glossa_shared::PhfL10nOrderedMap;
  /// pub(crate) const fn map(language: &[u8]) -> super::PhfL10nOrderedMap {
  ///   use super::*;
  ///   match language {
  ///     #[cfg(feature = "l10n-ar")]
  ///     b"ar" => l10n_ar::map(),
  ///     #[cfg(feature = "l10n-as")]
  ///     b"as" => l10n_as::map(),
  ///     _ => "",
  ///   }
  /// }
  /// ```
  pub fn output_router_for_phf_maps(
    &self,
    map_type: MapType,
    contains_map_name: bool,
  ) -> io::Result<String> {
    let header = format!(
      "const fn map(language: &[u8]) -> super::{phf_type} {{
      use super::*;
      match language {{
    ",
      phf_type = if contains_map_name { "PhfL10nOrderedMap" } else { "PhfStrMap" }
    );

    let new_header = || self.new_fn_header(&header);

    let locales = self.collect_raw_locales(map_type)?;
    let features = self.collect_cargo_feature_names(map_type)?;
    let modules = self.collect_rs_mod_names(map_type)?;
    ensure_length_equal(features.len(), modules.len())?;

    features
      .iter()
      .zip(modules)
      .zip(locales)
      .fold(
        new_header(), //
        |mut acc, ((feat, mod_name), locale)| {
          let cfg_line = fmt_compact!("  #[cfg(feature = \"{feat}\")]\n  ");
          let match_line = fmt_compact!(r##"b"{locale}" => {mod_name}::map(),"##);
          [cfg_line, match_line].map(|s| acc.push_str(&s));
          acc.push('\n');
          acc
        },
      )
      .tap_mut(|buf| buf.push_str("  _ => \"\",\n}}"))
      .pipe(Ok)
  }
}

#[cfg(test)]
mod tests {
  use glossa_shared::display::puts;

  use super::*;
  use crate::{Visibility, generator::dbg_generator::new_generator};

  #[ignore]
  #[test]
  fn test_output_router_map() -> io::Result<()> {
    new_generator()
      .with_visibility(Visibility::Pub)
      .output_router_for_match_fns(MapType::Regular, false)?
      .pipe_ref(puts)
      .pipe(Ok)
  }
}
