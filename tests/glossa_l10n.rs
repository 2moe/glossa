use std::{fs, path::PathBuf};

use anyhow::Result as AnyResult;
use compact_str::format_compact;
use glossa_codegen::{Generator, L10nResources, Visibility, generator::MapType};
use tap::Pipe;

#[ignore]
#[test]
fn init_glossa_l10n_data() -> AnyResult<()> {
  [
    ("error", None), //
    ("yes-no", Some("yes_no")),
  ]
  .into_iter()
  .map(write_contents)
  .try_for_each(|x| x)
}

fn write_contents((map_name, mod_name): (&str, Option<&str>)) -> AnyResult<()> {
  let include_maps = [map_name];
  let generator = Generator::default()
    .with_resources(
      L10nResources::new("locales").with_include_map_names(&include_maps),
    )
    .with_visibility(Visibility::Pub);

  let dir: PathBuf = {
    let mod_stem = mod_name.unwrap_or(map_name);
    format_compact!("crates/l10n/src/{mod_stem}")
  }
  .into();

  fs::create_dir_all(&dir)?;

  {
    let content = generator.output_match_fn_all_in_one(MapType::Regular)?;
    println!("{content}");
    let write_file = |file| fs::write(file, content);

    dir
      .join("matches.rs")
      .pipe(write_file)?
  }

  // {
  //   let content = "pub mod locale_registry;\npub mod matches;\n";
  //   let write_file = |file| fs::write(file, content);
  //   dir
  //     .join("mod.rs")
  //     .pipe(write_file)?
  // }

  {
    let all_locales = generator.output_locales_fn(MapType::Regular, true)?;
    println!("{all_locales}");
    let write_file = |file| fs::write(file, all_locales);

    dir
      .join("locale_registry.rs")
      .pipe(write_file)?
  }
  Ok(())
}
