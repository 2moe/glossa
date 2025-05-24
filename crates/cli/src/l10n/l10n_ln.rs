pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kokómisa fond"#####,
    b"base_name" => r#####"Nkombo ya libanda ya "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Bincode sufixe ya fisyé"#####,
    b"custom_syntax_set" => r#####"Fisyé ya syntax personnalisé"#####,
    b"custom_theme_set" => r#####"Fisyé ya thème personnalisé"#####,
    b"display_config_dir" => r#####"Komɔnisa dosíye ya configuration glossa"#####,
    b"dsl_suffix" => r#####"Suffixe fisyé DSL (par défaut ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode liste noir: Kopɛsí initialiser ID ndinga o list"#####
    }
    b"exclude_map_names" => r#####"Kopɛsí initialiser nkombo ya map o list"#####,
    b"include_languages" => {
      r#####"Mode liste blanc: Initialiser kaka ID ya ndinga o list"#####
    }
    b"include_map_names" => r#####"Initialiser kaka nkombo ya map o list"#####,
    b"input" => r#####"Dosíye source ya resourse ya lokalisasion"#####,
    b"list_all_syntaxes" => {
      r#####"Komɔnisa nkombo ya syntax nyɔnsɔ mpé extension"#####
    }
    b"list_all_themes" => r#####"Komɔnisa nkombo ya thème nyɔnsɔ"#####,
    b"mod_prefix" => r#####"Préfixe ya fisyé mod (par défaut "l10n_")"#####,
    b"outdir" => r#####"Dosíye ya bokɔtisi"#####,
    b"output_bincode" => {
      r#####"Kokela fisyé bincode indépendant na ndinga nyɔnsɔ"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Kobimisa bincode ya ndinga nyɔnsɔ na fisyé moko"#####
    }
    b"output_locales_fn" => r#####"Kobimisa fonction all_locales"#####,
    b"output_match_fn" => {
      r#####"Kokela fisyé Rust na fonction match na ndinga nyɔnsɔ"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kobimisa makambo nyɔnsɔ na fonction match moko (chaine)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonction match na nkombo ya ndinga lokola cle"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Fonction match na cle ya combinaison (ndinga + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Kosálela map_key kaka lokola cle (map_name tê)"#####
    }
    b"output_phf" => r#####"Kokela fonction phf map indépendant na ndinga"#####,
    b"output_phf_all_in_one" => {
      r#####"Kobimisa phf map nyɔnsɔ na fonction moko"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map na cle ya chaine simple (tô TupleKey)"#####
    }
    b"output_ron" => r#####"Kobimisa chaine na format RON"#####,
    b"suffix" => r#####"Suffixe ya sika ya "Highlight-Map""#####,
    b"syntax_name" => r#####"Nkombo ya syntax"#####,
    b"theme_name" => r#####"Nkombo ya thème"#####,
    b"true_color" => r#####"Langilangi ya 24-bit"#####,
    b"visibility" => r#####"Komɔnisi ya kode ya kobongwama"#####,
    _ => "",
  }
}
