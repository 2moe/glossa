pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Washa Usuli wa Mandharinyuma"#####,
    b"base_name" => r#####"Jina la Msingi la "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Kiambishi cha Faili ya Bincode"#####,
    b"custom_syntax_set" => r#####"Faili maalum ya seti ya sintaksia"#####,
    b"custom_theme_set" => r#####"Faili maalum ya seti ya mada"#####,
    b"display_config_dir" => r#####"Onyesha folda ya usanidi ya Glossa"#####,
    b"dsl_suffix" => r#####"Kiambishi cha Faili ya DSL (chaguo-msingi ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Hali ya Orodha Nyeusi: Usianzishe ID za lugha katika orodha"#####
    }
    b"exclude_map_names" => r#####"Usianzishe majina ya ramani katika orodha"#####,
    b"include_languages" => {
      r#####"Hali ya Orodha Nyeupe: Weka kuanzisha ID za lugha katika orodha tu"#####
    }
    b"include_map_names" => {
      r#####"Weka kuanzisha majina ya ramani katika orodha tu"#####
    }
    b"input" => r#####"Folda chanzi ya rasilimali za ukalimani"#####,
    b"list_all_syntaxes" => {
      r#####"Onyesha majina yote ya sintaksia na viambishi"#####
    }
    b"list_all_themes" => r#####"Onyesha majina yote ya mada"#####,
    b"mod_prefix" => {
      r#####"Kiambishi cha Awali cha Faili ya Mod (chaguo-msingi "l10n_")"#####
    }
    b"outdir" => r#####"Folda ya Matokeo"#####,
    b"output_bincode" => {
      r#####"Tengeneza faili za Bincode tofauti kwa lugha mbalimbali"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Toa Bincode za lugha zote kwa faili moja"#####
    }
    b"output_locales_fn" => r#####"Toa kazi ya all_locales"#####,
    b"output_match_fn" => {
      r#####"Tengeneza faili za Rust zenye vitendakazi vya match"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Toa data zote kwa kazi moja ya match (mfululizo)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Kazi ya match yenye jina la lugha kama ufunguo"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Kazi ya match yenye ufunguo mchanganyiko (jina la lugha + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Tumia map_key pekee kama ufunguo (bila map_name)"#####
    }
    b"output_phf" => r#####"Tengeneza kazi za PHF Map tofauti kwa lugha"#####,
    b"output_phf_all_in_one" => r#####"Unganisha PHF Map zote kwa kazi moja"#####,
    b"output_phf_without_map_name" => {
      r#####"PHF Map yenye vfunguo vya kawaida vya mfululizo (sio TupleKey)"#####
    }
    b"output_ron" => r#####"Toa mfululizo katika umbizo la RON"#####,
    b"suffix" => r#####"Kiambishi kipya cha "Highlight-Map""#####,
    b"syntax_name" => r#####"Jina la Sintaksia"#####,
    b"theme_name" => r#####"Jina la Mada"#####,
    b"true_color" => r#####"Rangi halisi 24-bit"#####,
    b"visibility" => r#####"Uonekano wa Msimbo Ulioundwa"#####,
    _ => "",
  }
}
