pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktivizo sfondin"#####,
    b"base_name" => r#####"Emri Bazë i "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Shtesë e File Bincode"#####,
    b"custom_syntax_set" => r#####"File Set Sintakse i Personalizuar"#####,
    b"custom_theme_set" => r#####"File Set Temash i Personalizuar"#####,
    b"display_config_dir" => r#####"Shfaq Direktorinë e Konfigurimit të Glossa"#####,
    b"dsl_suffix" => r#####"Shtesë DSL File (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modalitet Lista e Zezë: Mos inicializo ID gjuhësh nga lista"#####
    }
    b"exclude_map_names" => r#####"Mos inicializo emrat e hartave nga lista"#####,
    b"include_languages" => {
      r#####"Modalitet Lista e Bardhë: Inicializo vetëm ID gjuhësh nga lista"#####
    }
    b"include_map_names" => r#####"Inicializo vetëm emrat e hartave nga lista"#####,
    b"input" => r#####"Direktoria Burimore e Burimeve të Lokalizuara"#####,
    b"list_all_syntaxes" => {
      r#####"Shfaq të gjitha emrat e sintaksës dhe zgjerimet"#####
    }
    b"list_all_themes" => r#####"Shfaq të gjitha emrat e temave"#####,
    b"mod_prefix" => r#####"Parashtesë Mod File (default "l10n_")"#####,
    b"outdir" => r#####"Direktoria e Daljes"#####,
    b"output_bincode" => {
      r#####"Gjenero file bincode të pavarura për gjuhë të ndryshme"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksporto të gjitha bincodet në një file"#####
    }
    b"output_locales_fn" => r#####"Eksporto funksionin all_locales"#####,
    b"output_match_fn" => {
      r#####"Gjenero file Rust me funksione match për gjuhë"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolido të gjitha të dhënat në një funksion match (varg)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funksion match me emrin e gjuhës si çelës"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Funksion match me çelës të kombinuar (gjuhë + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Përdor vetëm map_key si çelës (pa map_name)"#####
    }
    b"output_phf" => r#####"Gjenero funksione phf map për gjuhë të ndryshme"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombino të gjitha phf map në një funksion"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map me çelësa të zakonshëm vargu (jo TupleKey)"#####
    }
    b"output_ron" => r#####"Eksporto varg në formatin RON"#####,
    b"suffix" => r#####"Shtesa e Re e "Highlight-Map""#####,
    b"syntax_name" => r#####"Emri i Sintaksës"#####,
    b"theme_name" => r#####"Emri i Temës"#####,
    b"true_color" => r#####"Ngjyra e Vërtetë 24-bit"#####,
    b"visibility" => r#####"Dukshmëria e Kodit të Gjeneruar"#####,
    _ => "",
  }
}
