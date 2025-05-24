pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Cumasú cúlra"#####,
    b"base_name" => r#####"Bunainm "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Iarmhír comhaid bincode"#####,
    b"custom_syntax_set" => r#####"Comhad tacair chomhréir saincheaptha"#####,
    b"custom_theme_set" => r#####"Comhad tacair téama saincheaptha"#####,
    b"display_config_dir" => r#####"Taispeáin comhadlann cumraíochta glossa"#####,
    b"dsl_suffix" => r#####"Iarmhír comhaid DSL (réamhshocraithe ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mód liosta dubh: ID teanga liostaithe gan thúsú"#####
    }
    b"exclude_map_names" => r#####"Ainmneacha léarphort liostaithe gan thúsú"#####,
    b"include_languages" => {
      r#####"Mód liosta bán: ID teanga liostaithe amháin a thúsú"#####
    }
    b"include_map_names" => {
      r#####"Ainmneacha léarphort liostaithe amháin a thúsú"#####
    }
    b"input" => r#####"Comhadlann fhoinse acmhainní logánúcháin"#####,
    b"list_all_syntaxes" => r#####"Taispeáin gach ainm comhréire agus síneadh"#####,
    b"list_all_themes" => r#####"Taispeáin gach ainm téama"#####,
    b"mod_prefix" => r#####"Réimír comhaid mod (réamhshocraithe "l10n_")"#####,
    b"outdir" => r#####"Comhadlann Aschur"#####,
    b"output_bincode" => {
      r#####"Gin comhaid bincode neamhspleácha do theangacha éagsúla"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Aschur gach bincode teanga i gcomhad amháin"#####
    }
    b"output_locales_fn" => r#####"Aschur feidhm all_locales"#####,
    b"output_match_fn" => {
      r#####"Gin comhaid Rust le feidhmeanna match do theangacha"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Comhcheangail sonraí i bhfeidhm match amháin (teaghrán)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Feidhm match le hainm teanga mar eochair"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Feidhm match le heochair chomhshuite (ainm teanga + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Úsáid map_key mar eochair amháin (gan map_name)"#####
    }
    b"output_phf" => r#####"Gin feidhmeanna léarphort phf do theangacha"#####,
    b"output_phf_all_in_one" => {
      r#####"Comhcheangail gach léarphort phf i bhfeidhm amháin"#####
    }
    b"output_phf_by_key" => {
      r#####"Léarphort phf le heochracha teaghrán (ní TupleKey)"#####
    }
    b"output_ron" => r#####"Aschur i bhformáid RON mar theaghrán"#####,
    b"suffix" => r#####"Iarmhír "Highlight-Map" nua"#####,
    b"syntax_name" => r#####"Ainm chomhréir"#####,
    b"theme_name" => r#####"Ainm téama"#####,
    b"true_color" => r#####"Dath fíor 24-giotán"#####,
    b"visibility" => r#####"Infeictheacht an chóid ghin"#####,
    _ => "",
  }
}
