pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"habilitar fondo"#####,
    b"base_name" => r#####"nombre base de Highlight-Map"#####,
    b"bincode_suffix" => r#####"sufijo de archivo bincode"#####,
    b"custom_syntax_set" => r#####"archivo personalizado de sintaxis"#####,
    b"custom_theme_set" => r#####"archivo personalizado de temas"#####,
    b"display_config_dir" => {
      r#####"mostrar directorio de configuración de glossa"#####
    }
    b"dsl_suffix" => r#####"sufijo de archivo DSL (por defecto: ".dsl")"#####,
    b"exclude_languages" => r#####"modo lista negra: excluir IDs listados"#####,
    b"exclude_map_names" => r#####"excluir map_names listados"#####,
    b"include_languages" => {
      r#####"modo lista blanca: solo inicializar IDs listados"#####
    }
    b"include_map_names" => r#####"solo inicializar map_names listados"#####,
    b"input" => r#####"directorio fuente de recursos de localización"#####,
    b"list_all_syntaxes" => r#####"mostrar nombres de sintaxis y extensiones"#####,
    b"list_all_themes" => r#####"mostrar nombres de temas"#####,
    b"mod_prefix" => r#####"prefijo de archivo mod (por defecto: "l10n_")"#####,
    b"outdir" => r#####"directorio de salida"#####,
    b"output_bincode" => {
      r#####"generar archivos bincode independientes por idioma"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"consolidar todos los bincode en un solo archivo"#####
    }
    b"output_locales_fn" => r#####"generar función all_locales"#####,
    b"output_match_fn" => {
      r#####"generar archivos Rust con funciones match por idioma"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"unificar datos en función match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"función match con nombre de idioma como clave"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"función match con clave compuesta (idioma + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"solo usa map_key como clave (sin map_name)"#####
    }
    b"output_phf" => r#####"generar funciones phf map por idioma"#####,
    b"output_phf_all_in_one" => {
      r#####"unificar todas las phf map en una función"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map con claves string (no TupleKey)"#####
    }
    b"output_ron" => r#####"generar string en formato RON"#####,
    b"suffix" => r#####"sufijo para nuevas Highlight-Maps"#####,
    b"syntax_name" => r#####"nombre de sintaxis"#####,
    b"theme_name" => r#####"nombre de tema"#####,
    b"true_color" => r#####"24-bit color verdadero"#####,
    b"visibility" => r#####"visibilidad del código generado"#####,
    _ => "",
  }
}
