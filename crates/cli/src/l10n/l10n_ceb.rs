pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"I-enable ang background"#####,
    b"base_name" => r#####"Base ngalan sa "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufiks sa bincode nga file"#####,
    b"custom_syntax_set" => r#####"Custom syntax set file"#####,
    b"custom_theme_set" => r#####"Custom theme set file"#####,
    b"display_config_dir" => r#####"Ipakita config directory sa glossa"#####,
    b"dsl_suffix" => r#####"DSL file suffix (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Black list mode: Ayaw i-initialize ang ID sa lista"#####
    }
    b"exclude_map_names" => r#####"Ayaw i-initialize ang map names sa lista"#####,
    b"include_languages" => {
      r#####"White list mode: I-initialize lamang ang ID sa lista"#####
    }
    b"include_map_names" => r#####"I-initialize lamang ang map names sa lista"#####,
    b"input" => r#####"Source directory sa lokal nga mga kapanguhaan"#####,
    b"list_all_syntaxes" => r#####"Ipakita tanang syntax names ug extensions"#####,
    b"list_all_themes" => r#####"Ipakita tanang theme names"#####,
    b"mod_prefix" => r#####"Mod file unahan (default "l10n_")"#####,
    b"outdir" => r#####"Direktoryo sa Gipagawas"#####,
    b"output_bincode" => {
      r#####"Mugna tagsa-tagsa nga bincode files para sa lain-laing pinulongan"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Ipagawas tanang bincode ngadto sa usa ka file"#####
    }
    b"output_locales_fn" => r#####"Ipagawas ang all_locales function"#####,
    b"output_match_fn" => {
      r#####"Mugna Rust files nga adunay match function matag pinulongan"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Ipagawas tanang datos ngadto sa usa ka match function (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match function nga gamiton ang pinulongan nga ngalan ingon key"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match function nga kombinasyon sa pinulongan + map_key ingon key"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Gamita lamang ang map_key ingon key (walay map_name)"#####
    }
    b"output_phf" => r#####"Mugna phf map functions matag pinulongan"#####,
    b"output_phf_all_in_one" => {
      r#####"Ipagawas tanang phf maps ngadto sa usa ka function"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map nga ordinaryo nga string keys (dili TupleKey)"#####
    }
    b"output_ron" => r#####"Ipagawas string sa RON format"#####,
    b"suffix" => r#####"Bag-ong himo nga suffix sa "Highlight-Map""#####,
    b"syntax_name" => r#####"Ngalan sa syntax"#####,
    b"theme_name" => r#####"Ngalan sa tema"#####,
    b"true_color" => r#####"24-bit tinuod nga kolor"#####,
    b"visibility" => r#####"Makita ang gihimong kodigo"#####,
    _ => "",
  }
}
