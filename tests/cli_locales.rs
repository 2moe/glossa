use std::fs;

use anyhow::Result as AnyResult;
use glossa::MiniStr;
use glossa_codegen::generator::to_lower_snake_case;
use glossa_shared::fmt_compact;

fn locale_names() -> Vec<MiniStr> {
  glossa_cli::all_locales()
    .iter()
    .map(|x| fmt_compact!("l10n_{x}",))
    .collect()
}

fn locale_mod_names() -> Vec<MiniStr> {
  glossa_cli::all_locales()
    .iter()
    .map(|x| fmt_compact!("l10n_{}", to_lower_snake_case(x)))
    .collect()
}

#[ignore]
#[test]
fn test_mod_names() {
  dbg!(locale_mod_names());
}

#[ignore]
#[test]
fn test_gen_cli_features() -> AnyResult<()> {
  let l10n_names = locale_names();

  let mut s = String::with_capacity(1024 * 8);

  s.push_str(&format!("{} = []\n", l10n_names.join(" = []\n")));

  let all = serde_json::to_string_pretty(&l10n_names)?;
  let l10n_all = format!("l10n_all = {all}");

  s.push_str(&l10n_all);

  fs::write("tmp.toml", s)?;

  Ok(())
}

#[ignore]
#[test]
fn test_gen_router_map() {
  let (feat_names, mod_names) = (locale_names(), locale_mod_names());
  assert_eq!(feat_names.len(), mod_names.len());

  let mut s = String::with_capacity(1024 * 16);

  s.push_str(
    r##"use super::*;

  pub const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match language {
  "##,
  );
  for ((feat_name, mod_name), language) in feat_names
    .iter()
    .zip(mod_names)
    .zip(glossa_cli::all_locales())
  {
    // eprintln!("name: {feat_name}, mod_name: {mod_name}, language: {language} ");

    let tmp = format!(
      r##"#[cfg(feature = "{feat_name}")]
      b"{language}" => {mod_name}::map(key),
    "##
    );
    s.push_str(&tmp)
  }

  s.push_str(
    r##"
    _ => "",
  }}
  "##,
  );

  println!("{s}");
  // fs::write("tmp.locale-router.rs", s).unwrap();
}
