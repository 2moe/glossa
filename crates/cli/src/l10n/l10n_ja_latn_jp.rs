pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"haikei yūkōka settei"#####,
    b"base_name" => r#####"kihon 「Highlight-Map」 meishō"#####,
    b"bincode_suffix" => r#####"bincode fairu kakuchōshi"#####,
    b"custom_syntax_set" => r#####"kasutamu kōbun setto fairu"#####,
    b"custom_theme_set" => r#####"kasutamu tēma setto fairu"#####,
    b"display_config_dir" => r#####"glossa settei direkutori o hyōji"#####,
    b"dsl_suffix" => r#####"DSL fairu kakuchōshi (defaruto ".dsl")"#####,
    b"exclude_languages" => {
      r#####"burakku risuto mōdo: risuto-nai gengo ID o jogai"#####
    }
    b"exclude_map_names" => r#####"risuto-nai mappu-mei o jogai"#####,
    b"include_languages" => {
      r#####"howaito risuto mōdo: risuto-nai gengo ID nomi shokika"#####
    }
    b"include_map_names" => r#####"risuto-nai mappu-mei nomi shokika"#####,
    b"input" => r#####"rōkaraizeu shisōsu no sōsu direkutori"#####,
    b"list_all_syntaxes" => r#####"zen-kōbun meishō to kakuchōshi o hyōji"#####,
    b"list_all_themes" => r#####"zen-tēma meishō o hyōji"#####,
    b"mod_prefix" => r#####"mod fairu settōji (defaruto "l10n_")"#####,
    b"outdir" => r#####"shutsuryoku direkutori"#####,
    b"output_bincode" => {
      r#####"gengo-goto ni dokuritsu shita bincode fairu o seisei"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"zen-gengo no bincode o tan'itsu fairu ni shutsuryoku"#####
    }
    b"output_locales_fn" => r#####"all_locales kansū o shutsuryoku"#####,
    b"output_match_fn" => {
      r#####"match-shiki o fukumu Rust kōdo fairu o gengo-betsu ni seisei"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"zen-dēta o tan'itsu match kansū de shutsuryoku (mojiretsu)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"gengo-mei o kī to shita tōgō match kansū"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"gengo-mei + map_key fukugō kī no match kansū"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key nomi o kī to shite shiyō (map_name jogai)"#####
    }
    b"output_phf" => r#####"gengo-betsu phf mappu kansū o seisei"#####,
    b"output_phf_all_in_one" => r#####"zen-phf mappu o tan'itsu kansū ni tōgō"#####,
    b"output_phf_by_key" => {
      r#####"tsūjō mojiretsu kī o shiyō shita phf mappu (TupleKey hi-shiyō)"#####
    }
    b"output_ron" => r#####"RON keishiki mojiretsu to shite shutsuryoku"#####,
    b"suffix" => r#####"shinki 「Highlight-Map」 setsūji"#####,
    b"syntax_name" => r#####"kōbun meishō"#####,
    b"theme_name" => r#####"tēma meishō"#####,
    b"true_color" => r#####"24-bitto True Color"#####,
    b"visibility" => r#####"seisei kōdo no kashisei"#####,
    _ => "",
  }
}
