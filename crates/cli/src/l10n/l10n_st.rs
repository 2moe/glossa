pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Enable background"#####,
    b"base_name" => r#####"Lebitso la motheo oa "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sefate sa bincode"#####,
    b"custom_syntax_set" => r#####"Faele ea setaepte ea syntax e ikhethileng"#####,
    b"custom_theme_set" => r#####"Faele ea setaepte ea ditheto e ikhethileng"#####,
    b"display_config_dir" => r#####"Hlahisa directory ea config ea glossa"#####,
    b"dsl_suffix" => r#####"Sefate sa DSL (ka khetho ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mokhoa oa lenane le lesoeu: Se ke oa hlophisa ID ea puo e le lenaneng"#####
    }
    b"exclude_map_names" => r#####"Se ke oa hlophisa maina a map e le lenaneng"#####,
    b"include_languages" => {
      r#####"Mokhoa oa lenane le lešoeu: Hlophisa ID ea puo feela haeba lenane le se le sieo"#####
    }
    b"include_map_names" => {
      r#####"Hlophisa maina a map feela haeba lenane le se le sieo"#####
    }
    b"input" => r#####"Directory ea mohloli oa lipatlisiso tsa lehae"#####,
    b"list_all_syntaxes" => r#####"Hlahisa maina ohle a syntax le li-extension"#####,
    b"list_all_themes" => r#####"Hlahisa maina ohle a ditheto"#####,
    b"mod_prefix" => r#####"Peo ea mod (ka khetho "l10n_")"#####,
    b"outdir" => r#####"Directory ea ho hlahisa"#####,
    b"output_bincode" => {
      r#####"Hlahisa lifaele tsa bincode tse ikemetseng ka puo e fapaneng"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Hlahisa bincode ea dipuo tsohle ka faele e le 'ngoe"#####
    }
    b"output_locales_fn" => r#####"Hlahisa mesebetsi ea all_locales"#####,
    b"output_match_fn" => {
      r#####"Hlahisa lifaele tsa Rust le mesebetsi ea match ka puo e fapaneng"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Hlahisa data kaofela ka mesebetsi e le 'ngoe ea match (lentsoe)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Mesebetsi ea match e nang le lebitso la puo e le senotlolo"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Mesebetsi ea match e nang le senotlolo sa lebitso + map_key"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Sebelisa map_key feela e le senotlolo (ha se map_name)"#####
    }
    b"output_phf" => r#####"Hlahisa mesebetsi ea phf map ka puo e fapaneng"#####,
    b"output_phf_all_in_one" => {
      r#####"Hlahisa phf map tsohle ka mesebetsi e le 'ngoe"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map e nang le lentsoe le tloaelehileng (eseng TupleKey)"#####
    }
    b"output_ron" => r#####"Hlahisa lentsoe ka foromo ea RON"#####,
    b"suffix" => r#####"Sefate se setjha sa "Highlight-Map""#####,
    b"syntax_name" => r#####"Lebitso la syntax"#####,
    b"theme_name" => r#####"Lebitso la thero"#####,
    b"true_color" => r#####"Mmala oa 'nete oa 24-bit"#####,
    b"visibility" => r#####"Ponahalo ea khoutu e hlahisitsoeng"#####,
    _ => "",
  }
}
