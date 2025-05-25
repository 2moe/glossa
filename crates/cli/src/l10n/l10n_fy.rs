pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Eftergrûn ynskeakelje"#####,
    b"base_name" => r#####"Basisnamme fan "Highlight-Map""#####,
    b"bincode_suffix" => r#####"bincode-efterheaksel"#####,
    b"custom_syntax_set" => r#####"Oanpaste syntaksis set bestân"#####,
    b"custom_theme_set" => r#####"Oanpast tema set bestân"#####,
    b"display_config_dir" => r#####"Toane glossa ynstellingsmap"#####,
    b"dsl_suffix" => r#####"DSL-efterheaksel (standert ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Swarte list modus: Talen yn list net inisjalisearje"#####
    }
    b"exclude_map_names" => r#####"Map-nammen yn list net inisjalisearje"#####,
    b"include_languages" => {
      r#####"Wite list modus: Allinnich talen yn list inisjalisearje"#####
    }
    b"include_map_names" => r#####"Allinnich map-nammen yn list inisjalisearje"#####,
    b"input" => r#####"Boarnemap foar lokalisearjebronnen"#####,
    b"list_all_syntaxes" => r#####"Toane alle syntaksisnammen mei útwreidingen"#####,
    b"list_all_themes" => r#####"Toane alle tema-nammen"#####,
    b"mod_prefix" => r#####"mod-foarheaksel (standert "l10n_")"#####,
    b"outdir" => r#####"Útfier map"#####,
    b"output_bincode" => {
      r#####"Genearje aparte bincode-triemmen foar ferskillende talen"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksportearje alle bincodes nei ien triem"#####
    }
    b"output_locales_fn" => r#####"Eksportearje all_locales-funksje"#####,
    b"output_match_fn" => {
      r#####"Meitsje Rust-koade mei match-funksjes per taal"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kombinearje alle gegevens yn ien match-funksje (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funksje mei taalnamme as kaai"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-funksje mei kombinaasje kaai (taalnamme + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Brûk allinnich map_key as kaai (sûnder map_name)"#####
    }
    b"output_phf" => r#####"Meitsje phf-map funksjes per taal"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombinearje alle phf-maps yn ien funksje"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf-map mei gewoane string-kaaien (gjin TupleKey)"#####
    }
    b"output_ron" => r#####"Eksportearje string yn RON-formaat"#####,
    b"suffix" => r#####"Nij generearre efterheaksel foar "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntaksisnamme"#####,
    b"theme_name" => r#####"Tema-namme"#####,
    b"true_color" => r#####"24-bit wiere kleur"#####,
    b"visibility" => r#####"Sichtberens fan generearre koade"#####,
    _ => "",
  }
}
