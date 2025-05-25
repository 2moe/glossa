pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktivera bakgrund"#####,
    b"base_name" => r#####"Basnamn för "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode-filsuffix"#####,
    b"custom_syntax_set" => r#####"Anpassad syntaxsamlingsfil"#####,
    b"custom_theme_set" => r#####"Anpassad temafilsamling"#####,
    b"display_config_dir" => r#####"Visa Glossas konfigurationskatalog"#####,
    b"dsl_suffix" => r#####"DSL-filsuffix (standard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Svartlistningsläge: Initiera inte språk-ID:n i listan"#####
    }
    b"exclude_map_names" => r#####"Initiera inte map-namn i listan"#####,
    b"include_languages" => {
      r#####"Vitlistningsläge: Endast initiera språk-ID:n i listan"#####
    }
    b"include_map_names" => r#####"Endast initiera map-namn i listan"#####,
    b"input" => r#####"Källkatalog för lokaliseringsresurser"#####,
    b"list_all_syntaxes" => r#####"Visa alla syntaxnamn och tillägg"#####,
    b"list_all_themes" => r#####"Visa alla temanamn"#####,
    b"mod_prefix" => r#####"Prefix för mod-fil (standard "l10n_")"#####,
    b"outdir" => r#####"Utdatakatalog"#####,
    b"output_bincode" => {
      r#####"Generera separata bincode-filer för olika språk"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportera alla språks bincode till en fil"#####
    }
    b"output_locales_fn" => r#####"Exportera all_locales-funktionen"#####,
    b"output_match_fn" => {
      r#####"Generera Rust-kodfiler med match-uttryck för olika språk"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Samla all data i en match-funktion (sträng)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funktion med språknamn som nyckel"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-funktion med sammansatt nyckel (språk + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Använd endast map_key som nyckel (exkluderar map_name)"#####
    }
    b"output_phf" => r#####"Generera phf-map-funktioner per språk"#####,
    b"output_phf_all_in_one" => r#####"Samla alla phf-mappar i en funktion"#####,
    b"output_phf_without_map_name" => {
      r#####"phf-map med vanliga strängnycklar (inte TupleKey)"#####
    }
    b"output_ron" => r#####"Exportera sträng i RON-format"#####,
    b"suffix" => r#####"Suffix för nygenererad "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntaxnamn"#####,
    b"theme_name" => r#####"Temanamn"#####,
    b"true_color" => r#####"24-bitars sann färg"#####,
    b"visibility" => r#####"Synlighet för genererad kod"#####,
    _ => "",
  }
}
