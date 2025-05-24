pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Активировать фон"#####,
    b"base_name" => r#####"Базовое название "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Суффикс файла bincode"#####,
    b"custom_syntax_set" => r#####"Пользовательский набор синтаксиса"#####,
    b"custom_theme_set" => r#####"Пользовательский набор тем"#####,
    b"display_config_dir" => {
      r#####"Показать конфигурационную директорию Glossa"#####
    }
    b"dsl_suffix" => r#####"Суффикс DSL-файла (по умолчанию ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Режим чёрного списка: Не инициализировать ID из списка"#####
    }
    b"exclude_map_names" => r#####"Не инициализировать названия карт из списка"#####,
    b"include_languages" => {
      r#####"Режим белого списка: Инициализировать только ID из списка"#####
    }
    b"include_map_names" => {
      r#####"Инициализировать только названия карт из списка"#####
    }
    b"input" => r#####"Исходная директория локализованных ресурсов"#####,
    b"list_all_syntaxes" => r#####"Показать все названия синтаксисов"#####,
    b"list_all_themes" => r#####"Показать все названия тем"#####,
    b"mod_prefix" => r#####"Префикс файла mod (по умолчанию "l10n_")"#####,
    b"outdir" => r#####"Выходная директория"#####,
    b"output_bincode" => {
      r#####"Генерировать отдельные bincode-файлы для разных языков"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Экспортировать все bincode в один файл"#####
    }
    b"output_locales_fn" => r#####"Экспортировать функцию all_locales"#####,
    b"output_match_fn" => {
      r#####"Генерировать Rust-файлы с match-функциями для языков"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Объединить все данные в одну match-функцию (строка)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-функция с названием языка как ключом"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match-функция с комбинированным ключом (язык+map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Использовать только map_key как ключ (без map_name)"#####
    }
    b"output_phf" => r#####"Генерировать phf-карты для разных языков"#####,
    b"output_phf_all_in_one" => r#####"Объединить все phf-карты в одну функцию"#####,
    b"output_phf_by_key" => {
      r#####"phf-карта с обычными строковыми ключами (не TupleKey)"#####
    }
    b"output_ron" => r#####"Экспортировать строку в формате RON"#####,
    b"suffix" => r#####"Новый суффикс для "Highlight-Map""#####,
    b"syntax_name" => r#####"Название синтаксиса"#####,
    b"theme_name" => r#####"Название темы"#####,
    b"true_color" => r#####"24-битный истинный цвет"#####,
    b"visibility" => r#####"Видимость сгенерированного кода"#####,
    _ => "",
  }
}
