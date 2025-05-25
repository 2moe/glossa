pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Activar fons"#####,
    b"base_name" => r#####"Nom base del "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufix de fitxer bincode"#####,
    b"custom_syntax_set" => r#####"Fitxer de sintaxi personalitzada"#####,
    b"custom_theme_set" => r#####"Fitxer de temes personalitzats"#####,
    b"display_config_dir" => {
      r#####"Mostrar directori de configuració de glossa"#####
    }
    b"dsl_suffix" => r#####"Sufix de fitxer DSL (per defecte ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode llista negra: No inicialitzar IDs d'idioma de la llista"#####
    }
    b"exclude_map_names" => r#####"No inicialitzar noms de mapa de la llista"#####,
    b"include_languages" => {
      r#####"Mode llista blanca: Inicialitzar només IDs d'idioma de la llista"#####
    }
    b"include_map_names" => {
      r#####"Inicialitzar només noms de mapa de la llista"#####
    }
    b"input" => r#####"Directori font de recursos de localització"#####,
    b"list_all_syntaxes" => {
      r#####"Mostrar tots els noms de sintaxi i extensions"#####
    }
    b"list_all_themes" => r#####"Mostrar tots els noms de temes"#####,
    b"mod_prefix" => r#####"Prefix de fitxer mod (per defecte "l10n_")"#####,
    b"outdir" => r#####"Directori de sortida"#####,
    b"output_bincode" => {
      r#####"Generar fitxers bincode independents per a cada idioma"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportar tots els bincode a un únic fitxer"#####
    }
    b"output_locales_fn" => r#####"Exportar funció all_locales"#####,
    b"output_match_fn" => {
      r#####"Generar fitxers Rust amb funcions match per idioma"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolidar dades en una funció match (cadena)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funció match amb nom d'idioma com a clau"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Funció match amb clau combinada (idioma + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Utilitzar només map_key com a clau (sense map_name)"#####
    }
    b"output_phf" => r#####"Generar funcions phf map per a diferents idiomes"#####,
    b"output_phf_all_in_one" => {
      r#####"Consolidar totes les phf map en una funció"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map amb claus de cadena simples (no TupleKey)"#####
    }
    b"output_ron" => r#####"Exportar cadena en format RON"#####,
    b"suffix" => r#####"Sufix del nou "Highlight-Map""#####,
    b"syntax_name" => r#####"Nom de la sintaxi"#####,
    b"theme_name" => r#####"Nom del tema"#####,
    b"true_color" => r#####"Color real de 24 bits"#####,
    b"visibility" => r#####"Visibilitat del codi generat"#####,
    _ => "",
  }
}
