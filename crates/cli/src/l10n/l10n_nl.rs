pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Achtergrond inschakelen"#####,
    b"base_name" => r#####"Basisnaam van "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode-bestandsextensie"#####,
    b"custom_syntax_set" => r#####"Aangepast syntaxisset-bestand"#####,
    b"custom_theme_set" => r#####"Aangepast themaset-bestand"#####,
    b"display_config_dir" => r#####"Toon configuratiemap van Glossa"#####,
    b"dsl_suffix" => r#####"DSL-bestandsextensie (standaard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Blacklist-modus: Initialiseer geen taal-ID's in de lijst"#####
    }
    b"exclude_map_names" => r#####"Initialiseer geen mapnamen in de lijst"#####,
    b"include_languages" => {
      r#####"Whitelist-modus: Initialiseer alleen taal-ID's in de lijst"#####
    }
    b"include_map_names" => r#####"Initialiseer alleen mapnamen in de lijst"#####,
    b"input" => r#####"Bronmap voor gelokaliseerde bronnen"#####,
    b"list_all_syntaxes" => r#####"Toon alle syntaxismen en extensies"#####,
    b"list_all_themes" => r#####"Toon alle thema's"#####,
    b"mod_prefix" => r#####"Mod-bestandsvoorvoegsel (standaard "l10n_")"#####,
    b"outdir" => r#####"Uitvoermap"#####,
    b"output_bincode" => {
      r#####"Genereer aparte bincode-bestanden voor verschillende talen"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exporteer alle bincodes naar één bestand"#####
    }
    b"output_locales_fn" => r#####"Exporteer all_locales-functie"#####,
    b"output_match_fn" => {
      r#####"Genereer Rust-bestanden met match-expressies per taal"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolideer alle data in één match-functie (tekenreeks)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-functie met taalnaam als sleutel"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match-functie met gecombineerde sleutel (taal + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Gebruik alleen map_key als sleutel (zonder map_name)"#####
    }
    b"output_phf" => r#####"Genereer PHF-mapfuncties per taal"#####,
    b"output_phf_all_in_one" => {
      r#####"Consolideer alle PHF-maps in één functie"#####
    }
    b"output_phf_by_key" => {
      r#####"PHF-map met standaard tekenreekssleutels (geen TupleKey)"#####
    }
    b"output_ron" => r#####"Exporteer tekenreeks in RON-formaat"#####,
    b"suffix" => r#####"Nieuw achtervoegsel voor "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntaxismaam"#####,
    b"theme_name" => r#####"Themanaam"#####,
    b"true_color" => r#####"24-bits truecolor"#####,
    b"visibility" => r#####"Zichtbaarheid van gegenereerde code"#####,
    _ => "",
  }
}
