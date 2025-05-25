pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"I-enable ang background"#####,
    b"base_name" => r#####"Pangalan ng base Highlight-Map"#####,
    b"bincode_suffix" => r#####"Suffix ng bincode file"#####,
    b"custom_syntax_set" => r#####"Custom syntax set file"#####,
    b"custom_theme_set" => r#####"Custom theme set file"#####,
    b"display_config_dir" => r#####"Ipakita ang config directory ng glossa"#####,
    b"dsl_suffix" => r#####"Suffix ng DSL file (default: ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Blacklist mode: Hindi i-initialize ang nakalista na language IDs"#####
    }
    b"exclude_map_names" => {
      r#####"Hindi i-initialize ang nakalista na map_names"#####
    }
    b"include_languages" => {
      r#####"Whitelist mode: I-initialize lang ang nakalista na language IDs"#####
    }
    b"include_map_names" => {
      r#####"I-initialize lang ang nakalista na map_names"#####
    }
    b"input" => r#####"Source directory ng localization resources"#####,
    b"list_all_syntaxes" => {
      r#####"Ipakita ang lahat ng syntax name at extensions"#####
    }
    b"list_all_themes" => r#####"Ipakita ang lahat ng theme name"#####,
    b"mod_prefix" => r#####"Prefix ng mod file (default: "l10n_")"#####,
    b"outdir" => r#####"Direktoryo ng Output"#####,
    b"output_bincode" => {
      r#####"Gumawa ng hiwalay na bincode file para sa bawat wika"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Ilagay ang lahat ng bincode sa iisang file"#####
    }
    b"output_locales_fn" => r#####"Ilabas ang all_locales function"#####,
    b"output_match_fn" => {
      r#####"Gumawa ng Rust code file na may match expression para sa bawat wika"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Isama ang lahat ng data sa iisang match function (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match function na may language name bilang key"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match function na may kombinasyong key (language + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Katulad ng output_match_fn pero map_key lang ang key (walang map_name)"#####
    }
    b"output_phf" => {
      r#####"Gumawa ng hiwalay na phf map function para sa bawat wika"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Pagsamasamahin ang lahat ng phf map sa iisang function"#####
    }
    b"output_phf_without_map_name" => {
      r#####"Phf map na may regular string key (hindi TupleKey)"#####
    }
    b"output_ron" => r#####"Ilabas bilang RON-formatted string"#####,
    b"suffix" => r#####"Suffix ng bagong Highlight-Map"#####,
    b"syntax_name" => r#####"Pangalan ng syntax"#####,
    b"theme_name" => r#####"Pangalan ng theme"#####,
    b"true_color" => r#####"24-bit True Color"#####,
    b"visibility" => r#####"Visibility ng nabuong code"#####,
    _ => "",
  }
}
