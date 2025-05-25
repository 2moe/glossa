pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Активирај позадина"#####,
    b"base_name" => r#####"Основно име за "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Наставка за bincode датотека"#####,
    b"custom_syntax_set" => r#####"Прилагодена синтаксна датотека"#####,
    b"custom_theme_set" => r#####"Прилагодена датотека за теми"#####,
    b"display_config_dir" => {
      r#####"Прикажи го конфигурацискиот директориум на Glossa"#####
    }
    b"dsl_suffix" => r#####"Наставка за DSL датотека (стандардно ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Режим на црна листа: Не ги иницијализирај ID на јазиците од листата"#####
    }
    b"exclude_map_names" => {
      r#####"Не ги иницијализирај имињата на карти од листата"#####
    }
    b"include_languages" => {
      r#####"Режим на бела листа: Иницијализирај ги само ID на јазиците од листата"#####
    }
    b"include_map_names" => {
      r#####"Иницијализирај ги само имињата на карти од листата"#####
    }
    b"input" => r#####"Изворен директориум за локализациски ресурси"#####,
    b"list_all_syntaxes" => {
      r#####"Прикажи ги сите имиња на синтакси и екстензии"#####
    }
    b"list_all_themes" => r#####"Прикажи ги сите имиња на теми"#####,
    b"mod_prefix" => r#####"Префикс за mod датотека (стандардно "l10n_")"#####,
    b"outdir" => r#####"Излезен директориум"#####,
    b"output_bincode" => {
      r#####"Генерирај посебни bincode датотеки за различни јазици"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Експортирај ги сите bincode во една датотека"#####
    }
    b"output_locales_fn" => r#####"Експортирај ја функцијата all_locales"#####,
    b"output_match_fn" => {
      r#####"Генерирај Rust кодни датотеки со match функции за јазици"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Експортирај ги сите податоци во една match функција (стринг)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match функција со име на јазик како клуч"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match функција со комбиниран клуч (јазик + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Користи само map_key како клуч (без map_name)"#####
    }
    b"output_phf" => r#####"Генерирај посебни phf map функции за јазици"#####,
    b"output_phf_all_in_one" => r#####"Спој ги сите phf карти во една функција"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map со обични стринг клучеви (не TupleKey)"#####
    }
    b"output_ron" => r#####"Експортирај стринг во RON формат"#####,
    b"suffix" => r#####"Нова наставка за "Highlight-Map""#####,
    b"syntax_name" => r#####"Име на синтакса"#####,
    b"theme_name" => r#####"Име на тема"#####,
    b"true_color" => r#####"24-битна вистинска боја"#####,
    b"visibility" => r#####"Видливост на генерираниот код"#####,
    _ => "",
  }
}
