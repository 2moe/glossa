pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Amkenye background"#####,
    b"base_name" => r#####"Ishina lyakuti "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Ifile ya bincode ya kumapila"#####,
    b"custom_syntax_set" => r#####"Ifile ya syntax set ya kwikala"#####,
    b"custom_theme_set" => r#####"Ifile ya theme set ya kwikala"#####,
    b"display_config_dir" => r#####"Langa indekelo ya config ya glossa"#####,
    b"dsl_suffix" => r#####"Ifile ya DSL ya kumapila (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Black list: Uwaya initializa ID ya ndimi sha list"#####
    }
    b"exclude_map_names" => r#####"Uwaya initializa map names sha list"#####,
    b"include_languages" => {
      r#####"White list: Initializa ID ya ndimi sha list fye"#####
    }
    b"include_map_names" => r#####"Initializa map names sha list fye"#####,
    b"input" => r#####"Indekelo ya mapango ya localization"#####,
    b"list_all_syntaxes" => {
      r#####"Langa ishina lyonse lya syntax ne extensions"#####
    }
    b"list_all_themes" => r#####"Langa ishina lyonse lya theme"#####,
    b"mod_prefix" => r#####"Ifile ya mod ya kumpanda (default "l10n_")"#####,
    b"outdir" => r#####"Indekelo yokupuma"#####,
    b"output_bincode" => {
      r#####"Amapanga ifile ya bincode yekha kuli ndimi elyo"#####
    }
    b"output_bincode_all_in_one" => r#####"Fumya bincode yonse mu ifile imo"#####,
    b"output_locales_fn" => r#####"Fumya all_locales function"#####,
    b"output_match_fn" => {
      r#####"Amapanga ifile ya Rust ine match functions kuli ndimi"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Fumya data yonse mu function imo ya match (ulubalushi)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match function nshi lyakuti ndimi nkana key"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match function ine key yasanganwa (ndimi + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Shatilako map_key fye nkana key (tafye map_name)"#####
    }
    b"output_phf" => r#####"Amapanga phf map functions yekha kuli ndimi"#####,
    b"output_phf_all_in_one" => {
      r#####"Sanganisha phf map yonse mu function imo"#####
    }
    b"output_phf_by_key" => r#####"Phf map ine key shalapo (shili TupleKey)"#####,
    b"output_ron" => r#####"Fumya ulubalushi mu format ya RON"#####,
    b"suffix" => r#####"Ishina lishya lya "Highlight-Map" ya kumapila"#####,
    b"syntax_name" => r#####"Ishina lya syntax"#####,
    b"theme_name" => r#####"Ishina lya theme"#####,
    b"true_color" => r#####"Ubwanga bwa 24-bit"#####,
    b"visibility" => r#####"Ukumoneka kwa code eyasanduka"#####,
    _ => "",
  }
}
