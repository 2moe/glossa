pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kukozesa ekirala wamu"#####,
    b"base_name" => r#####"Elinnya ly''ekisinga "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Endabika y'efayile ya bincode"#####,
    b"custom_syntax_set" => r#####"Fayile y'endabika ey'okwegomba"#####,
    b"custom_theme_set" => r#####"Fayile y'emitwe gy'okwegomba"#####,
    b"display_config_dir" => r#####"Okulaga ekika ky'okutegekera glossa"#####,
    b"dsl_suffix" => r#####"Endabika y'efayile ya DSL (Ebisangibwa ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Engeri y'okugoba: ID z'olulimi ezikwatiddwa tezikozesebwa"#####
    }
    b"exclude_map_names" => r#####"Amannya g'emapu agakwatiddwa tegaakozesebwa"#####,
    b"include_languages" => {
      r#####"Engeri y'olukalata: ID z'olulimi ezigendereza gye zikozesebwa"#####
    }
    b"include_map_names" => r#####"Amannya g'emapu agendereza gye gakozesebwa"#####,
    b"input" => r#####"Ekika ky'ebikozesebwa eby'olulimi"#####,
    b"list_all_syntaxes" => r#####"Okulaga amannya gendabika n'okwongerwako"#####,
    b"list_all_themes" => r#####"Okulaga amannya g'emitwe gyonna"#####,
    b"mod_prefix" => {
      r#####"Engeri yokutandika kwa fayile ya mod (Ebisangibwa "l10n_")"#####
    }
    b"outdir" => r#####"Ekika ky'okushushana"#####,
    b"output_bincode" => {
      r#####"Kukora fayile za bincode ezaawukana mu nnimi ezaawukana"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Okushushana ebintu byonna bya bincode mu fayile emu"#####
    }
    b"output_locales_fn" => r#####"Okushushana ekintu ky'all_locales"#####,
    b"output_match_fn" => {
      r#####"Kukora fayile za Rust ezikwatagana ne match mu nnimi"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kushushana byonna mu kintu kimu ekikwatana (olukwegu)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Ekintu ekikwatana kikozese erinnya ly'olulimi"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Ekintu ekikwatana kikozese erinnya + map_key"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Kukozesa map_key okukka ekintu ekikulu (tetwaliramu map_name)"#####
    }
    b"output_phf" => r#####"Kukora phf map ezaawukana mu nnimi"#####,
    b"output_phf_all_in_one" => r#####"Kushushana phf map zonna mu kintu kimu"#####,
    b"output_phf_by_key" => {
      r#####"phf map ezikozesa olukwegu olwa bulijjo (si TupleKey)"#####
    }
    b"output_ron" => r#####"Okushushana olukwegu lwa RON"#####,
    b"suffix" => r#####"Endabika y''ekisinga "Highlight-Map" empya"#####,
    b"syntax_name" => r#####"Elinnya ly'endabika"#####,
    b"theme_name" => r#####"Elinnya ly'omutwe"#####,
    b"true_color" => r#####"Amabara agenkufu 24-bit"#####,
    b"visibility" => r#####"Okurikwata kwa code eyaakozesebwa"#####,
    _ => "",
  }
}
