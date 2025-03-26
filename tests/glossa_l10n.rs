use std::{fs, io};

use glossa_codegen::{Generator, L10nResources, Visibility, generator::MapType};

#[ignore]
#[test]
fn init_glossa_l10n_data() -> io::Result<()> {
  let content = Generator::default()
    .with_resources(L10nResources::new("locales").with_include_map_names(&["error"]))
    .with_visibility(Visibility::Pub)
    .output_match_fn_all_in_one(MapType::Regular)?;

  println!("{content}");
  fs::write("crates/l10n/src/data.rs", content)
  // Ok(())
}
