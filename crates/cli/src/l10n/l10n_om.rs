pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Duuba dhaabbachuu"#####,
    b"base_name" => r#####"Maqaa jalqabaa "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffix failii bincode"#####,
    b"custom_syntax_set" => r#####"Failii syntax sirreeffama"#####,
    b"custom_theme_set" => r#####"Failii theme sirreeffama"#####,
    b"display_config_dir" => r#####"Fooqonna saagantaa glossa agarsiisuu"#####,
    b"dsl_suffix" => r#####"Suffix failii DSL (haqa ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Fakkii Listaa Gurraacha: ID afaanota hin jalqabsiisin"#####
    }
    b"exclude_map_names" => r#####"Maqoota map hin jalqabsiisin"#####,
    b"include_languages" => {
      r#####"Fakkii Listaa: ID afaanota listaa qofa jalqabsiisuu"#####
    }
    b"include_map_names" => r#####"Maqoota map listaa qofa jalqabsiisuu"#####,
    b"input" => r#####"Fooqonna utubaa lokalizeessuu"#####,
    b"list_all_syntaxes" => r#####"Maqoota syntax hunda agarsiisuu"#####,
    b"list_all_themes" => r#####"Maqoota theme hunda agarsiisuu"#####,
    b"mod_prefix" => r#####"Prefiksi failii mod (haqa "l10n_")"#####,
    b"outdir" => r#####"Fooqonna baasii"#####,
    b"output_bincode" => r#####"Uumuu failota bincode addaa afaan adda addaaf"#####,
    b"output_bincode_all_in_one" => {
      r#####"Baasii bincode afaanota hunda failii tokkotti"#####
    }
    b"output_locales_fn" => r#####"Baasii function all_locales"#####,
    b"output_match_fn" => {
      r#####"Uumuu failota Rust waliin match function afaan adda addaaf"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Madda afaanota hunda match function tokko keessatti (jijjiirama)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match function maqaa afaanii fayyadamuun"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match function waliigala (maqaa afaanii + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Fayyadama map_key qofa (map_name hin qabu)"#####
    }
    b"output_phf" => r#####"Uumuu phf map function addaa afaan adda addaaf"#####,
    b"output_phf_all_in_one" => {
      r#####"Madda phf map hunda function tokko keessatti"#####
    }
    b"output_phf_by_key" => r#####"phf map jijjiirama qaamolee (hin TaplKey)"#####,
    b"output_ron" => r#####"Baasii jijjiirama fomataa RON"#####,
    b"suffix" => r#####"Suffix haaraa "Highlight-Map""#####,
    b"syntax_name" => r#####"Maqaa syntax"#####,
    b"theme_name" => r#####"Maqaa theme"#####,
    b"true_color" => r#####"Halluu dhugaa 24-bit"#####,
    b"visibility" => r#####"Mul'ata koodii kan uumame"#####,
    _ => "",
  }
}
