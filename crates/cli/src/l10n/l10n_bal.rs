pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"پس منظر اینیبل کٙرٙ"#####,
    b"base_name" => r#####""Highlight-Map" کٙا بنیادی ناں"#####,
    b"bincode_suffix" => r#####"bincode فائل سَڄٙ"#####,
    b"custom_syntax_set" => r#####"کسٹم سِنٹیکس سیٹ فائل"#####,
    b"custom_theme_set" => r#####"کسٹم تھیم سیٹ فائل"#####,
    b"display_config_dir" => r#####"glossa کٙے کانفیگ ڊائریکٹری ڏیٙکھاؤ"#####,
    b"dsl_suffix" => r#####"DSL فائل سَڄٙ (ڊیفالٹ ".dsl")"#####,
    b"exclude_languages" => {
      r#####"سیاه لیست موڊ: لسٹ اِچ زبان آئی ڊیز اینیٙت نٙہ کٙرٙ"#####
    }
    b"exclude_map_names" => r#####"لسٹ اِچ میپ ناں اینیٙت نٙہ کٙرٙ"#####,
    b"include_languages" => {
      r#####"سفید لیست موڊ: صرف لسٹ اِچ زبان آئی ڊیز اینیٙت کٙرٙ"#####
    }
    b"include_map_names" => r#####"صرف لسٹ اِچ میپ ناں اینیٙت کٙرٙ"#####,
    b"input" => r#####"لوکلائزیشن وسائل کٙا ماخذ ڊائریکٹری"#####,
    b"list_all_syntaxes" => r#####"سٙبھ سِنٹیکس ناں تٙہ ایکسٹینشنز ڏیٙکھاؤ"#####,
    b"list_all_themes" => r#####"سٙبھ تھیم ناں ڏیٙکھاؤ"#####,
    b"mod_prefix" => r#####"mod فائل پیشوند (ڊیفالٹ "l10n_")"#####,
    b"outdir" => r#####"آوٙٹ ڊائریکٹری"#####,
    b"output_bincode" => r#####"ویکّھے زباناں کٙی خودمختار bincode فائلاں ٹیار کٙرٙ"#####,
    b"output_bincode_all_in_one" => {
      r#####"سٙبھ زباناں کٙے bincodes ہک فائل اِچ ایکسپورٹ کٙرٙ"#####
    }
    b"output_locales_fn" => r#####"all_locales فَنکشن ایکسپورٹ کٙرٙ"#####,
    b"output_match_fn" => {
      r#####"روست کوڊ فائلاں ٹیار کٙرٙ (match اِظہار والے فَنکشنز)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"سٙبھ ڊیٹا ہک match فَنکشن اِچ ایکسپورٹ کٙرٙ (سٹرنگ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"زبان کٙے ناں سان کلید والی match فَنکشن"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"مُکٙل کلید (زبان + map_key) سان match فَنکشن"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"صرف map_key کلید طور استعمال کٙرٙ (map_name شامل نٙہ)"#####
    }
    b"output_phf" => r#####"ویکّھے زباناں کٙی خودمختار phf میپ فَنکشنز"#####,
    b"output_phf_all_in_one" => r#####"سٙبھ phf میپز ہک فَنکشن اِچ مٙلاءِ کٙرٙ"#####,
    b"output_phf_without_map_name" => r#####"عام سٹرنگ کلیداں سان phf میپ (TupleKey نٙہ)"#####,
    b"output_ron" => r#####"RON فارمیٹ اِچ سٹرنگ ایکسپورٹ کٙرٙ"#####,
    b"suffix" => r#####""Highlight-Map" کٙا نواں سَڄٙ"#####,
    b"syntax_name" => r#####"سِنٹیکس ناں"#####,
    b"theme_name" => r#####"موضوع ناں"#####,
    b"true_color" => r#####"24-بٹ اصلی رنگ"#####,
    b"visibility" => r#####"ڄمٙل کٙوڊ جو نظارٙو"#####,
    _ => "",
  }
}
