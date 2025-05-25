pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Qalisa ingemva"#####,
    b"base_name" => r#####"Igama lesiseko se-"Highlight-Map""#####,
    b"bincode_suffix" => r#####"Isihloko sefayile ye-bincode"#####,
    b"custom_syntax_set" => r#####"Ifayile yokucwangcisa isintaksi"#####,
    b"custom_theme_set" => r#####"Ifayile yokucwangcisa imixholo"#####,
    b"display_config_dir" => r#####"Bonisa indlela yokucwangcisa ye-glossa"#####,
    b"dsl_suffix" => r#####"Isihloko sefayile ye-DSL (okwendalo ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Imo yeluhlu olumnyama: Musa ukuqalisa ii-ID zolwimi ezikuluhlu"#####
    }
    b"exclude_map_names" => r#####"Musa ukuqalisa amagama e-map akuluhlu"#####,
    b"include_languages" => {
      r#####"Imo yeluhlu olumhlophe: Qalisa kuphela ii-ID zolwimi ezikuluhlu"#####
    }
    b"include_map_names" => r#####"Qalisa kuphela amagama e-map akuluhlu"#####,
    b"input" => r#####"Indlela yemithombo yokufaka ulwimi"#####,
    b"list_all_syntaxes" => {
      r#####"Bonisa zonke iigama zesintaksi kunye nokunwetshwa"#####
    }
    b"list_all_themes" => r#####"Bonisa zonke iigama zemixholo"#####,
    b"mod_prefix" => r#####"Isiqalo sefayile ye-mod (okwendalo "l10n_")"#####,
    b"outdir" => r#####"Indlela yokuphuma"#####,
    b"output_bincode" => {
      r#####"Yenza iifayile ze-bincode ezizimeleyo ngeelwimi ezahlukeneyo"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Khupha zonke ii-bincode zolwimi kufayile enye"#####
    }
    b"output_locales_fn" => r#####"Khupha umsebenzi we-all_locales"#####,
    b"output_match_fn" => {
      r#####"Yenza iifayile ze-Rust ezinemisebenzi ye-match ngeelwimi ezahlukeneyo"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Khupha yonke idatha kumsebenzi omnye we-match (umgca)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Umsebenzi we-match onegama lolwimi njengekhi"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Umsebenzi we-match onekhi edityanisiweyo (igama lolwimi + i-map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Sebenzisa kuphela i-map_key njengekhi (akubandakanyi i-map_name)"#####
    }
    b"output_phf" => r#####"Yenza imisebenzi ye-phf map ngeelwimi ezahlukeneyo"#####,
    b"output_phf_all_in_one" => {
      r#####"Khupha zonke ii-phf map kumsebenzi omnye"#####
    }
    b"output_phf_without_map_name" => {
      r#####"I-phf map enekhi yomgca oqhelekileyo (hayi i-TupleKey)"#####
    }
    b"output_ron" => r#####"Khupha umgca ngefomethi ye-RON"#####,
    b"suffix" => r#####"Isihloko esitsha se-"Highlight-Map""#####,
    b"syntax_name" => r#####"Igama lesintaksi"#####,
    b"theme_name" => r#####"Igama lomxholo"#####,
    b"true_color" => r#####"Umbala wenyani we-24-bit"#####,
    b"visibility" => r#####"Ukubonakala kwekhowudi eyenziwe"#####,
    _ => "",
  }
}
