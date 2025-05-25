pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Фонды қосу"#####,
    b"base_name" => r#####""Highlight-Map" негізгі атауы"#####,
    b"bincode_suffix" => r#####"bincode файл кенестіргіші"#####,
    b"custom_syntax_set" => r#####"Теңдеу жинағы файлы"#####,
    b"custom_theme_set" => r#####"Тақырып жинағы файлы"#####,
    b"display_config_dir" => r#####"Glossa конфигурация директориясын көрсету"#####,
    b"dsl_suffix" => r#####"DSL файл кенестіргіші (әдепкі ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Қара тізім режимі: Тізімдегі тіл ID-лерін инициализацияламау"#####
    }
    b"exclude_map_names" => {
      r#####"Тізімдегі карта атауларын инициализацияламау"#####
    }
    b"include_languages" => {
      r#####"Ақ тізім режимі: Тізімдегі тіл ID-лерін ғана инициализациялау"#####
    }
    b"include_map_names" => {
      r#####"Тізімдегі карта атауларын ғана инициализациялау"#####
    }
    b"input" => r#####"Локализация ресурстарының түпнұсқа директориясы"#####,
    b"list_all_syntaxes" => {
      r#####"Барлық синтаксис атаулары мен кеңейтулерін көрсету"#####
    }
    b"list_all_themes" => r#####"Барлық тақырып атауларын көрсету"#####,
    b"mod_prefix" => r#####"mod файл префиксі (әдепкі "l10n_")"#####,
    b"outdir" => r#####"Шығару директориясы"#####,
    b"output_bincode" => {
      r#####"Түрлі тілдер үшін бөлек bincode файлдарын жасау"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Барлық тілдердің bincode-ын бір файлға шығару"#####
    }
    b"output_locales_fn" => r#####"all_locales функциясын шығару"#####,
    b"output_match_fn" => {
      r#####"Rust кодының файлдарын жасау (match өрнектері бар функциялар)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Барлық деректерді бір match функциясына шығару (жол)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Тіл атауы кілт болатын match функциясы"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Қосылған кілт (тіл+map_key) бар match функциясы"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Тек map_key кілт ретінде (map_name жоқ)"#####
    }
    b"output_phf" => r#####"Әр тілге арналған бөлек phf карта функциялары"#####,
    b"output_phf_all_in_one" => {
      r#####"Барлық phf карталарды бір функцияға біріктіру"#####
    }
    b"output_phf_without_map_name" => {
      r#####"Қарапайым жол кілттері бар phf карта (TupleKey емес)"#####
    }
    b"output_ron" => r#####"RON форматында жолды шығару"#####,
    b"suffix" => r#####""Highlight-Map" жаңа кенестіргіші"#####,
    b"syntax_name" => r#####"Синтаксис атауы"#####,
    b"theme_name" => r#####"Тақырып атауы"#####,
    b"true_color" => r#####"24-биттік шынайы түс"#####,
    b"visibility" => r#####"Жасалған кодтың көрінуі"#####,
    _ => "",
  }
}
