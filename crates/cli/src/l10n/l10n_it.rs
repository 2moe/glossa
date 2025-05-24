pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Abilita sfondo"#####,
    b"base_name" => r#####"Nome base "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffisso file bincode"#####,
    b"custom_syntax_set" => r#####"File set sintassi personalizzato"#####,
    b"custom_theme_set" => r#####"File set temi personalizzato"#####,
    b"display_config_dir" => r#####"Mostra directory configurazione Glossa"#####,
    b"dsl_suffix" => r#####"Suffisso file DSL (predefinito ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modalità blacklist: Non inizializza ID lingue elencate"#####
    }
    b"exclude_map_names" => r#####"Non inizializza nomi mappa elencati"#####,
    b"include_languages" => {
      r#####"Modalità whitelist: Inizializza solo ID lingue elencate"#####
    }
    b"include_map_names" => r#####"Inizializza solo nomi mappa elencati"#####,
    b"input" => r#####"Directory sorgente risorse localizzate"#####,
    b"list_all_syntaxes" => r#####"Mostra tutti i nomi sintassi con estensioni"#####,
    b"list_all_themes" => r#####"Mostra tutti i nomi temi"#####,
    b"mod_prefix" => r#####"Prefisso file mod (predefinito "l10n_")"#####,
    b"outdir" => r#####"Directory di output"#####,
    b"output_bincode" => {
      r#####"Genera file bincode separati per lingue diverse"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Esporta tutti i bincode in un unico file"#####
    }
    b"output_locales_fn" => r#####"Esporta funzione all_locales"#####,
    b"output_match_fn" => {
      r#####"Genera file Rust con funzioni match per ogni lingua"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolida tutti i dati in una funzione match (stringa)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funzione match con nome lingua come chiave"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Funzione match con chiave combinata (lingua + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Usa solo map_key come chiave (esclude map_name)"#####
    }
    b"output_phf" => r#####"Genera funzioni phf map per lingue diverse"#####,
    b"output_phf_all_in_one" => {
      r#####"Unifica tutte le phf map in una funzione"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map con chiavi stringa semplici (non TupleKey)"#####
    }
    b"output_ron" => r#####"Esporta come stringa formato RON"#####,
    b"suffix" => r#####"Suffisso per nuove Highlight-Map"#####,
    b"syntax_name" => r#####"Nome sintassi"#####,
    b"theme_name" => r#####"Nome tema"#####,
    b"true_color" => r#####"Colore vero a 24 bit"#####,
    b"visibility" => r#####"Visibilità del codice generato"#####,
    _ => "",
  }
}
