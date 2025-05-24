pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Activa Fundum"#####,
    b"base_name" => r#####"Nomen Basile "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffixum Bincode Fasciculi"#####,
    b"custom_syntax_set" => r#####"Fasciculus Syntaxeos Custom"#####,
    b"custom_theme_set" => r#####"Fasciculus Thematum Custom"#####,
    b"display_config_dir" => {
      r#####"Monstrare Directorum Configurationis Glossa"#####
    }
    b"dsl_suffix" => r#####"Suffixum DSL Fasciculi (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modus Nigrum Indicem: Non Initia ID Linguarum in Indice"#####
    }
    b"exclude_map_names" => r#####"Non Initia Nomina Maparum in Indice"#####,
    b"include_languages" => {
      r#####"Modus Album Indicem: Initia ID Linguarum in Indice"#####
    }
    b"include_map_names" => r#####"Initia Nomina Maparum in Indice"#####,
    b"input" => r#####"Directorum Fontis Locorum"#####,
    b"list_all_syntaxes" => {
      r#####"Monstrare Omnia Nomina Syntaxeos cum Extensionibus"#####
    }
    b"list_all_themes" => r#####"Monstrare Omnia Nomina Thematum"#####,
    b"mod_prefix" => r#####"Praefixum Mod Fasciculi (default "l10n_")"#####,
    b"outdir" => r#####"Directorum Outputi"#####,
    b"output_bincode" => {
      r#####"Generare Fasciculos Bincode Separatos pro Linguis Diversis"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportare Omnes Bincode in Unum Fasciculum"#####
    }
    b"output_locales_fn" => r#####"Exportare Functionem all_locales"#####,
    b"output_match_fn" => {
      r#####"Generare Fasciculos Rust cum Functionibus Match pro Linguis"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolidare Omnes Data in Functionem Match Unam (Fila)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Functio Match cum Nomine Linguae pro Clave"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Functio Match cum Clave Composita (Lingua + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Simile output_match_fn, sed cum Clave map_key tantum"#####
    }
    b"output_phf" => r#####"Generare Functiones Map PHF pro Linguis Diversis"#####,
    b"output_phf_all_in_one" => {
      r#####"Consolidare Omnes Maps PHF in Unam Functionem"#####
    }
    b"output_phf_by_key" => r#####"Map PHF cum Clavibus Filorum (Non TupleKey)"#####,
    b"output_ron" => r#####"Exportare Filum in Formato RON"#####,
    b"suffix" => r#####"Suffixum Novi "Highlight-Map""#####,
    b"syntax_name" => r#####"Nomen Syntaxeos"#####,
    b"theme_name" => r#####"Nomen Thematis"#####,
    b"true_color" => r#####"Verum Colorem 24-bit"#####,
    b"visibility" => r#####"Visibilitas Codicis Generati"#####,
    _ => "",
  }
}
