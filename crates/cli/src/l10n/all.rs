pub(crate) const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match (language, key) {
    (b"zh", b"background") => r#####"是否启用背景"#####,
    (b"zh", b"base_name") => r#####"基础"高亮Map"的名称"#####,
    (b"zh", b"bincode_suffix") => r#####"bincode文件后缀"#####,
    (b"zh", b"custom_syntax_set") => r#####"自定义语法集文件"#####,
    (b"zh", b"custom_theme_set") => r#####"自定义主题集文件"#####,
    (b"zh", b"dsl_suffix") => r#####"DSL 文件后缀(默认 ".dsl")"#####,
    (b"zh", b"exclude_languages") => {
      r#####"黑名单模式。位于黑名单中的语言 id 不会被初始化"#####
    }
    (b"zh", b"exclude_map_names") => {
      r#####"位于列表中的 map_names 不会被初始化"#####
    }
    (b"zh", b"include_languages") => {
      r#####"白名单模式，当其不为空时，只有位于列表中的语言 id 才会被初始化"#####
    }
    (b"zh", b"include_map_names") => {
      r#####"当其不为空时，只有位于列表中的 map_names 才会被初始化。"#####
    }
    (b"zh", b"input") => r#####"本地化资源的源目录"#####,
    (b"zh", b"mod_prefix") => r#####"mod 文件前缀 (默认 "l10n_")"#####,
    (b"zh", b"outdir") => r#####"输出的目录"#####,
    (b"zh", b"output_bincode") => r#####"输出为不同语言的 bincode 文件"#####,
    (b"zh", b"output_bincode_all_in_one") => {
      r#####"将所有语言的 bincode 输出到同一个文件"#####
    }
    (b"zh", b"output_locales_fn") => r#####"输出 all_locales 函数"#####,
    (b"zh", b"output_match_fn") => {
      r#####"输出为不同语言的 match表达式的函数的文件"#####
    }
    (b"zh", b"output_match_fn_all_in_one") => {
      r#####"将所有语言的数据输出为同一个match函数（字符串）"#####
    }
    (b"zh", b"output_match_fn_all_in_one_by_language") => {
      r#####"将所有语言的数据输出为同一个match 函数（字符串），key 为语言名"#####
    }
    (b"zh", b"output_match_fn_all_in_one_by_language_and_key") => {
      r#####"将所有语言的数据输出为同一个 match 函数（字符串），key 为语言名和 map_key"#####
    }
    (b"zh", b"output_phf") => r#####"输出为不同语言的 phf map 函数"#####,
    (b"zh", b"output_phf_all_in_one") => {
      r#####"将所有语言的 phf map 输出到同一个函数"#####
    }
    (b"zh", b"output_ron") => r#####"输出为 ron 格式的字符串"#####,
    (b"zh", b"show_all_syntaxes") => r#####"显示所有语法名称及其扩展名"#####,
    (b"zh", b"show_all_themes") => r#####"显示所有主题名称"#####,
    (b"zh", b"suffix") => r#####"新生成的"高亮Map"的后缀"#####,
    (b"zh", b"syntax_name") => r#####"语法名称"#####,
    (b"zh", b"theme_name") => r#####"主题名称"#####,
    (b"zh", b"true_color") => r#####"24位真彩色"#####,
    (b"zh", b"visibility") => r#####"生成的代码的可见性"#####,
    _ => "",
  }
}
