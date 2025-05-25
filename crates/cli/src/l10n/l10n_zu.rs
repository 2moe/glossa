pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Vula isizinda"#####,
    b"base_name" => r#####"Igama eliyisisekelo le-"Highlight-Map""#####,
    b"bincode_suffix" => r#####"Isijobelelo sefayela ye-bincode"#####,
    b"custom_syntax_set" => r#####"Ifayela lesiTakhi elikhethekile"#####,
    b"custom_theme_set" => r#####"Ifayela lamathimu akhethekile"#####,
    b"display_config_dir" => r#####"Bonisa indawo yokucushwa kwe-Glossa"#####,
    b"dsl_suffix" => {
      r#####"Isijobelelo sefayela ye-DSL (okungu-".dsl" ngokumiselwe)"#####
    }
    b"exclude_languages" => {
      r#####"Imodi yohlu olumnyama: Ungaqalisi ama-ID ezilimi ahlwini"#####
    }
    b"exclude_map_names" => r#####"Ungaqalisi amagama e-map ahlwini"#####,
    b"include_languages" => {
      r#####"Imodi yohlu olumhlophe: Qalisa kuphela ama-ID ezilimi ohlwini"#####
    }
    b"include_map_names" => r#####"Qalisa kuphela amagama e-map ahlwini"#####,
    b"input" => r#####"Indawo yemithombo yesiZulu"#####,
    b"list_all_syntaxes" => {
      r#####"Bonisa wonke amagama esiTakhi kanye nezandiso"#####
    }
    b"list_all_themes" => r#####"Bonisa wonke amagama amathimu"#####,
    b"mod_prefix" => {
      r#####"Isiqalo sefayela ye-mod (okungu-"l10n_" ngokumiselwe)"#####
    }
    b"outdir" => r#####"Indawo yokuphuma"#####,
    b"output_bincode" => {
      r#####"Dala amafayela e-bincode ahlukene ngezilimi ezahlukene"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Khipha zonke izilimi ze-bincode kufayela elilodwa"#####
    }
    b"output_locales_fn" => r#####"Khipha i-all_locales function"#####,
    b"output_match_fn" => {
      r#####"Dala amafayela e-Rust ane-match functions ezilimini ezahlukene"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Khipha yonke idatha kufayela elilodwa le-match function (uchungechunge)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"I-match function enegama lesiLimi njengesihluthuli"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"I-match function enesihluthuli esihlanganisiwe (igama lesiLimi + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Sebenzisa i-map_key kuphela njengesihluthuli (akufaki i-map_name)"#####
    }
    b"output_phf" => r#####"Dala ama-phf map functions ahlukene ngezilimi"#####,
    b"output_phf_all_in_one" => {
      r#####"Hlanganisa zonke iziphakamiso ze-phf map kufayela elilodwa"#####
    }
    b"output_phf_without_map_name" => {
      r#####"I-phf map enezihluthuli eziyimvumelwano (hhayi i-TupleKey)"#####
    }
    b"output_ron" => r#####"Khipha uchungechunge ngefomethi ye-RON"#####,
    b"suffix" => r#####"Isijobelelo esisha se-"Highlight-Map""#####,
    b"syntax_name" => r#####"Igama lesiTakhi"#####,
    b"theme_name" => r#####"Igama lesihloko"#####,
    b"true_color" => r#####"Umbala weqiniso onamabhithi angu-24"#####,
    b"visibility" => r#####"Ukubonakala kwekhodi ekhiqiziwe"#####,
    _ => "",
  }
}
