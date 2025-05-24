pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Sɛ background be tumi ase"#####,
    b"base_name" => r#####"Mfitiaseɛ "Highlight-Map" din"#####,
    b"bincode_suffix" => r#####"Bincode file akyi"#####,
    b"custom_syntax_set" => r#####"Wɔn ano adwuma syntax set file"#####,
    b"custom_theme_set" => r#####"Wɔn ano adwuma theme set file"#####,
    b"display_config_dir" => r#####"Kyerɛ glossa configuration fie"#####,
    b"dsl_suffix" => r#####"DSL file akyi (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"BlackList ɔkwan: Language ID a ɛwɔ lista no mu nnhyɛ ase"#####
    }
    b"exclude_map_names" => r#####"Map_names a ɛwɔ lista no mu nnhyɛ ase"#####,
    b"include_languages" => {
      r#####"WhiteList ɔkwan: Sɛ lista no nyɛ empty a, language ID a ɛwɔ lista no mu na ɛbɛhyɛ ase"#####
    }
    b"include_map_names" => {
      r#####"Sɛ lista no nyɛ empty a, map_names a ɛwɔ lista no mu na ɛbɛhyɛ ase"#####
    }
    b"input" => r#####"Fie a localization nnwuma fi"#####,
    b"list_all_syntaxes" => r#####"Kyerɛ syntax din nyinaa ne extensions"#####,
    b"list_all_themes" => r#####"Kyerɛ theme din nyinaa"#####,
    b"mod_prefix" => r#####"Mod file din a wɔde hyɛ ase (default "l10n_")"#####,
    b"outdir" => r#####"Nnwoma a wɔato so"#####,
    b"output_bincode" => r#####"Yɛ bincode file ahorow ma nnkyekyɛmu kasa"#####,
    b"output_bincode_all_in_one" => {
      r#####"To bincode a ɛwɔ kasa nyinaa gu file baako mu"#####
    }
    b"output_locales_fn" => r#####"To all_locales function"#####,
    b"output_match_fn" => {
      r#####"Yɛ Rust kod file ahorow (ne match expression function) ma kasa nnkyekyɛmu"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"To kasa data nyinaa to match function baako mu (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"To kasa data nyinaa to match function baako mu (string) a ɔkyerɛw kasa din"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"To kasa data nyinaa to match function baako mu (string) a ɔkyerɛw kasa din ne map_key"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Sɛnea ɛte wɔ output_match_fn, nanso map_key nko na ɛyɛ key, map_name nni mu"#####
    }
    b"output_phf" => r#####"Yɛ phf map function ahorow ma nnkyekyɛmu kasa"#####,
    b"output_phf_all_in_one" => {
      r#####"To phf map a ɛwɔ kasa nyinaa to function baako mu"#####
    }
    b"output_phf_by_key" => {
      r#####"Sɛnea ɛte wɔ output_phf, nanso string key na ɛwɔ hɔ, nnyɛ TupleKey"#####
    }
    b"output_ron" => r#####"To RON format string"#####,
    b"suffix" => r#####"Akyi a ɛwɔ Highlight-Map foforo so"#####,
    b"syntax_name" => r#####"Syntax din"#####,
    b"theme_name" => r#####"Theme din"#####,
    b"true_color" => r#####"24-bit nokware kɔla"#####,
    b"visibility" => r#####"Biakoyɛ a kod a wɔayɛ no wɔ"#####,
    _ => "",
  }
}
