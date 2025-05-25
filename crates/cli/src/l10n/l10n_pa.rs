pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"ਬੈਕਗ੍ਰਾਊਂਡ ਸਮਰੱਥ ਕਰੋ"#####,
    b"base_name" => r#####"ਮੂਲ "ਹਾਈਲਾਈਟ-ਮੈਪ" ਦਾ ਨਾਮ"#####,
    b"bincode_suffix" => r#####"bincode ਫਾਇਲ ਪਿਛੇਤਰ"#####,
    b"custom_syntax_set" => r#####"ਕਸਟਮ ਸਿੰਟੈਕਸ ਸੈੱਟ ਫਾਇਲ"#####,
    b"custom_theme_set" => r#####"ਕਸਟਮ ਥੀਮ ਸੈੱਟ ਫਾਇਲ"#####,
    b"display_config_dir" => r#####"glossa ਦੀ ਕਨਫਿਗਰੇਸ਼ਨ ਡਾਇਰੈਕਟਰੀ ਦਿਖਾਓ"#####,
    b"dsl_suffix" => r#####"DSL ਫਾਇਲ ਪਿਛੇਤਰ (ਡਿਫੌਲਟ ".dsl")"#####,
    b"exclude_languages" => r#####"ਬਲੈਕਲਿਸਟ ਮੋਡ: ਸੂਚੀ ਵਾਲੇ ਭਾਸ਼ਾ ID ਇਨੀਸ਼ੀਏਟ ਨਹੀਂ ਹੋਣ"#####,
    b"exclude_map_names" => r#####"ਸੂਚੀ ਵਾਲੇ ਮੈਪ ਨਾਮ ਇਨੀਸ਼ੀਏਟ ਨਹੀਂ ਹੋਣ"#####,
    b"include_languages" => r#####"ਵ੍ਹਾਈਟਲਿਸਟ ਮੋਡ: ਸੂਚੀ ਵਾਲੇ ਭਾਸ਼ਾ ID ਹੀ ਇਨੀਸ਼ੀਏਟ ਹੋਣ"#####,
    b"include_map_names" => r#####"ਸੂਚੀ ਵਾਲੇ ਮੈਪ ਨਾਮ ਹੀ ਇਨੀਸ਼ੀਏਟ ਹੋਣ"#####,
    b"input" => r#####"ਲੋਕਲਾਈਜ਼ੇਸ਼ਨ ਸਰੋਤਾਂ ਦੀ ਸਰੋਤ ਡਾਇਰੈਕਟਰੀ"#####,
    b"list_all_syntaxes" => r#####"ਸਾਰੇ ਸਿੰਟੈਕਸ ਨਾਮ ਅਤੇ ਐਕਸਟੈਨਸ਼ਨ ਦਿਖਾਓ"#####,
    b"list_all_themes" => r#####"ਸਾਰੇ ਥੀਮ ਨਾਮ ਦਿਖਾਓ"#####,
    b"mod_prefix" => r#####"mod ਫਾਇਲ ਪ੍ਰੀਫਿਕਸ (ਡਿਫੌਲਟ "l10n_")"#####,
    b"outdir" => r#####"ਆਉਟਪੁੱਟ ਡਾਇਰੈਕਟਰੀ"#####,
    b"output_bincode" => r#####"ਵੱਖ-ਵੱਖ ਭਾਸ਼ਾਵਾਂ ਲਈ ਅਲੱਗ bincode ਫਾਇਲਾਂ ਬਣਾਓ"#####,
    b"output_bincode_all_in_one" => {
      r#####"ਸਾਰੀਆਂ ਭਾਸ਼ਾਵਾਂ ਦੇ bincode ਇੱਕੋ ਫਾਇਲ ਵਿੱਚ ਐਕਸਪੋਰਟ ਕਰੋ"#####
    }
    b"output_locales_fn" => r#####"all_locales ਫੰਕਸ਼ਨ ਐਕਸਪੋਰਟ ਕਰੋ"#####,
    b"output_match_fn" => r#####"ਰੱਸਟ ਕੋਡ ਫਾਇਲਾਂ (match ਐਕਸਪ੍ਰੈਸ਼ਨ ਫੰਕਸ਼ਨ ਸਮੇਤ) ਬਣਾਓ"#####,
    b"output_match_fn_all_in_one" => {
      r#####"ਸਾਰੇ ਡੇਟਾ ਨੂੰ ਇੱਕ match ਫੰਕਸ਼ਨ ਵਿੱਚ ਐਕਸਪੋਰਟ ਕਰੋ (ਸਟ੍ਰਿੰਗ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ਭਾਸ਼ਾ ਨਾਮ ਨਾਲ ਕੁੰਜੀ ਵਾਲੀ match ਫੰਕਸ਼ਨ"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ਕੁੰਜੀ (ਭਾਸ਼ਾ+map_key) ਨਾਲ match ਫੰਕਸ਼ਨ"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"ਕੇਵਲ map_key ਨੂੰ ਕੁੰਜੀ ਵਜੋਂ ਵਰਤੋਂ (map_name ਨਾਲ ਨਹੀਂ)"#####
    }
    b"output_phf" => r#####"ਭਾਸ਼ਾਵਾਂ ਲਈ ਅਲੱਗ phf ਮੈਪ ਫੰਕਸ਼ਨ ਬਣਾਓ"#####,
    b"output_phf_all_in_one" => r#####"ਸਾਰੇ phf ਮੈਪ ਇੱਕ ਫੰਕਸ਼ਨ ਵਿੱਚ ਐਕਸਪੋਰਟ ਕਰੋ"#####,
    b"output_phf_without_map_name" => r#####"ਸਧਾਰਨ ਸਟ੍ਰਿੰਗ ਕੁੰਜੀਆਂ ਵਾਲੇ phf ਮੈਪ (TupleKey ਨਹੀਂ)"#####,
    b"output_ron" => r#####"RON ਫਾਰਮੈਟ ਵਿੱਚ ਸਟ੍ਰਿੰਗ ਐਕਸਪੋਰਟ ਕਰੋ"#####,
    b"suffix" => r#####"ਨਵੇਂ "ਹਾਈਲਾਈਟ-ਮੈਪ" ਦਾ ਪਿਛੇਤਰ"#####,
    b"syntax_name" => r#####"ਸਿੰਟੈਕਸ ਨਾਮ"#####,
    b"theme_name" => r#####"ਥੀਮ ਨਾਮ"#####,
    b"true_color" => r#####"24-ਬਿੱਟ ਅਸਲੀ ਰੰਗ"#####,
    b"visibility" => r#####"ਜਨਰੇਟ ਕੋਡ ਦੀ ਦ੍ਰਿਸ਼ਟੀਗੋਚਰਤਾ"#####,
    _ => "",
  }
}
