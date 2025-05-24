pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktiveer agtergrond"#####,
    b"base_name" => r#####"Basisnaam van "Highlight-Map""#####,
    b"bincode_suffix" => r#####"bincode-lêeragtervoegsel"#####,
    b"custom_syntax_set" => r#####"Aangepaste sintaksisstel-lêer"#####,
    b"custom_theme_set" => r#####"Aangepaste temastel-lêer"#####,
    b"display_config_dir" => r#####"Vertoon glossa se opstellinggids"#####,
    b"dsl_suffix" => r#####"DSL-lêeragtervoegsel (standaard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Swartlysmodus: Moet nie taal-ID's inisialiseer nie"#####
    }
    b"exclude_map_names" => r#####"Moet nie kaartname in lys inisialiseer nie"#####,
    b"include_languages" => {
      r#####"Witlysmodus: Inisialiseer slegs taal-ID's in lys"#####
    }
    b"include_map_names" => r#####"Inisialiseer slegs kaartname in lys"#####,
    b"input" => r#####"Bronkatalogus vir lokaliseringhulpbronne"#####,
    b"list_all_syntaxes" => r#####"Vertoon alle sintaksname en uitbreidings"#####,
    b"list_all_themes" => r#####"Vertoon alle temaname"#####,
    b"mod_prefix" => r#####"mod-lêervoorvoegsel (standaard "l10n_")"#####,
    b"outdir" => r#####"Uitvoergids"#####,
    b"output_bincode" => {
      r#####"Genereer aparte bincode-lêers vir verskillende tale"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksporteer alle tale se bincode na een lêer"#####
    }
    b"output_locales_fn" => r#####"Eksporteer all_locales-funksie"#####,
    b"output_match_fn" => {
      r#####"Genereer Rust-lêers met match-uitdrukkings vir tale"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolideer alle data in een match-funksie (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funksie met taalnaam as sleutel"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match-funksie met gekombineerde sleutel (taal + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Gebruik slegs map_key as sleutel (sonder map_name)"#####
    }
    b"output_phf" => r#####"Genereer phf-kaartfunksies per taal"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombineer alle phf-kaarte in een funksie"#####
    }
    b"output_phf_by_key" => {
      r#####"phf-kaart met gewone string-sleutels (nie TupleKey)"#####
    }
    b"output_ron" => r#####"Eksporteer string in RON-formaat"#####,
    b"suffix" => r#####"Nuwe "Highlight-Map"-agtervoegsel"#####,
    b"syntax_name" => r#####"Sintaksnaam"#####,
    b"theme_name" => r#####"Temanaam"#####,
    b"true_color" => r#####"24-bis ware kleur"#####,
    b"visibility" => r#####"Sigbaarheid van gegenereerde kode"#####,
    _ => "",
  }
}
