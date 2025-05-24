pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Фоннары холбооһун"#####,
    b"base_name" => r#####""Highlight-Map" төрүт аата"#####,
    b"bincode_suffix" => r#####"Bincode билэ суффикса"#####,
    b"custom_syntax_set" => r#####"Анал синтаксис хомуурун билэтэ"#####,
    b"custom_theme_set" => r#####"Анал тема хомуурун билэтэ"#####,
    b"display_config_dir" => r#####"Glossa конфиг паапкатын көрдөр"#####,
    b"dsl_suffix" => r#####"DSL билэ суффикса (сүрүннээх ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Хара испииһэк: Испииһэк тыл ID-ларын инициализациялаама"#####
    }
    b"exclude_map_names" => r#####"Испииһэк хаарта ааттарын инициализациялаама"#####,
    b"include_languages" => {
      r#####"Үрүҥ испииһэк: Испииһэккэ баар тыл ID-ларын эрэ инициализациялаа"#####
    }
    b"include_map_names" => {
      r#####"Испииһэккэ баар хаарта ааттарын эрэ инициализациялаа"#####
    }
    b"input" => r#####"Локализация ресурстарын төрүт паапката"#####,
    b"list_all_syntaxes" => {
      r#####"Бары синтаксис ааттарын уонна эбии ааттарын көрдөр"#####
    }
    b"list_all_themes" => r#####"Бары тема ааттарын көрдөр"#####,
    b"mod_prefix" => r#####"Mod билэ префикса (сүрүннээх "l10n_")"#####,
    b"outdir" => r#####"Таһаарыы паапката"#####,
    b"output_bincode" => {
      r#####"Тус-туспа тылларга араас bincode билэлэри оҥоруу"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Бары тыллар bincode-ларын биир билэҕэ таһаарыы"#####
    }
    b"output_locales_fn" => r#####"all_locales функцияны таһаарыы"#####,
    b"output_match_fn" => {
      r#####"Rust коду билэлэрин оҥоруу (match функциялаах)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Бары дааннайы биир match функцияҕа таһаарыы (строка)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Тыл аата ключ курдук match функция"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Холбоһуктуу ключ (тыл аата + map_key) match функция"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key-ны эрэ ключ курдук тутун (map_name суох)"#####
    }
    b"output_phf" => r#####"Тылларга phf хаарта функцияларын оҥоруу"#####,
    b"output_phf_all_in_one" => {
      r#####"Бары phf хаарталары биир функцияҕа таһаарыы"#####
    }
    b"output_phf_by_key" => {
      r#####"Эрээссийэ строка ключтаах phf хаарта (TupleKey буолбатах)"#####
    }
    b"output_ron" => r#####"RON формаатыгар строканы таһаарыы"#####,
    b"suffix" => r#####""Highlight-Map" саҥа суффикса"#####,
    b"syntax_name" => r#####"Синтаксис аата"#####,
    b"theme_name" => r#####"Тема аата"#####,
    b"true_color" => r#####"24-бит истиэннээх өҥ"#####,
    b"visibility" => r#####"Үөскээбит коду көстүүтэ"#####,
    _ => "",
  }
}
