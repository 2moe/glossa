pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Fon işletmek"#####,
    b"base_name" => r#####""Highlight-Map" esasy ady"#####,
    b"bincode_suffix" => r#####"Bincode faýl suffiksi"#####,
    b"custom_syntax_set" => r#####"Özleşdirilen sintaksis toplumy faýly"#####,
    b"custom_theme_set" => r#####"Özleşdirilen tema toplumy faýly"#####,
    b"display_config_dir" => r#####"Glossa konfigurasiýa katalogyny görkez"#####,
    b"dsl_suffix" => r#####"DSL faýl suffiksi (deslapky ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Gara sanaw tertibi: Sanawdaky dil ID-laryny başlatma"#####
    }
    b"exclude_map_names" => r#####"Sanawdaky kart adlaryny başlatma"#####,
    b"include_languages" => {
      r#####"Ak sanaw tertibi: Sanawdaky dil ID-laryny diňe başlat"#####
    }
    b"include_map_names" => r#####"Sanawdaky kart adlaryny diňe başlat"#####,
    b"input" => r#####"Lokalizasiýa resurslarynyň çeşmesi"#####,
    b"list_all_syntaxes" => {
      r#####"Ähli sintaksis adlaryny we giňeltmeleri görkez"#####
    }
    b"list_all_themes" => r#####"Ähli tema adlaryny görkez"#####,
    b"mod_prefix" => r#####"Mod faýl prefiksi (deslapky "l10n_")"#####,
    b"outdir" => r#####"Çykdylar katalogy"#####,
    b"output_bincode" => {
      r#####"Aýratyn diller üçin aýratyn bincode faýllary döretmek"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Ähli dilleriň bincode-laryny bir faýla çykarmak"#####
    }
    b"output_locales_fn" => r#####"all_locales funksiýasyny çykarmak"#####,
    b"output_match_fn" => {
      r#####"Rust kody faýllaryny döretmek (match ifadeli funksiýalar bilen)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Ähli maglumatlary bir match funksiýasynda çykarmak (setir)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Dil ady bilen açar ulanylýan match funksiýasy"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Birleşdirilen açar (dil+map_key) bilen match funksiýasy"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Diňe map_key açar ulanyň (map_name ýok)"#####
    }
    b"output_phf" => r#####"Aýratyn diller üçin phf kart funksiýalary döretmek"#####,
    b"output_phf_all_in_one" => {
      r#####"Ähli phf kartlary bir funksiýada birleşdirmek"#####
    }
    b"output_phf_without_map_name" => {
      r#####"Adaty setir açarlary bilen phf kart (TupleKey däl)"#####
    }
    b"output_ron" => r#####"RON formatynda setir çykarmak"#####,
    b"suffix" => r#####""Highlight-Map" täze suffiksi"#####,
    b"syntax_name" => r#####"Sintaksis ady"#####,
    b"theme_name" => r#####"Tema ady"#####,
    b"true_color" => r#####"24-bit hakyky reňk"#####,
    b"visibility" => r#####"Generirlen kodyň görnüşi"#####,
    _ => "",
  }
}
