pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Goyo background"#####,
    b"base_name" => r#####"Nying map mar "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Yien bincode"#####,
    b"custom_syntax_set" => r#####"Fichier syntax ma itimo"#####,
    b"custom_theme_set" => r#####"Fichier theme ma itimo"#####,
    b"display_config_dir" => r#####"Nyiso diri config mar glossa"#####,
    b"dsl_suffix" => r#####"Yien DSL (ma chiel ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Par Black List: Kik ket ID kod molil ma e list"#####
    }
    b"exclude_map_names" => r#####"Kik ket nying map ma e list"#####,
    b"include_languages" => r#####"Par White List: Ket ID kod molil ma e list"#####,
    b"include_map_names" => r#####"Ket nying map ma e list"#####,
    b"input" => r#####"Diri moting'o gi lok"#####,
    b"list_all_syntaxes" => r#####"Nyiso nying syntax gi extension duto"#####,
    b"list_all_themes" => r#####"Nyiso nying theme duto"#####,
    b"mod_prefix" => r#####"Nyime mod (ma chiel "l10n_")"#####,
    b"outdir" => r#####"Diri Yalo"#####,
    b"output_bincode" => r#####"Ket fichier bincode machielgi kode duto"#####,
    b"output_bincode_all_in_one" => r#####"Goy bincode duto e fichier achiel"#####,
    b"output_locales_fn" => r#####"Goy function all_locales"#####,
    b"output_match_fn" => r#####"Ket fichier Rust gi function match machielgi"#####,
    b"output_match_fn_all_in_one" => {
      r#####"Goy data duto e function match achiel (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Function match gi nying kod molil"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Function match gi nying kod gi map_key"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Tiyo map_key kodi (map_name onge)"#####
    }
    b"output_phf" => r#####"Ket phf map machielgi kode duto"#####,
    b"output_phf_all_in_one" => r#####"Goy phf map duto e function achiel"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map gi string kodi (kama TupleKey)"#####
    }
    b"output_ron" => r#####"Goy string e format RON"#####,
    b"suffix" => r#####"Yien map manyien mar "Highlight-Map""#####,
    b"syntax_name" => r#####"Nying syntax"#####,
    b"theme_name" => r#####"Nying theme"#####,
    b"true_color" => r#####"Rangi ma 24-bit"#####,
    b"visibility" => r#####"Nen mar code ma oyier"#####,
    _ => "",
  }
}
