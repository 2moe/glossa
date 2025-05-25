pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kunna Bango"#####,
    b"base_name" => r#####"Sunan Asali na "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Ƙarshen Fayil na Bincode"#####,
    b"custom_syntax_set" => r#####"Fayil na Saitin Syntax na Musamman"#####,
    b"custom_theme_set" => r#####"Fayil na Saitin Jigo na Musamman"#####,
    b"display_config_dir" => r#####"Nuna Kundin Saitin Glossa"#####,
    b"dsl_suffix" => r#####"Ƙarshen Fayil na DSL (Tsohuwar ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Yanayin Jerin Baki: IDs na Harsunan da aka hana"#####
    }
    b"exclude_map_names" => r#####"Sunayen Map da aka hana"#####,
    b"include_languages" => {
      r#####"Yanayin Jerin Fari: IDs na Harsunan da aka jera kawai"#####
    }
    b"include_map_names" => r#####"Sunayen Map da aka jera kawai"#####,
    b"input" => r#####"Kundin Tushen Albarkatun Ƙabila"#####,
    b"list_all_syntaxes" => r#####"Nuna Dukkan Sunayen Syntax da Ƙararrakinsu"#####,
    b"list_all_themes" => r#####"Nuna Dukkan Sunayen Jigogi"#####,
    b"mod_prefix" => r#####"Gaban Fayil na Mod (Tsohuwar "l10n_")"#####,
    b"outdir" => r#####"Kundin Fitarwa"#####,
    b"output_bincode" => {
      r#####"Ƙirƙiri Fayilolin Bincode Daban don Harsuna Daban-daban"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Fitarda Dukkan Bincode na Harsuna cikin Fayil ɗaya"#####
    }
    b"output_locales_fn" => r#####"Fitarda Aikin all_locales"#####,
    b"output_match_fn" => {
      r#####"Ƙirƙiri Fayilolin Rust masu ayyukan Match don Harsuna"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Haɗa Dukkan Bayanai cikin Aikin Match ɗaya (String)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Aikin Match mai Sunan Harshe a matsayin Makulli"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Aikin Match tare da Makullin (Harshe + Map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Kamar output_match_fn amma Map_key kawai ake amfani"#####
    }
    b"output_phf" => r#####"Ƙirƙiri Ayyukan PHF Map na Harsuna"#####,
    b"output_phf_all_in_one" => r#####"Haɗa Dukkan PHF Map cikin Aiki ɗaya"#####,
    b"output_phf_without_map_name" => {
      r#####"PHF Map tare da Makullin String na yau da kullun"#####
    }
    b"output_ron" => r#####"Fitarda Bayanai cikin Tsarin RON"#####,
    b"suffix" => r#####"Ƙarshen Sabuwar "Highlight-Map""#####,
    b"syntax_name" => r#####"Sunan Syntax"#####,
    b"theme_name" => r#####"Sunan Jigo"#####,
    b"true_color" => r#####"Launi na Gaskiya 24-Bit"#####,
    b"visibility" => r#####"Ganewar Code da aka Ƙirƙira"#####,
    _ => "",
  }
}
