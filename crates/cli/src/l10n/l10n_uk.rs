pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Активувати підтримку фону"#####,
    b"base_name" => r#####"Базова назва "Highlight-Map""#####,
    b"bincode_suffix" => r#####"суфікс bincode-файлів"#####,
    b"custom_syntax_set" => r#####"Файл користувацького набору синтаксисів"#####,
    b"custom_theme_set" => r#####"Файл користувацького набору тем"#####,
    b"display_config_dir" => r#####"Відобразити каталог конфігурації glossa"#####,
    b"dsl_suffix" => r#####"суфікс DSL-файлів (за замовчуванням ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Режим чорного списку: виключити зазначені мовні ID"#####
    }
    b"exclude_map_names" => r#####"Виключити зазначені map_names"#####,
    b"include_languages" => {
      r#####"Режим білого списку: ініціалізуються лише зазначені мовні ID"#####
    }
    b"include_map_names" => r#####"Ініціалізуються лише зазначені map_names"#####,
    b"input" => r#####"Вихідний каталог локалізаційних ресурсів"#####,
    b"list_all_syntaxes" => {
      r#####"Відображати всі назви синтаксисів з розширеннями"#####
    }
    b"list_all_themes" => r#####"Відображати всі назви тем"#####,
    b"mod_prefix" => r#####"префікс mod-файлів (за замовчуванням "l10n_")"#####,
    b"outdir" => r#####"Каталог виводу"#####,
    b"output_bincode" => r#####"Генерувати окремі bincode-файли для різних мов"#####,
    b"output_bincode_all_in_one" => {
      r#####"Експортувати bincode всіх мов в один файл"#####
    }
    b"output_locales_fn" => r#####"Експортувати функцію all_locales"#####,
    b"output_match_fn" => {
      r#####"Генерувати окремі Rust-файли з match-виразами для кожної мови"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Об'єднати всі мовні дані в одну match-функцію (рядок)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-функція з назвою мови як ключем"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-функція з ключем (мова + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Аналогічно output_match_fn, але використовує лише map_key (без map_name)"#####
    }
    b"output_phf" => r#####"Генерувати окремі phf-map функції для кожної мови"#####,
    b"output_phf_all_in_one" => r#####"Об'єднати всі phf-map в одну функцію"#####,
    b"output_phf_without_map_name" => {
      r#####"phf-map зі звичайними рядковими ключами (не TupleKey)"#####
    }
    b"output_ron" => r#####"Експортувати дані у форматі RON"#####,
    b"suffix" => r#####"Суфікс для нових Highlight-Map"#####,
    b"syntax_name" => r#####"Назва синтаксису"#####,
    b"theme_name" => r#####"Назва теми"#####,
    b"true_color" => r#####"24-бітний справжній колір"#####,
    b"visibility" => r#####"Видимість згенерованого коду"#####,
    _ => "",
  }
}
