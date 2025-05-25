pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"是否启用背景"#####,
    b"base_name" => r#####"基础"高亮Map"的名称"#####,
    b"bincode_suffix" => r#####"bincode文件后缀"#####,
    b"custom_syntax_set" => r#####"自定义语法集文件"#####,
    b"custom_theme_set" => r#####"自定义主题集文件"#####,
    b"display_config_dir" => r#####"显示glossa的配置目录"#####,
    b"dsl_suffix" => r#####"DSL 文件后缀(默认 ".dsl")"#####,
    b"exclude_languages" => {
      r#####"黑名单模式。位于黑名单中的语言 id 不会被初始化"#####
    }
    b"exclude_map_names" => r#####"位于列表中的 map_names 不会被初始化"#####,
    b"include_languages" => {
      r#####"白名单模式。当其不为空时，只有位于列表中的语言 id 才会被初始化"#####
    }
    b"include_map_names" => {
      r#####"当其不为空时，只有位于列表中的 map_names 才会被初始化"#####
    }
    b"input" => r#####"本地化资源的源目录"#####,
    b"list_all_syntaxes" => r#####"显示所有语法名称及其扩展名"#####,
    b"list_all_themes" => r#####"显示所有主题名称"#####,
    b"mod_prefix" => r#####"mod 文件前缀 (默认 "l10n_")"#####,
    b"outdir" => r#####"输出的目录"#####,
    b"output_bincode" => r#####"为不同的语言生成独立的 bincode 文件"#####,
    b"output_bincode_all_in_one" => {
      r#####"将所有语言的 bincode 输出到同一个文件"#####
    }
    b"output_locales_fn" => r#####"输出 all_locales 函数"#####,
    b"output_match_fn" => {
      r#####"为不同的语言生成独立的 rust 代码的文件 (包含match表达式的函数)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"将所有语言的数据都输出为一个match函数（字符串）"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"将所有语言的数据输出为同一个match 函数（字符串），key 为语言名"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"将所有语言的数据输出为同一个 match 函数（字符串），key 为语言名和 map_key"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"类似于 output_match_fn，但生成的函数只以 map_key 作为 key, 不包含 map_name"#####
    }
    b"output_phf" => r#####"为不同语言生成独立的 phf map 函数"#####,
    b"output_phf_all_in_one" => r#####"将所有语言的 phf map 输出到同一个函数"#####,
    b"output_phf_without_map_name" => {
      r#####"类似于 output_phf, 但生成的函数的key 为普通字符串，而不是 TupleKey"#####
    }
    b"output_ron" => r#####"输出为 ron 格式的字符串"#####,
    b"suffix" => r#####"新生成的"高亮Map"的后缀"#####,
    b"syntax_name" => r#####"语法名称"#####,
    b"theme_name" => r#####"主题名称"#####,
    b"true_color" => r#####"24位真彩色"#####,
    b"visibility" => r#####"生成的代码的可见性"#####,
    _ => "",
  }
}
