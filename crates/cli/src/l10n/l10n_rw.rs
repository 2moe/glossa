pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kwinjiza inyuma"#####,
    b"base_name" => r#####"Izina ry''ishingiro rya "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Indangamugaragaro y'idosiye bincode"#####,
    b"custom_syntax_set" => r#####"Idosiye y'ibisobanuro byihariye"#####,
    b"custom_theme_set" => r#####"Idosiye y'inyandiko zihariye"#####,
    b"display_config_dir" => r#####"Kwerekana indangamuntu y'ibintu bya glossa"#####,
    b"dsl_suffix" => r#####"Indangamugaragaro y'idosiye DSL (Bisanzwe ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Uburyo gahunda mbi: ID z'indimi ziri murutonde ntabwo zishoboka"#####
    }
    b"exclude_map_names" => {
      r#####"Amazina ya map yiri murutonde ntabwo ari byemewe"#####
    }
    b"include_languages" => {
      r#####"Uburyo gahunda yera: ID z'indimi ziri murutonde gusa"#####
    }
    b"include_map_names" => r#####"Amazina ya map yiri murutonde gusa"#####,
    b"input" => r#####"Indangamuntu y'ingenzi y'ibikoresho by'ururimi"#####,
    b"list_all_syntaxes" => {
      r#####"Kwerekana amazina y'ibisobanuro n'ibinyurizo"#####
    }
    b"list_all_themes" => r#####"Kwerekana amazina y'inyandiko zose"#####,
    b"mod_prefix" => r#####"Indangamugaragaro y'idosiye mod (Bisanzwe "l10n_")"#####,
    b"outdir" => r#####"Indangamuntu y’ibyazamutungo"#####,
    b"output_bincode" => {
      r#####"Kubaka idosiye bincode zihariye ku ndimi zitandukanye"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Gusesa buri bincode y'indimi muri dosiye imwe"#####
    }
    b"output_locales_fn" => r#####"Gusesa fonction ya all_locales"#####,
    b"output_match_fn" => {
      r#####"Kubaka idosiye Rust zihariye hamwe n'imikorere ya match"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Gusesa byose muri fonction imwe ya match (umurongo)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonction ya match ifite izina ry'ururimi nk'urufunguzo"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fonction ya match ifite urufunguzo (izina + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Nka output_match_fn ariko ikoresha map_key nk'urufunguzo gusa"#####
    }
    b"output_phf" => r#####"Kubaka phf map zihariye ku ndimi"#####,
    b"output_phf_all_in_one" => r#####"Gusesa buri phf map muri fonction imwe"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map ifite imibare y'umurongo (Oya TupleKey)"#####
    }
    b"output_ron" => r#####"Gusesa umurongo wa format RON"#####,
    b"suffix" => r#####"Indangamugaragaro ya "Highlight-Map" nshya"#####,
    b"syntax_name" => r#####"Izina ry'ibisobanuro"#####,
    b"theme_name" => r#####"Izina ry'inyandiko"#####,
    b"true_color" => r#####"Amabara nyakuri 24-bit"#####,
    b"visibility" => r#####"Kugaragara kwa code yubatswe"#####,
    _ => "",
  }
}
