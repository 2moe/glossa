pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktiver bakgrunn"#####,
    b"base_name" => r#####"Grunnnavn for "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode-filsuffiks"#####,
    b"custom_syntax_set" => r#####"Tilpasset syntakssett-fil"#####,
    b"custom_theme_set" => r#####"Tilpasset temasett-fil"#####,
    b"display_config_dir" => r#####"Vis konfigurasjonskatalog for glossa"#####,
    b"dsl_suffix" => r#####"DSL-filsuffiks (standard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Svartliste-modus: Ikke initialiser språk-IDer i listen"#####
    }
    b"exclude_map_names" => r#####"Ikke initialiser map-navn i listen"#####,
    b"include_languages" => {
      r#####"Hvitliste-modus: Kun initialiser språk-IDer i listen"#####
    }
    b"include_map_names" => r#####"Kun initialiser map-navn i listen"#####,
    b"input" => r#####"Katalog for lokaliseringsressurser"#####,
    b"list_all_syntaxes" => r#####"Vis alle syntaksnavn og utvidelser"#####,
    b"list_all_themes" => r#####"Vis alle temanavn"#####,
    b"mod_prefix" => r#####"Mod-prefiks for fil (standard "l10n_")"#####,
    b"outdir" => r#####"Utdatakatalog"#####,
    b"output_bincode" => {
      r#####"Generer separate bincode-filer for forskjellige språk"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksporter alle språks bincode til én fil"#####
    }
    b"output_locales_fn" => r#####"Eksporter all_locales-funksjonen"#####,
    b"output_match_fn" => {
      r#####"Generer Rust-filer med match-uttrykksfunksjoner per språk"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolider alle data i én match-funksjon (streng)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funksjon med språknavn som nøkkel"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-funksjon med kombinasjonsnøkkel (språk + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Bruk kun map_key som nøkkel (ekskluder map_name)"#####
    }
    b"output_phf" => r#####"Generer phf-kartfunksjoner per språk"#####,
    b"output_phf_all_in_one" => r#####"Konsolider alle phf-kart i én funksjon"#####,
    b"output_phf_without_map_name" => {
      r#####"phf-kart med vanlige strengnøkler (ikke TupleKey)"#####
    }
    b"output_ron" => r#####"Eksporter streng i RON-format"#####,
    b"suffix" => r#####"Suffiks for nygenerert "Highlight-Map""#####,
    b"syntax_name" => r#####"Syntaksnavn"#####,
    b"theme_name" => r#####"Temanavn"#####,
    b"true_color" => r#####"24-biters ekte farger"#####,
    b"visibility" => r#####"Synlighet for generert kode"#####,
    _ => "",
  }
}
