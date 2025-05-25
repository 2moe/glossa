pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"enable background support"#####,
    b"base_name" => r#####"base "Highlight-Map" name"#####,
    b"bincode_suffix" => r#####"bincode file suffix"#####,
    b"custom_syntax_set" => r#####"custom syntax set file"#####,
    b"custom_theme_set" => r#####"custom theme set file"#####,
    b"dsl_suffix" => r#####"DSL file suffix (default: ".dsl")"#####,
    b"exclude_languages" => {
      r#####"block list mode: exclude listed language IDs"#####
    }
    b"exclude_map_names" => r#####"block list mode: exclude listed map names"#####,
    b"include_languages" => {
      r#####"allow list mode: only initialize listed language IDs"#####
    }
    b"include_map_names" => {
      r#####"allow list mode: only initialize listed map names"#####
    }
    b"input" => r#####"source directory of localization resources"#####,
    b"list_all_syntaxes" => r#####"display all syntax names with extensions"#####,
    b"list_all_themes" => r#####"display all theme names"#####,
    b"mod_prefix" => r#####"mod file prefix (default: "l10n_")"#####,
    b"outdir" => r#####"output directory"#####,
    b"output_bincode" => r#####"generate separate bincode files per language"#####,
    b"output_bincode_all_in_one" => {
      r#####"output all languages' bincode into a single file"#####
    }
    b"output_locales_fn" => r#####"generate all_locales function"#####,
    b"output_match_fn" => {
      r#####"generate Rust code files with match expressions per language"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"consolidate all language data into a single match function (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"single match function (string) with language name as key"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"single match function (string) with composite key (language + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"similar to output_match_fn, but generates functions using map_key only (without map_name)"#####
    }
    b"output_phf" => r#####"generate PHF map functions per language"#####,
    b"output_phf_all_in_one" => {
      r#####"output all PHF maps into a unified function"#####
    }
    b"output_phf_without_map_name" => {
      r#####"PHF maps with simple string keys (instead of TupleKey)"#####
    }
    b"output_ron" => r#####"output RON-formatted strings"#####,
    b"suffix" => r#####"suffix for new Highlight-Maps"#####,
    b"syntax_name" => r#####"syntax name"#####,
    b"theme_name" => r#####"theme name"#####,
    b"true_color" => r#####"24-bit true color"#####,
    b"visibility" => r#####"visibility of generated code"#####,
    _ => "",
  }
}
