pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ativar fundo"#####,
    b"base_name" => r#####"Nome base do "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufixo do arquivo bincode"#####,
    b"custom_syntax_set" => {
      r#####"Arquivo de conjunto de sintaxe personalizado"#####
    }
    b"custom_theme_set" => r#####"Arquivo de conjunto de temas personalizado"#####,
    b"display_config_dir" => {
      r#####"Mostrar diretório de configuração do glossa"#####
    }
    b"dsl_suffix" => r#####"Sufixo do arquivo DSL (padrão ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modo lista negra: Não inicializar IDs de idioma na lista"#####
    }
    b"exclude_map_names" => r#####"Não inicializar nomes de mapas na lista"#####,
    b"include_languages" => {
      r#####"Modo lista branca: Inicializar apenas IDs de idioma na lista"#####
    }
    b"include_map_names" => r#####"Inicializar apenas nomes de mapas na lista"#####,
    b"input" => r#####"Diretório fonte de recursos de localização"#####,
    b"list_all_syntaxes" => {
      r#####"Mostrar todos os nomes de sintaxe e extensões"#####
    }
    b"list_all_themes" => r#####"Mostrar todos os nomes de temas"#####,
    b"mod_prefix" => r#####"Prefixo do arquivo mod (padrão "l10n_")"#####,
    b"outdir" => r#####"Diretório de saída"#####,
    b"output_bincode" => {
      r#####"Gerar arquivos bincode separados para idiomas diferentes"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportar todos os bincodes em um único arquivo"#####
    }
    b"output_locales_fn" => r#####"Exportar função all_locales"#####,
    b"output_match_fn" => {
      r#####"Gerar arquivos Rust com funções match para idiomas"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Exportar todos os dados em uma função match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Função match usando nome do idioma como chave"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Função match com chave composta (idioma+map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Usar apenas map_key como chave (sem map_name)"#####
    }
    b"output_phf" => r#####"Gerar funções phf map separadas para idiomas"#####,
    b"output_phf_all_in_one" => {
      r#####"Combinar todos os phf maps em uma função"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map com chaves de string simples (não TupleKey)"#####
    }
    b"output_ron" => r#####"Exportar string no formato RON"#####,
    b"suffix" => r#####"Novo sufixo para "Highlight-Map""#####,
    b"syntax_name" => r#####"Nome da sintaxe"#####,
    b"theme_name" => r#####"Nome do tema"#####,
    b"true_color" => r#####"Cores verdadeiras de 24 bits"#####,
    b"visibility" => r#####"Visibilidade do código gerado"#####,
    _ => "",
  }
}
