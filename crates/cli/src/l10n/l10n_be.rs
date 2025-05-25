pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Уключыць фон"#####,
    b"base_name" => r#####"Базавая назва "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Суфікс файла bincode"#####,
    b"custom_syntax_set" => r#####"Карыстацкі файл сінтаксісу"#####,
    b"custom_theme_set" => r#####"Карыстацкі файл тэм"#####,
    b"display_config_dir" => r#####"Адлюстраваць канфігурацыйны каталог glossa"#####,
    b"dsl_suffix" => r#####"Суфікс DSL-файла (па змаўчанні ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Рэжым чорнага спісу: Не ініцыялізаваць ID моў са спісу"#####
    }
    b"exclude_map_names" => r#####"Не ініцыялізаваць назвы map са спісу"#####,
    b"include_languages" => {
      r#####"Рэжым белага спісу: Ініцыялізаваць толькі ID моў са спісу"#####
    }
    b"include_map_names" => r#####"Ініцыялізаваць толькі назвы map са спісу"#####,
    b"input" => r#####"Зыходны каталог лакалізацыйных рэсурсаў"#####,
    b"list_all_syntaxes" => {
      r#####"Адлюстраваць усе назвы сінтаксісаў і пашырэнні"#####
    }
    b"list_all_themes" => r#####"Адлюстраваць усе назвы тэм"#####,
    b"mod_prefix" => r#####"Прэфікс файла mod (па змаўчанні "l10n_")"#####,
    b"outdir" => r#####"Каталог вываду"#####,
    b"output_bincode" => r#####"Ствараць асобныя файлы bincode для розных моў"#####,
    b"output_bincode_all_in_one" => {
      r#####"Экспартаваць усе bincode ў адзін файл"#####
    }
    b"output_locales_fn" => r#####"Экспартаваць функцыю all_locales"#####,
    b"output_match_fn" => r#####"Ствараць Rust-файлы з функцыямі match для моў"#####,
    b"output_match_fn_all_in_one" => {
      r#####"Аб'яднаць усе дадзеныя ў адну функцыю match (радок)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Функцыя match з назвай мовы як ключом"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Функцыя match з камбінаваным ключом (мова + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Выкарыстоўваць толькі map_key як ключ (без map_name)"#####
    }
    b"output_phf" => r#####"Ствараць асобныя phf-функцыі для моў"#####,
    b"output_phf_all_in_one" => r#####"Аб'яднаць усе phf-map у адну функцыю"#####,
    b"output_phf_without_map_name" => {
      r#####"phf-map з звычайнымі радковымі ключамі (не TupleKey)"#####
    }
    b"output_ron" => r#####"Экспартаваць радок у фармаце RON"#####,
    b"suffix" => r#####"Новы суфікс для "Highlight-Map""#####,
    b"syntax_name" => r#####"Назва сінтаксісу"#####,
    b"theme_name" => r#####"Назва тэмы"#####,
    b"true_color" => r#####"24-бітны сапраўдны колер"#####,
    b"visibility" => r#####"Бачнасць згенераванага кода"#####,
    _ => "",
  }
}
