pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Activar fondo"#####,
    b"base_name" => r#####"Nome base do "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufixo de ficheiro bincode"#####,
    b"custom_syntax_set" => {
      r#####"Ficheiro de conxunto de sintaxe personalizado"#####
    }
    b"custom_theme_set" => r#####"Ficheiro de conxunto de temas personalizado"#####,
    b"display_config_dir" => {
      r#####"Mostrar directorio de configuración de glossa"#####
    }
    b"dsl_suffix" => r#####"Sufixo de ficheiro DSL (por defecto ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modo lista negra: Non inicializar IDs de lingua na lista"#####
    }
    b"exclude_map_names" => r#####"Non inicializar nomes de mapa na lista"#####,
    b"include_languages" => {
      r#####"Modo lista branca: Inicializar só IDs de lingua na lista"#####
    }
    b"include_map_names" => r#####"Inicializar só nomes de mapa na lista"#####,
    b"input" => r#####"Directorio fonte de recursos localizados"#####,
    b"list_all_syntaxes" => {
      r#####"Mostrar todos os nomes de sintaxe e extensións"#####
    }
    b"list_all_themes" => r#####"Mostrar todos os nomes de temas"#####,
    b"mod_prefix" => r#####"Prefixo de ficheiro mod (por defecto "l10n_")"#####,
    b"outdir" => r#####"Directorio de saída"#####,
    b"output_bincode" => {
      r#####"Xerar ficheiros bincode independentes para cada lingua"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportar todos os bincodes a un ficheiro"#####
    }
    b"output_locales_fn" => r#####"Exportar función all_locales"#####,
    b"output_match_fn" => {
      r#####"Xerar ficheiros Rust con funcións match para cada lingua"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolidar todos os datos nunha función match (cadea)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Función match con nome de lingua como chave"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Función match con chave combinada (lingua + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Usar só map_key como chave (excluíndo map_name)"#####
    }
    b"output_phf" => r#####"Xerar funcións phf map independentes por lingua"#####,
    b"output_phf_all_in_one" => {
      r#####"Consolidar todos os phf maps nunha función"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map con chaves de cadea simples (non TupleKey)"#####
    }
    b"output_ron" => r#####"Exportar cadea en formato RON"#####,
    b"suffix" => r#####"Sufixo novo do "Highlight-Map""#####,
    b"syntax_name" => r#####"Nome da sintaxe"#####,
    b"theme_name" => r#####"Nome do tema"#####,
    b"true_color" => r#####"Cor real de 24 bits"#####,
    b"visibility" => r#####"Visibilidade do código xerado"#####,
    _ => "",
  }
}
