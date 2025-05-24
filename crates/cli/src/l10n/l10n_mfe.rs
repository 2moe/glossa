pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Active fond"#####,
    b"base_name" => r#####"Non baz "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufiks fichier bincode"#####,
    b"custom_syntax_set" => r#####"Fichier sentaks personnalisé"#####,
    b"custom_theme_set" => r#####"Fichier tèm personnalisé"#####,
    b"display_config_dir" => r#####"Afiche dosye konfigirasion Glossa"#####,
    b"dsl_suffix" => r#####"Sufiks fichier DSL (default ".dsl")"#####,
    b"exclude_languages" => r#####"Mòd lis nwar: Pa sir ID langaz dan lis"#####,
    b"exclude_map_names" => r#####"Pa sir non map dan lis"#####,
    b"include_languages" => r#####"Mòd lis blan: Sir ID langaz dan lis selman"#####,
    b"include_map_names" => r#####"Sir non map dan lis selman"#####,
    b"input" => r#####"Dosye resous lokalizasion"#####,
    b"list_all_syntaxes" => r#####"Afiche tou non sentaks ek ekstansyon"#####,
    b"list_all_themes" => r#####"Afiche tou non tèm"#####,
    b"mod_prefix" => r#####"Prefiks fichier mod (default "l10n_")"#####,
    b"outdir" => r#####"Dosye Sorti"#####,
    b"output_bincode" => {
      r#####"Gener fichier bincode separe pou bann langaz diferan"#####
    }
    b"output_bincode_all_in_one" => r#####"Met tou bincode dan enn sel fichier"#####,
    b"output_locales_fn" => r#####"Sorti fonksyon all_locales"#####,
    b"output_match_fn" => {
      r#####"Gener fichier Rust ek fonksyon match pou sak langaz"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Met tou done dan enn sel fonksyon match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonksyon match avek non langaz kouma kle"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Fonksyon match avek kle konbine (langaz + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Sir map_key kouma kle selman (pa ena map_name)"#####
    }
    b"output_phf" => r#####"Gener fonksyon phf map separe pou langaz"#####,
    b"output_phf_all_in_one" => r#####"Met tou phf map dan enn sel fonksyon"#####,
    b"output_phf_by_key" => {
      r#####"phf map avek kle string ordinær (pa TupleKey)"#####
    }
    b"output_ron" => r#####"Sorti string dan format RON"#####,
    b"suffix" => r#####"Nouvo sufix "Highlight-Map""#####,
    b"syntax_name" => r#####"Non sentaks"#####,
    b"theme_name" => r#####"Non tèm"#####,
    b"true_color" => r#####"Koulè reyèl 24-bit"#####,
    b"visibility" => r#####"Vizibilite kod ki finn generé"#####,
    _ => "",
  }
}
