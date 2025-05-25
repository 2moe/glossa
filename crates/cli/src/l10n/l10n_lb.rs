pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Hannergrond aktivéieren"#####,
    b"base_name" => r#####"Basisnumm vum "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode-Fichier Suffix"#####,
    b"custom_syntax_set" => r#####"Personaliséiert Syntax-Set Fichier"#####,
    b"custom_theme_set" => r#####"Personaliséiert Theme-Set Fichier"#####,
    b"display_config_dir" => r#####"Weist d'Glossa Config-Verzeichnes"#####,
    b"dsl_suffix" => r#####"DSL-Fichier Suffix (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Schwaarz-Lëscht Modus: Initialiséiert keng Sprooch-IDs aus der Lëscht"#####
    }
    b"exclude_map_names" => r#####"Initialiséiert keng Map-Nimm aus der Lëscht"#####,
    b"include_languages" => {
      r#####"Whëss-Lëscht Modus: Initialiséiert nëmmen Sprooch-IDs aus der Lëscht"#####
    }
    b"include_map_names" => {
      r#####"Initialiséiert nëmmen Map-Nimm aus der Lëscht"#####
    }
    b"input" => r#####"Quellverzeichnes fir lokaliséiert Ressourcen"#####,
    b"list_all_syntaxes" => r#####"Weist all Syntax-Nimm an Erweiderungen"#####,
    b"list_all_themes" => r#####"Weist all Theema-Nimm"#####,
    b"mod_prefix" => r#####"Präfix fir Mod-Fichieren (default "l10n_")"#####,
    b"outdir" => r#####"Output-Verzeichnes"#####,
    b"output_bincode" => {
      r#####"Generéiert onofhängeg Bincode-Fichiere fir verschidde Sproochen"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"All Bincode-Fichieren an een Fichier exportéieren"#####
    }
    b"output_locales_fn" => r#####"Exportéiert all_locales Funktioun"#####,
    b"output_match_fn" => {
      r#####"Generéiert Rust-Code-Fichiere mat Match-Funktiounen pro Sprooch"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kombinéiert all Daten an eng Match-Funktioun (String)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-Funktioun mat Sprooche Numm als Schlëssel"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-Funktioun mat kombinéiertem Schlëssel (Sprooch + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Benotzt nëmmen map_key als Schlëssel (ouni map_name)"#####
    }
    b"output_phf" => r#####"Generéiert PHF-Map Funktiounen pro Sprooch"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombinéiert all PHF-Mappen an eng Funktioun"#####
    }
    b"output_phf_without_map_name" => {
      r#####"PHF-Map mat normale String-Schlësselen (keen TupleKey)"#####
    }
    b"output_ron" => r#####"Exportéiert String am RON-Format"#####,
    b"suffix" => r#####"Neit Suffix fir "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntax Numm"#####,
    b"theme_name" => r#####"Thema Numm"#####,
    b"true_color" => r#####"24-bit richteg Faarf"#####,
    b"visibility" => r#####"Sichtbarkeet vum generéierte Code"#####,
    _ => "",
  }
}
