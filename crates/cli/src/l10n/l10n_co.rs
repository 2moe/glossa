pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Attivà u fondu"#####,
    b"base_name" => r#####"Nome di basa di "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffissu di u schedariu bincode"#####,
    b"custom_syntax_set" => r#####"Schedariu di sintassi persunalizatu"#####,
    b"custom_theme_set" => r#####"Schedariu di temi persunalizati"#####,
    b"display_config_dir" => {
      r#####"Mustrà u cartulare di cunfigurazione Glossa"#####
    }
    b"dsl_suffix" => r#####"Suffissu di u schedariu DSL (predefinitu ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modu lista nera: Micca inizializà l'ID di lingue in lista"#####
    }
    b"exclude_map_names" => r#####"Micca inizializà i nomi di mappe in lista"#####,
    b"include_languages" => {
      r#####"Modu lista bianca: Inizializà solu l'ID di lingue in lista"#####
    }
    b"include_map_names" => r#####"Inizializà solu i nomi di mappe in lista"#####,
    b"input" => r#####"Cartulare di risorse lucalizate"#####,
    b"list_all_syntaxes" => {
      r#####"Mustrà tutti i nomi di sintassi è estensioni"#####
    }
    b"list_all_themes" => r#####"Mustrà tutti i nomi di temi"#####,
    b"mod_prefix" => r#####"Prefissu di u schedariu mod (predefinitu "l10n_")"#####,
    b"outdir" => r#####"Cartulare di Esportazione"#####,
    b"output_bincode" => {
      r#####"Generà schedarii bincode separati per ogni lingua"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Esportà tutti i bincode in un schedariu"#####
    }
    b"output_locales_fn" => r#####"Esportà a funzione all_locales"#####,
    b"output_match_fn" => {
      r#####"Generà schedarii Rust cù funzioni match per ogni lingua"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Esportà tutti i dati in una funzione match (stringa)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funzione match cù u nome di a lingua cum'è chjave"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Funzione match cù chjave cumbinata (lingua + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Utilizà solu map_key cum'è chjave (senza map_name)"#####
    }
    b"output_phf" => r#####"Generà funzioni phf map separati per lingue"#####,
    b"output_phf_all_in_one" => r#####"Cumbinà tutti i phf map in una funzione"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map cù chjavi stringa simplice (micca TupleKey)"#####
    }
    b"output_ron" => r#####"Esportà stringa in formatu RON"#####,
    b"suffix" => r#####"Suffissu novu di "Highlight-Map""#####,
    b"syntax_name" => r#####"Nome di a sintassi"#####,
    b"theme_name" => r#####"Nome di u tema"#####,
    b"true_color" => r#####"24-bit culore veru"#####,
    b"visibility" => r#####"Visibilità di u codice generatu"#####,
    _ => "",
  }
}
