pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"attiva appoġġ għall-isfond"#####,
    b"base_name" => r#####"isem bażi tal-"Highlight-Map""#####,
    b"bincode_suffix" => r#####"suffiss tal-fajl bincode"#####,
    b"custom_syntax_set" => r#####"fajl sett sintassi personalizzat"#####,
    b"custom_theme_set" => r#####"fajl sett tema personalizzat"#####,
    b"display_config_dir" => {
      r#####"uri direttorju tal-konfigurazzjoni ta' glossa"#####
    }
    b"dsl_suffix" => r#####"suffiss tal-fajl DSL (default: ".dsl")"#####,
    b"exclude_languages" => {
      r#####"modalità lista sewda: skip IDs lingwi listati"#####
    }
    b"exclude_map_names" => r#####"skip map_names listati"#####,
    b"include_languages" => {
      r#####"modalità lista bajda: initialize biss IDs lingwi listati"#####
    }
    b"include_map_names" => r#####"initialize biss map_names listati"#####,
    b"input" => r#####"direttorju tas-sors għar-riżorsi tal-lokalizzazzjoni"#####,
    b"list_all_syntaxes" => {
      r#####"uri ismijiet sintassi u estensjonijiet kollha"#####
    }
    b"list_all_themes" => r#####"uri ismijiet tema kollha"#####,
    b"mod_prefix" => r#####"prefiss tal-fajl mod (default: "l10n_")"#####,
    b"outdir" => r#####"direttorju tal-output"#####,
    b"output_bincode" => {
      r#####"toħloq fajls bincode separati għal lingwi differenti"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"esporta bincode ta' lingwi kollha f'fajl wieħed"#####
    }
    b"output_locales_fn" => r#####"esporta funzjoni all_locales"#####,
    b"output_match_fn" => {
      r#####"toħloq fajls Rust b'funzjonijiet match għal kull lingwa"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"ikkonsolida data ta' lingwi kollha f'funzjoni match waħda (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"funzjoni match b'isem tal-lingwa bħala ċavetta"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"funzjoni match b'ċavetti kombinati (lingwa + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"bħal output_match_fn iżda biss juża map_key bħala ċavetta"#####
    }
    b"output_phf" => {
      r#####"toħloq funzjonijiet phf map separati għal lingwi differenti"#####
    }
    b"output_phf_all_in_one" => {
      r#####"ikkonsolida phf map kollha f'funzjoni waħda"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map b'ċavetti string ordinarji (mhux TupleKey)"#####
    }
    b"output_ron" => r#####"esporta string f'format RON"#####,
    b"suffix" => r#####"suffiss għal Highlight-Map ġodda"#####,
    b"syntax_name" => r#####"isem tas-sintassi"#####,
    b"theme_name" => r#####"isem tat-tema"#####,
    b"true_color" => r#####"24-bit kulur veru"#####,
    b"visibility" => r#####"viżibilità tal-kodiġi ġenerat"#####,
    _ => "",
  }
}
