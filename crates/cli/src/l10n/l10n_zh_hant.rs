pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"是否啟用背景"#####,
    b"base_name" => r#####"基礎"高亮Map"的名稱"#####,
    b"bincode_suffix" => r#####"bincode檔案字尾"#####,
    b"custom_syntax_set" => r#####"自定義語法集檔案"#####,
    b"custom_theme_set" => r#####"自定義主題集檔案"#####,
    b"display_config_dir" => r#####"顯示glossa的配置目錄"#####,
    b"dsl_suffix" => r#####"DSL 檔案字尾(預設 ".dsl")"#####,
    b"exclude_languages" => {
      r#####"黑名單模式。位於黑名單中的語言 id 不會被初始化"#####
    }
    b"exclude_map_names" => r#####"位於列表中的 map_names 不會被初始化"#####,
    b"include_languages" => {
      r#####"白名單模式。當其不為空時，只有位於列表中的語言 id 才會被初始化"#####
    }
    b"include_map_names" => {
      r#####"當其不為空時，只有位於列表中的 map_names 才會被初始化"#####
    }
    b"input" => r#####"本地化資源的源目錄"#####,
    b"list_all_syntaxes" => r#####"顯示所有語法名稱及其副檔名"#####,
    b"list_all_themes" => r#####"顯示所有主題名稱"#####,
    b"mod_prefix" => r#####"mod 檔案字首 (預設 "l10n_")"#####,
    b"outdir" => r#####"輸出的目錄"#####,
    b"output_bincode" => r#####"為不同的語言生成獨立的 bincode 檔案"#####,
    b"output_bincode_all_in_one" => {
      r#####"將所有語言的 bincode 輸出到同一個檔案"#####
    }
    b"output_locales_fn" => r#####"輸出 all_locales 函式"#####,
    b"output_match_fn" => {
      r#####"為不同的語言生成獨立的 rust 程式碼的檔案 (包含match表示式的函式)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"將所有語言的資料都輸出為一個match函式（字串）"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"將所有語言的資料輸出為同一個match 函式（字串），key 為語言名"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"將所有語言的資料輸出為同一個 match 函式（字串），key 為語言名和 map_key"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"類似於 output_match_fn，但生成的函式只以 map_key 作為 key, 不包含 map_name"#####
    }
    b"output_phf" => r#####"為不同語言生成獨立的 phf map 函式"#####,
    b"output_phf_all_in_one" => r#####"將所有語言的 phf map 輸出到同一個函式"#####,
    b"output_phf_without_map_name" => {
      r#####"類似於 output_phf, 但生成的函式的key 為普通字串，而不是 TupleKey"#####
    }
    b"output_ron" => r#####"輸出為 ron 格式的字串"#####,
    b"suffix" => r#####"新生成的"高亮Map"的字尾"#####,
    b"syntax_name" => r#####"語法名稱"#####,
    b"theme_name" => r#####"主題名稱"#####,
    b"true_color" => r#####"24位真彩色"#####,
    b"visibility" => r#####"生成的程式碼的可見性"#####,
    _ => "",
  }
}
