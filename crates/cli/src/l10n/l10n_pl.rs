pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktywuj tło"#####,
    b"base_name" => r#####"Podstawowa nazwa "Highlight-Map""#####,
    b"bincode_suffix" => r#####"sufiks pliku bincode"#####,
    b"custom_syntax_set" => r#####"Niestandardowy zestaw składni"#####,
    b"custom_theme_set" => r#####"Niestandardowy zestaw motywów"#####,
    b"display_config_dir" => r#####"Pokaż katalog konfiguracyjny Glossa"#####,
    b"dsl_suffix" => r#####"sufiks pliku DSL (domyślnie ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Tryb czarnej listy: Nie inicjuj ID języków z listy"#####
    }
    b"exclude_map_names" => r#####"Nie inicjuj nazw map z listy"#####,
    b"include_languages" => {
      r#####"Tryb białej listy: Inicjuj tylko ID języków z listy"#####
    }
    b"include_map_names" => r#####"Inicjuj tylko nazwy map z listy"#####,
    b"input" => r#####"Katalog źródłowy zasobów lokalizacyjnych"#####,
    b"list_all_syntaxes" => {
      r#####"Wyświetl wszystkie nazwy składni i rozszerzenia"#####
    }
    b"list_all_themes" => r#####"Wyświetl wszystkie nazwy motywów"#####,
    b"mod_prefix" => r#####"prefiks pliku mod (domyślnie "l10n_")"#####,
    b"outdir" => r#####"Katalog wyjściowy"#####,
    b"output_bincode" => {
      r#####"Generuj osobne pliki bincode dla różnych języków"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksportuj wszystkie bincode do jednego pliku"#####
    }
    b"output_locales_fn" => r#####"Eksportuj funkcję all_locales"#####,
    b"output_match_fn" => {
      r#####"Generuj pliki kodu Rust z funkcjami match dla języków"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Eksportuj wszystkie dane do jednej funkcji match (ciąg znaków)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funkcja match z nazwą języka jako kluczem"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Funkcja match z kluczem złożonym (język + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Używaj tylko map_key jako klucza (bez map_name)"#####
    }
    b"output_phf" => r#####"Generuj osobne funkcje phf map dla języków"#####,
    b"output_phf_all_in_one" => {
      r#####"Scal wszystkie phf mapy w jednej funkcji"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map ze zwykłymi kluczami tekstowymi (nie TupleKey)"#####
    }
    b"output_ron" => r#####"Eksportuj ciąg w formacie RON"#####,
    b"suffix" => r#####"Nowy sufiks "Highlight-Map""#####,
    b"syntax_name" => r#####"Nazwa składni"#####,
    b"theme_name" => r#####"Nazwa motywu"#####,
    b"true_color" => r#####"24-bitowy prawdziwy kolor"#####,
    b"visibility" => r#####"Widoczność wygenerowanego kodu"#####,
    _ => "",
  }
}
