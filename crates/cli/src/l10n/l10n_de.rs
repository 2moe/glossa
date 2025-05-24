pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Hintergrund aktivieren"#####,
    b"base_name" => r#####"Basisname der "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode-Dateiendung"#####,
    b"custom_syntax_set" => r#####"Benutzerdefinierte Syntax-Set-Datei"#####,
    b"custom_theme_set" => r#####"Benutzerdefiniertes Themen-Set"#####,
    b"display_config_dir" => r#####"Glossa-Konfigurationsverzeichnis anzeigen"#####,
    b"dsl_suffix" => r#####"DSL-Dateiendung (Standard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Blacklist-Modus: Sprach-IDs aus der Liste nicht initialisieren"#####
    }
    b"exclude_map_names" => {
      r#####"Map-Namen aus der Liste nicht initialisieren"#####
    }
    b"include_languages" => {
      r#####"Whitelist-Modus: Nur Sprach-IDs aus der Liste initialisieren"#####
    }
    b"include_map_names" => r#####"Nur Map-Namen aus der Liste initialisieren"#####,
    b"input" => r#####"Quellverzeichnis für Lokalisierungsressourcen"#####,
    b"list_all_syntaxes" => r#####"Alle Syntaxnamen und Erweiterungen anzeigen"#####,
    b"list_all_themes" => r#####"Alle Themennamen anzeigen"#####,
    b"mod_prefix" => r#####"Mod-Präfix (Standard "l10n_")"#####,
    b"outdir" => r#####"Ausgabeverzeichnis"#####,
    b"output_bincode" => {
      r#####"Separate Bincode-Dateien für verschiedene Sprachen generieren"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Alle Bincodes in einer Datei ausgeben"#####
    }
    b"output_locales_fn" => r#####"all_locales-Funktion exportieren"#####,
    b"output_match_fn" => {
      r#####"Rust-Code-Dateien mit Match-Funktionen für Sprachen erstellen"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Alle Daten in einer Match-Funktion ausgeben (Zeichenkette)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-Funktion mit Sprachnamen als Schlüssel"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match-Funktion mit kombiniertem Schlüssel (Sprache + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Nur map_key als Schlüssel verwenden (ohne map_name)"#####
    }
    b"output_phf" => {
      r#####"Separate PHF-Map-Funktionen für Sprachen generieren"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Alle PHF-Maps in einer Funktion zusammenführen"#####
    }
    b"output_phf_by_key" => {
      r#####"PHF-Map mit einfachen Zeichenkettenschlüsseln (kein TupleKey)"#####
    }
    b"output_ron" => r#####"Zeichenkette im RON-Format exportieren"#####,
    b"suffix" => r#####"Neue Endung für "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntaxname"#####,
    b"theme_name" => r#####"Themenname"#####,
    b"true_color" => r#####"24-Bit-Echtfarbe"#####,
    b"visibility" => r#####"Sichtbarkeit des generierten Codes"#####,
    _ => "",
  }
}
