pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktivér baggrund"#####,
    b"base_name" => r#####""Highlight-Map" basisnavn"#####,
    b"bincode_suffix" => r#####"Bincode-filendelse"#####,
    b"custom_syntax_set" => r#####"Brugerdefineret syntakssæt-fil"#####,
    b"custom_theme_set" => r#####"Brugerdefineret temasæt-fil"#####,
    b"display_config_dir" => r#####"Vis Glossas konfigurationsmappe"#####,
    b"dsl_suffix" => r#####"DSL-filendelse (standard ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Sortlistetilstand: Initialisér ikke sprog-ID'er på listen"#####
    }
    b"exclude_map_names" => r#####"Initialisér ikke map-navne på listen"#####,
    b"include_languages" => {
      r#####"Hvidlistetilstand: Initialisér kun sprog-ID'er på listen"#####
    }
    b"include_map_names" => r#####"Initialisér kun map-navne på listen"#####,
    b"input" => r#####"Kildemappe for lokaliseringsressourcer"#####,
    b"list_all_syntaxes" => r#####"Vis alle syntaksnavne og -udvidelser"#####,
    b"list_all_themes" => r#####"Vis alle temanavne"#####,
    b"mod_prefix" => r#####"Mod-filpræfiks (standard "l10n_")"#####,
    b"outdir" => r#####"Outputmappe"#####,
    b"output_bincode" => {
      r#####"Generer separate bincode-filer for forskellige sprog"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksporter alle sprogs bincode til én fil"#####
    }
    b"output_locales_fn" => r#####"Eksporter all_locales-funktionen"#####,
    b"output_match_fn" => {
      r#####"Generer Rust-kodefiler med match-udtryk (funktioner)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Eksporter alle data til én match-funktion (streng)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funktion med sprognavn som nøgle"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-funktion med kombineret nøgle (sprog+map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Brug kun map_key som nøgle (ekskluder map_name)"#####
    }
    b"output_phf" => r#####"Generer separate phf-map-funktioner per sprog"#####,
    b"output_phf_all_in_one" => r#####"Kombiner alle phf-maps i én funktion"#####,
    b"output_phf_without_map_name" => {
      r#####"phf-map med almindelige strengnøgler (ikke TupleKey)"#####
    }
    b"output_ron" => r#####"Eksporter streng i RON-format"#####,
    b"suffix" => r#####""Highlight-Map" ny endelse"#####,
    b"syntax_name" => r#####"Syntaksnavn"#####,
    b"theme_name" => r#####"Temanavn"#####,
    b"true_color" => r#####"24-bit ægte farve"#####,
    b"visibility" => r#####"Synlighed for genereret kode"#####,
    _ => "",
  }
}
