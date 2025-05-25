pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"ᱯᱟᱹᱪᱷᱞᱟᱹ ᱪᱟᱹᱞᱩ ᱥᱮ ᱵᱟᱝ"#####,
    b"base_name" => r#####""Highlight-Map" ᱨᱮᱭᱟᱜ ᱢᱩᱞ ᱧᱩᱛᱩᱢ"#####,
    b"bincode_suffix" => r#####"bincode ᱨᱮᱭᱟᱜ ᱯᱷᱟᱭᱤᱞ ᱛᱟᱭᱚᱢᱛᱮ"#####,
    b"custom_syntax_set" => r#####"ᱠᱚᱥᱴᱚᱢ ᱥᱤᱱᱴᱟᱠᱥ ᱥᱮᱴ ᱯᱷᱟᱭᱤᱞ"#####,
    b"custom_theme_set" => r#####"ᱠᱚᱥᱴᱚᱢ ᱛᱷᱤᱢ ᱥᱮᱴ ᱯᱷᱟᱭᱤᱞ"#####,
    b"display_config_dir" => r#####"glossa ᱨᱮᱭᱟᱜ ᱠᱚᱱᱯᱷᱤᱜ ᱴᱷᱤᱠᱟᱹ ᱫᱮᱠᱷᱟᱣ"#####,
    b"dsl_suffix" => r#####"DSL ᱯᱷᱟᱭᱤᱞ ᱛᱟᱭᱚᱢᱛᱮ (ᱢᱩᱞ ᱛᱮ ".dsl")"#####,
    b"exclude_languages" => r#####"ᱠᱟᱞᱟ ᱞᱤᱥᱴ ᱢᱳᱰ: ᱞᱤᱥᱴ ᱯᱟᱹᱨᱥᱤ ID ᱵᱟᱝ ᱮᱦᱚᱵ"#####,
    b"exclude_map_names" => r#####"ᱞᱤᱥᱴ ᱨᱮᱭᱟᱜ ᱢᱮᱯ ᱧᱩᱛᱩᱢ ᱵᱟᱝ ᱮᱦᱚᱵ"#####,
    b"include_languages" => {
      r#####"ᱯᱮᱸᱰᱟ ᱞᱤᱥᱴ ᱢᱳᱰ: ᱞᱤᱥᱴ ᱨᱮᱭᱟᱜ ᱯᱟᱹᱨᱥᱤ ID ᱜᱮ ᱮᱦᱚᱵ"#####
    }
    b"include_map_names" => r#####"ᱞᱤᱥᱴ ᱨᱮᱭᱟᱜ ᱢᱮᱯ ᱧᱩᱛᱩᱢ ᱜᱮ ᱮᱦᱚᱵ"#####,
    b"input" => r#####"ᱞᱳᱠᱟᱞ ᱥᱟᱫᱷᱚᱱ ᱨᱮᱭᱟᱜ ᱢᱩᱞ ᱴᱷᱤᱠᱟᱹ"#####,
    b"list_all_syntaxes" => r#####"ᱡᱷᱚᱛᱚ ᱥᱤᱱᱴᱟᱠᱥ ᱧᱩᱛᱩᱢ ᱟᱨ ᱮᱠᱥᱴᱮᱱᱥᱚᱱ ᱫᱮᱠᱷᱟᱣ"#####,
    b"list_all_themes" => r#####"ᱡᱷᱚᱛᱚ ᱛᱷᱤᱢ ᱧᱩᱛᱩᱢ ᱫᱮᱠᱷᱟᱣ"#####,
    b"mod_prefix" => r#####"mod ᱯᱷᱟᱭᱤᱞ ᱢᱩᱪᱟᱹᱫ (ᱢᱩᱞ ᱛᱮ "l10n_")"#####,
    b"outdir" => r#####"ᱵᱟᱦᱨᱮ ᱴᱷᱤᱠᱟᱹ ᱰᱟᱭᱨᱮᱠᱴᱚᱨᱤ"#####,
    b"output_bincode" => {
      r#####"ᱵᱷᱮᱜᱟᱨ ᱯᱟᱹᱨᱥᱤ ᱞᱟᱹᱜᱤᱫ ᱵᱮᱼᱵᱷᱮᱡᱟᱨ bincode ᱯᱷᱟᱭᱤᱞ ᱵᱮᱱᱟᱣ"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"ᱡᱷᱚᱛᱚ ᱯᱟᱹᱨᱥᱤ ᱨᱮᱭᱟᱜ bincode ᱢᱤᱫ ᱯᱷᱟᱭᱤᱞ ᱨᱮ ᱚᱛᱟᱭᱚᱢ"#####
    }
    b"output_locales_fn" => r#####"all_locales ᱯᱷᱚᱝᱥᱚᱱ ᱚᱛᱟᱭᱚᱢ"#####,
    b"output_match_fn" => {
      r#####"ᱯᱟᱹᱨᱥᱤ ᱠᱚ ᱞᱟᱹᱜᱤᱫ Rust ᱠᱳᱰ ᱯᱷᱟᱭᱤᱞ ᱵᱮᱱᱟᱣ (match ᱮᱠᱥᱯᱨᱮᱥᱚᱱ ᱥᱟᱶ)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"ᱡᱷᱚᱛᱚ ᱰᱮᱴᱟ ᱢᱤᱫ match ᱯᱷᱚᱝᱥᱚᱱ ᱨᱮ ᱚᱛᱟᱭᱚᱢ (ᱥᱴᱨᱤᱝ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ᱯᱟᱹᱨᱥᱤ ᱧᱩᱛᱩᱢ ᱠᱤ ᱛᱮ ᱢᱤᱫ ᱢᱟᱪᱮᱛ ᱯᱷᱚᱝᱥᱚᱱ"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ᱯᱟᱹᱨᱥᱤ ᱧᱩᱛᱩᱢ+map_key ᱠᱤ ᱛᱮ ᱢᱤᱫ ᱢᱟᱪᱮᱛ ᱯᱷᱚᱝᱥᱚᱱ"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"map_key ᱛᱮ ᱠᱤ ᱵᱮᱱᱟᱣ (map_name ᱵᱟᱝ ᱥᱮᱞᱮᱫ)"#####
    }
    b"output_phf" => r#####"ᱯᱟᱹᱨᱥᱤ ᱠᱚ ᱞᱟᱹᱜᱤᱫ phf ᱢᱮᱯ ᱯᱷᱚᱝᱥᱚᱱ ᱵᱮᱱᱟᱣ"#####,
    b"output_phf_all_in_one" => r#####"ᱡᱷᱚᱛᱚ phf ᱢᱮᱯ ᱢᱤᱫ ᱯᱷᱚᱝᱥᱚᱱ ᱨᱮ ᱚᱛᱟᱭᱚᱢ"#####,
    b"output_phf_without_map_name" => {
      r#####"ᱥᱟᱫᱷᱟᱨᱚᱱ ᱥᱴᱨᱤᱝ ᱠᱤ ᱛᱮ phf ᱢᱮᱯ (TupleKey ᱵᱟᱝ)"#####
    }
    b"output_ron" => r#####"RON ᱯᱷᱚᱨᱢᱮᱴ ᱨᱮ ᱥᱴᱨᱤᱝ ᱚᱛᱟᱭᱚᱢ"#####,
    b"suffix" => r#####""Highlight-Map" ᱨᱮᱭᱟᱜ ᱱᱚᱶᱟ ᱛᱟᱭᱚᱢᱛᱮ"#####,
    b"syntax_name" => r#####"ᱥᱤᱱᱴᱟᱠᱥ ᱧᱩᱛᱩᱢ"#####,
    b"theme_name" => r#####"ᱛᱷᱤᱢ ᱧᱩᱛᱩᱢ"#####,
    b"true_color" => r#####"24-ᱵᱤᱴ ᱥᱟᱹᱨᱤ ᱨᱚᱝ"#####,
    b"visibility" => r#####"ᱵᱮᱱᱟᱣ ᱠᱳᱰ ᱧᱮᱞᱚᱜ"#####,
    _ => "",
  }
}
