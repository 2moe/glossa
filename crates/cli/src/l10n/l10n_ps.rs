pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"شالید فعال کول"#####,
    b"base_name" => r#####"د "Highlight-Map" اصلي نوم"#####,
    b"bincode_suffix" => r#####"د bincode د فایل وروستي"#####,
    b"custom_syntax_set" => r#####"دودیز نحوي تنظیم فایل"#####,
    b"custom_theme_set" => r#####"دودیز تمې تنظیم فایل"#####,
    b"display_config_dir" => r#####"د glossa د تنظیماتو فولډر ښکاره کول"#####,
    b"dsl_suffix" => r#####"د DSL فایل وروستي (اصلي ".dsl")"#####,
    b"exclude_languages" => r#####"تور لېست: په لېست کې ژبې ID فعال مه کړئ"#####,
    b"exclude_map_names" => r#####"په لېست کې نقشه نومونه فعال مه کړئ"#####,
    b"include_languages" => r#####"سپینه لېست: یوازې په لېست کې ژبې ID فعالول"#####,
    b"include_map_names" => r#####"یوازې په لېست کې نقشه نومونه فعالول"#####,
    b"input" => r#####"د محلي سرچینو اصلی فولډر"#####,
    b"list_all_syntaxes" => r#####"ټول نحوي نومونه او غځونې ښکاره کول"#####,
    b"list_all_themes" => r#####"ټول تمې نومونه ښکاره کول"#####,
    b"mod_prefix" => r#####"د mod فایل مخکښ (اصلي "l10n_")"#####,
    b"outdir" => r#####"د تولید د فولډر"#####,
    b"output_bincode" => r#####"د مختلفو ژبو لپاره جلا bincode فایلونه جوړول"#####,
    b"output_bincode_all_in_one" => {
      r#####"ټولو ژبو bincode په یو فایل کې خپرول"#####
    }
    b"output_locales_fn" => r#####"د all_locales فانکشن خپرول"#####,
    b"output_match_fn" => {
      r#####"د مختلفو ژبو لپاره د match فانکشن سره Rust فایلونه جوړول"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"ټول ډیټا په یو match فانکشن کې خپرول (سترګه)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"د ژبې نوم د کیلي په توګه match فانکشن"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"د ترکیبي کیلي سره match فانکشن (ژبه + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"یوازې map_key د کیلي په توګه کارول (map_name نه لري)"#####
    }
    b"output_phf" => r#####"د مختلفو ژبو لپاره د phf نقشه فانکشنونه جوړول"#####,
    b"output_phf_all_in_one" => r#####"ټولې phf نقشې په یو فانکشن کې یوځای کول"#####,
    b"output_phf_by_key" => {
      r#####"د عامو سترګو کیلي سره phf نقشه (نه TupleKey)"#####
    }
    b"output_ron" => r#####"په RON بڼه کې سترګه خپرول"#####,
    b"suffix" => r#####"د نوي "Highlight-Map" وروستي"#####,
    b"syntax_name" => r#####"د نحوې نوم"#####,
    b"theme_name" => r#####"د تمې نوم"#####,
    b"true_color" => r#####"24-بټه اصلي رنګ"#####,
    b"visibility" => r#####"د جوړ شوي کوډ لید"#####,
    _ => "",
  }
}
