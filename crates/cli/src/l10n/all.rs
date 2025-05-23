pub(crate) const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match (language, key) {
    (b"en", b"background") => r#####"enable background support"#####,
    (b"en", b"base_name") => r#####"base "Highlight-Map" name"#####,
    (b"en", b"bincode_suffix") => r#####"bincode file suffix"#####,
    (b"en", b"custom_syntax_set") => r#####"custom syntax set file"#####,
    (b"en", b"custom_theme_set") => r#####"custom theme set file"#####,
    (b"en", b"dsl_suffix") => r#####"DSL file suffix (default: ".dsl")"#####,
    (b"en", b"exclude_languages") => {
      r#####"block list mode: exclude listed language IDs"#####
    }
    (b"en", b"exclude_map_names") => {
      r#####"block list mode: exclude listed map names"#####
    }
    (b"en", b"include_languages") => {
      r#####"allow list mode: only initialize listed language IDs"#####
    }
    (b"en", b"include_map_names") => {
      r#####"allow list mode: only initialize listed map names"#####
    }
    (b"en", b"input") => r#####"source directory of localization resources"#####,
    (b"en", b"mod_prefix") => r#####"mod file prefix (default: "l10n_")"#####,
    (b"en", b"outdir") => r#####"output directory"#####,
    (b"en", b"output_bincode") => {
      r#####"generate separate bincode files per language"#####
    }
    (b"en", b"output_bincode_all_in_one") => {
      r#####"output all languages' bincode into a single file"#####
    }
    (b"en", b"output_locales_fn") => r#####"generate all_locales function"#####,
    (b"en", b"output_match_fn") => {
      r#####"generate Rust code files with match expressions per language"#####
    }
    (b"en", b"output_match_fn_all_in_one") => {
      r#####"consolidate all language data into a single match function (string)"#####
    }
    (b"en", b"output_match_fn_all_in_one_by_language") => {
      r#####"single match function (string) with language name as key"#####
    }
    (b"en", b"output_match_fn_all_in_one_by_language_and_key") => {
      r#####"single match function (string) with composite key (language + map_key)"#####
    }
    (b"en", b"output_match_fn_by_key") => {
      r#####"similar to output_match_fn, but generates functions using map_key only (without map_name)"#####
    }
    (b"en", b"output_phf") => r#####"generate PHF map functions per language"#####,
    (b"en", b"output_phf_all_in_one") => {
      r#####"output all PHF maps into a unified function"#####
    }
    (b"en", b"output_phf_by_key") => {
      r#####"PHF maps with simple string keys (instead of TupleKey)"#####
    }
    (b"en", b"output_ron") => r#####"output RON-formatted strings"#####,
    (b"en", b"list_all_syntaxes") => {
      r#####"display all syntax names with extensions"#####
    }
    (b"en", b"list_all_themes") => r#####"display all theme names"#####,
    (b"en", b"suffix") => r#####"suffix for new Highlight-Maps"#####,
    (b"en", b"syntax_name") => r#####"syntax name"#####,
    (b"en", b"theme_name") => r#####"theme name"#####,
    (b"en", b"true_color") => r#####"24-bit true color"#####,
    (b"en", b"visibility") => r#####"visibility of generated code"#####,
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
      r#####"当其不为空时，只有位于列表中的 map_names 才会被初始化"#####
    }
    (b"zh", b"input") => r#####"本地化资源的源目录"#####,
    (b"zh", b"mod_prefix") => r#####"mod 文件前缀 (默认 "l10n_")"#####,
    (b"zh", b"outdir") => r#####"输出的目录"#####,
    (b"zh", b"output_bincode") => r#####"为不同的语言生成独立的 bincode 文件"#####,
    (b"zh", b"output_bincode_all_in_one") => {
      r#####"将所有语言的 bincode 输出到同一个文件"#####
    }
    (b"zh", b"output_locales_fn") => r#####"输出 all_locales 函数"#####,
    (b"zh", b"output_match_fn") => {
      r#####"为不同的语言生成独立的 rust 代码的文件 (包含match表达式的函数)"#####
    }
    (b"zh", b"output_match_fn_all_in_one") => {
      r#####"将所有语言的数据都输出为一个match函数（字符串）"#####
    }
    (b"zh", b"output_match_fn_all_in_one_by_language") => {
      r#####"将所有语言的数据输出为同一个match 函数（字符串），key 为语言名"#####
    }
    (b"zh", b"output_match_fn_all_in_one_by_language_and_key") => {
      r#####"将所有语言的数据输出为同一个 match 函数（字符串），key 为语言名和 map_key"#####
    }
    (b"zh", b"output_match_fn_by_key") => {
      r#####"类似于 output_match_fn，但生成的函数只以 map_key 作为 key, 不包含 map_name"#####
    }
    (b"zh", b"output_phf") => r#####"为不同语言生成独立的 phf map 函数"#####,
    (b"zh", b"output_phf_all_in_one") => {
      r#####"将所有语言的 phf map 输出到同一个函数"#####
    }
    (b"zh", b"output_phf_by_key") => {
      r#####"类似于 output_phf, 但生成的函数的key 为普通字符串，而不是 TupleKey"#####
    }
    (b"zh", b"output_ron") => r#####"输出为 ron 格式的字符串"#####,
    (b"zh", b"list_all_syntaxes") => r#####"显示所有语法名称及其扩展名"#####,
    (b"zh", b"list_all_themes") => r#####"显示所有主题名称"#####,
    (b"zh", b"suffix") => r#####"新生成的"高亮Map"的后缀"#####,
    (b"zh", b"syntax_name") => r#####"语法名称"#####,
    (b"zh", b"theme_name") => r#####"主题名称"#####,
    (b"zh", b"true_color") => r#####"24位真彩色"#####,
    (b"zh", b"visibility") => r#####"生成的代码的可见性"#####,
    _ => "",
  }
}
