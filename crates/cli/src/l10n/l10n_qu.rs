pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Wañunanpaq llamk'achiy"#####,
    b"base_name" => r#####"Tiksi "Highlight-Map" sutin"#####,
    b"bincode_suffix" => r#####"bincode willay qipa"#####,
    b"custom_syntax_set" => r#####"Kikinmanta ruwasqa sintaxis wakichi"#####,
    b"custom_theme_set" => r#####"Kikinmanta ruwasqa tema wakichi"#####,
    b"display_config_dir" => r#####"Glossa configuración qututa rikuchiy"#####,
    b"dsl_suffix" => r#####"DSL willay qipa (sapallanta ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Yana lista: Listapi kaq simi ID mana qallarin"#####
    }
    b"exclude_map_names" => r#####"Listapi kaq map_names mana qallarin"#####,
    b"include_languages" => {
      r#####"Yuraq lista: Listapi kaq simi ID-kunalla qallarin"#####
    }
    b"include_map_names" => r#####"Listapi kaq map_names-kunalla qallarin"#####,
    b"input" => r#####"Localización qallariy qutukuna"#####,
    b"list_all_syntaxes" => r#####"Llapa sintaxis sutinkunata rikuchiy"#####,
    b"list_all_themes" => r#####"Llapa tema sutinkunata rikuchiy"#####,
    b"mod_prefix" => r#####"mod willaw qallariy (sapallanta "l10n_")"#####,
    b"outdir" => r#####"Lluqsiq qillqana"#####,
    b"output_bincode" => r#####"Simikunapaq sapalla bincode willayta ruway"#####,
    b"output_bincode_all_in_one" => {
      r#####"Llapa simikunapa bincoden huk willayman apay"#####
    }
    b"output_locales_fn" => r#####"All_locales funcionta apay"#####,
    b"output_match_fn" => {
      r#####"Rust code willayta ruway (match expresionniyuq) simikunapaq"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Llapa simi datakunata huk match funcionman (qillqa)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funcionpi simi sutitaq llamk'achiy"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match funcionpi simi suti + map_key llamk'achiy"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"map_key-lla llamk'achiy (map_name mana)"#####
    }
    b"output_phf" => r#####"PHF map funcionkunata simikunapaq ruway"#####,
    b"output_phf_all_in_one" => r#####"Llapa phf mapkuna huk funcionman"#####,
    b"output_phf_without_map_name" => r#####"PHF map kaq stringwan (mana TupleKey)"#####,
    b"output_ron" => r#####"RON formato stringta apay"#####,
    b"suffix" => r#####"Musuq Highlight-Map qipan"#####,
    b"syntax_name" => r#####"Simi kamachiy sutin"#####,
    b"theme_name" => r#####"Tema sutin"#####,
    b"true_color" => r#####"24-bit chiqap k'illim"#####,
    b"visibility" => r#####"Rurasqa codepa rikchakuynin"#####,
    _ => "",
  }
}
