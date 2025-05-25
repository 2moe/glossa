pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Fa'aaga le talaaga"#####,
    b"base_name" => r#####"Igoa autu o le "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Fa'ai'uga o le faila bincode"#####,
    b"custom_syntax_set" => r#####"Faila seti syntax fa'apitoa"#####,
    b"custom_theme_set" => r#####"Faila seti autu fa'apitoa"#####,
    b"display_config_dir" => r#####"Fa'aali le lisi fa'atulagaina o le glossa"#####,
    b"dsl_suffix" => r#####"Fa'ai'uga o le faila DSL (le masani ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Faiga Lisi Uliuli: Aua ne'i fa'ato'a ID gagana i le lisi"#####
    }
    b"exclude_map_names" => r#####"Aua ne'i fa'ato'a igoa map i le lisi"#####,
    b"include_languages" => {
      r#####"Faiga Lisi Pa'epa'e: Fa'ato'a ID gagana i le lisi pe a le gaogao"#####
    }
    b"include_map_names" => r#####"Fa'ato'a igoa map i le lisi pe a le gaogao"#####,
    b"input" => r#####"Lisi puna o punaoa fa'alotoifale"#####,
    b"list_all_syntaxes" => r#####"Fa'aali uma igoa syntax ma fa'aopoopoga"#####,
    b"list_all_themes" => r#####"Fa'aali uma igoa autu"#####,
    b"mod_prefix" => r#####"Fa'amuamua o le faila mod (le masani "l10n_")"#####,
    b"outdir" => r#####"Lisi o Galuega"#####,
    b"output_bincode" => r#####"Fausia faila bincode eseese mo gagana eseese"#####,
    b"output_bincode_all_in_one" => {
      r#####"Tu'u uma bincode gagana i se faila e tasi"#####
    }
    b"output_locales_fn" => r#####"Tu'u atu le galuega all_locales"#####,
    b"output_match_fn" => {
      r#####"Fausia faila Rust ma galuega match mo gagana ta'itasi"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Fa'atasi fa'amaumauga i se galuega match e tasi (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Galuega match ma igoa gagana e fai ma ki"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Galuega match ma ki tu'ufa'atasi (igoa gagana + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Fa'aaoga na'o le map_key e fai ma ki (leai se map_name)"#####
    }
    b"output_phf" => r#####"Fausia galuega phf map mo gagana eseese"#####,
    b"output_phf_all_in_one" => {
      r#####"Fa'atasi uma phf map i se galuega e tasi"#####
    }
    b"output_phf_without_map_name" => r#####"phf map ma ki masani (leai se TupleKey)"#####,
    b"output_ron" => r#####"Tu'u atu string i le faatulagaga RON"#####,
    b"suffix" => r#####"Fa'ai'uga fou o le "Highlight-Map""#####,
    b"syntax_name" => r#####"Igoa o le syntax"#####,
    b"theme_name" => r#####"Igoa autu"#####,
    b"true_color" => r#####"Lanu moni 24-bit"#####,
    b"visibility" => r#####"Malamalama o le code ua fausia"#####,
    _ => "",
  }
}
