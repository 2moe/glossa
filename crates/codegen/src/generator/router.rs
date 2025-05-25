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
  pub fn output_router_for_match_fns(
    &self,
    map_type: MapType,
    contains_map_name: bool,
  ) -> io::Result<String> {
    let header = format!(
      "use super::*;\n\
    pub const fn map(language: &[u8], {opt_parm}key: &[u8]) -> &'static str {{
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
