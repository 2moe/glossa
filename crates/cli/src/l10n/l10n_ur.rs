pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"بیک گراؤنڈ فعال کریں"#####,
    b"base_name" => r#####""Highlight-Map" کا بنیادی نام"#####,
    b"bincode_suffix" => r#####"bincode فائل کا سابقہ"#####,
    b"custom_syntax_set" => r#####"خصوصی نحو سیٹ فائل"#####,
    b"custom_theme_set" => r#####"خصوصی تھیم سیٹ فائل"#####,
    b"display_config_dir" => r#####"Glossa کی کنفیگریشن ڈائریکٹری دکھائیں"#####,
    b"dsl_suffix" => r#####"DSL فائل کا سابقہ (ڈیفالٹ ".dsl")"#####,
    b"exclude_languages" => {
      r#####"سیاہ فہرست موڈ: فہرست والی زبان IDs شروع نہ کریں"#####
    }
    b"exclude_map_names" => r#####"فہرست والے میپ نام شروع نہ کریں"#####,
    b"include_languages" => {
      r#####"سفید فہرست موڈ: صرف فہرست والی زبان IDs کو شروع کریں"#####
    }
    b"include_map_names" => r#####"صرف فہرست والے میپ ناموں کو شروع کریں"#####,
    b"input" => r#####"مقامی وسائل کا ماخذ ڈائریکٹری"#####,
    b"list_all_syntaxes" => r#####"تمام نحو ناموں اور ایکسٹینشنز دکھائیں"#####,
    b"list_all_themes" => r#####"تمام تھیم نام دکھائیں"#####,
    b"mod_prefix" => r#####"mod فائل کا سابقہ (ڈیفالٹ "l10n_")"#####,
    b"outdir" => r#####"آؤٹ پٹ ڈائریکٹری"#####,
    b"output_bincode" => {
      r#####"مختلف زبانوں کے لیے علیحدہ bincode فائلیں تخلیق کریں"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"تمام زبانوں کے bincode ایک فائل میں برآمد کریں"#####
    }
    b"output_locales_fn" => r#####"all_locales فنکشن برآمد کریں"#####,
    b"output_match_fn" => {
      r#####"مختلف زبانوں کے لیے Rust کوڈ فائلیں (match ایکسپریشنز والے فنکشنز)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"تمام ڈیٹا کو ایک match فنکشن میں برآمد کریں (سٹرنگ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"زبان کے نام کو کلید بنانے والا match فنکشن"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"مشترکہ کلید (زبان+map_key) والا match فنکشن"#####
    }
    b"output_match_fn_by_key" => {
      r#####"صرف map_key کو کلید کے طور پر استعمال کریں (map_name شامل نہیں)"#####
    }
    b"output_phf" => r#####"مختلف زبانوں کے لیے علیحدہ phf میپ فنکشنز"#####,
    b"output_phf_all_in_one" => r#####"تمام phf میپز کو ایک فنکشن میں جمع کریں"#####,
    b"output_phf_by_key" => {
      r#####"عام سٹرنگ کلیدوں والی phf میپ (TupleKey نہیں)"#####
    }
    b"output_ron" => r#####"RON فارمیٹ میں سٹرنگ برآمد کریں"#####,
    b"suffix" => r#####""Highlight-Map" کا نیا سابقہ"#####,
    b"syntax_name" => r#####"نحو کا نام"#####,
    b"theme_name" => r#####"تھیم کا نام"#####,
    b"true_color" => r#####"24-بٹ اصلی رنگ"#####,
    b"visibility" => r#####"تخلیق شدہ کوڈ کی نظاریت"#####,
    _ => "",
  }
}
