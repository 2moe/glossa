pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"بڪ گرائونڊ کي فعال ڪريو"#####,
    b"base_name" => r#####""Highlight-Map" جو بنيادي نالو"#####,
    b"bincode_suffix" => r#####"bincode فائيل سڏايو"#####,
    b"custom_syntax_set" => r#####"سِنٽيڪس سيٽ فائيل ڪسٽم"#####,
    b"custom_theme_set" => r#####"موضوع سيٽ فائيل ڪسٽم"#####,
    b"display_config_dir" => r#####"glossa جي سيٽ اپ ڊائريڪٽري ڏيکاريو"#####,
    b"dsl_suffix" => r#####"DSL فائيل سڏايو (ڊفالٽ ".dsl")"#####,
    b"exclude_languages" => r#####"بليڪ لسٽ موڊ: فهرست ۾ زبان ID شروع نہ ڪريو"#####,
    b"exclude_map_names" => r#####"فهرست ۾ نقشا نالا شروع نہ ڪريو"#####,
    b"include_languages" => {
      r#####"وائيٽ لسٽ موڊ: فهرست ۾ زبان ID صرف شروع ڪريو"#####
    }
    b"include_map_names" => r#####"فهرست ۾ نقشا نالا صرف شروع ڪريو"#####,
    b"input" => r#####"مقامي وسيلن جو اصل ڊائريڪٽري"#####,
    b"list_all_syntaxes" => r#####"سڀ سِنٽيڪس نالا ۽ وڌائڻ ڏيکاريو"#####,
    b"list_all_themes" => r#####"سڀ موضوع نالا ڏيکاريو"#####,
    b"mod_prefix" => r#####"mod فائيل اڳوڻو (ڊفالٽ "l10n_")"#####,
    b"outdir" => r#####"آئوٽ پُٽ ڊائريڪٽري"#####,
    b"output_bincode" => r#####"الڳ زبانن لاءِ الڳ bincode فائيلون ٺاهيو"#####,
    b"output_bincode_all_in_one" => {
      r#####"سڀني زبانن جا bincode هڪ فائيل ۾ ڪڍيو"#####
    }
    b"output_locales_fn" => r#####"all_locales فنڪشن ڪڍيو"#####,
    b"output_match_fn" => {
      r#####"الڳ زبانن لاءِ Rust ڪوڊ فائيلون (match اظهار وارا فنڪشن)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"سڀ ڊيٽا کي هڪ match فنڪشن ۾ ڪڍيو (اسٽرنگ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"زبان جي نالي سان ڪلي وارو match فنڪشن"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ڪمپوزٽ ڪلي (زبان + map_key) سان match فنڪشن"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"صرف map_key کي ڪلي طور استعمال ڪريو (map_name شامل ناهي)"#####
    }
    b"output_phf" => r#####"الڳ زبانن لاءِ phf نقشا فنڪشن ٺاهيو"#####,
    b"output_phf_all_in_one" => r#####"سڀ phf نقشا هڪ فنڪشن ۾ گڏيو"#####,
    b"output_phf_without_map_name" => r#####"عام اسٽرنگ ڪلي سان phf نقشا (TupleKey ناهي)"#####,
    b"output_ron" => r#####"RON فارميٽ ۾ اسٽرنگ ڪڍيو"#####,
    b"suffix" => r#####"نئين "Highlight-Map" جو سڏايو"#####,
    b"syntax_name" => r#####"سِنٽيڪس نالو"#####,
    b"theme_name" => r#####"موضوع جو نالو"#####,
    b"true_color" => r#####"24-بٽ اصل رنگ"#####,
    b"visibility" => r#####"جنريٽ ڪيل ڪوڊ جي نمائش"#####,
    _ => "",
  }
}
