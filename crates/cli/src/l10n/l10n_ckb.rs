pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"چالاککردنی پاشبنەما"#####,
    b"base_name" => r#####"ناوی بنەڕەتی "Highlight-Map""#####,
    b"bincode_suffix" => r#####"پاشەکەوتی فایلی bincode"#####,
    b"custom_syntax_set" => r#####"فایلی دەستکاریکراوی سینتاکس"#####,
    b"custom_theme_set" => r#####"فایلی دەستکاریکراوی تێم"#####,
    b"display_config_dir" => r#####"دایرێکتۆری ڕێکخستنی glossa پیشانبدە"#####,
    b"dsl_suffix" => r#####"پاشەکەوتی فایلی DSL (بنەڕەت ".dsl")"#####,
    b"exclude_languages" => {
      r#####"دۆخی لیستی ڕەش: IDی زمانە لیستکراوەکان دەستپێناکەن"#####
    }
    b"exclude_map_names" => r#####"ناوە mapە لیستکراوەکان دەستپێناکەن"#####,
    b"include_languages" => {
      r#####"دۆخی لیستی سپی: تەنها IDی زمانە لیستکراوەکان دەستپێدەکەن"#####
    }
    b"include_map_names" => r#####"تەنها ناوە mapە لیستکراوەکان دەستپێدەکەن"#####,
    b"input" => r#####"دایرێکتۆری سەرچاوەی سەرچاوەکانی لوکالایزکردن"#####,
    b"list_all_syntaxes" => {
      r#####"هەموو ناوەکانی سینتاکس و گەشەپێدانەکان پیشانبدە"#####
    }
    b"list_all_themes" => r#####"هەموو ناوەکانی تێمەکان پیشانبدە"#####,
    b"mod_prefix" => r#####"پێشگری فایلی mod (بنەڕەت "l10n_")"#####,
    b"outdir" => r#####"دایرێکتۆریی دەرچوون"#####,
    b"output_bincode" => {
      r#####"فایلی bincodeی سەربەخۆ دروست بکە بۆ زمانە جیاوازەکان"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"هەموو bincodeی زمانەکان لە فایلێکی تاکدا دەرچوون"#####
    }
    b"output_locales_fn" => r#####"فەنکشنی all_locales دەرچوون"#####,
    b"output_match_fn" => {
      r#####"فایلی کۆدی RUSTی سەربەخۆ دروست بکە بۆ زمانەکان (بە فەنکشنی match)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"هەموو داتاکان لە فەنکشنێکی matchی تاکدا دەرچوون (سٹرینگ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"فەنکشنی match بە کلیلی ناوی زمان"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"فەنکشنی match بە کلیلی (نامە زمان + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"وەک output_match_fn، بەڵام تەنها map_key وەک کلیلی فەنکشن"#####
    }
    b"output_phf" => r#####"فەنکشنی phf mapی سەربەخۆ دروست بکە بۆ زمانەکان"#####,
    b"output_phf_all_in_one" => {
      r#####"هەموو phf mapەکان لە فەنکشنێکی تاکدا یەکبخە"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map بە کلیلی سترینگی ئاسایی (نەک TupleKey)"#####
    }
    b"output_ron" => r#####"دەرچوون بە شێوازی RON"#####,
    b"suffix" => r#####"پاشەکەوتی "Highlight-Map"ی نوێ"#####,
    b"syntax_name" => r#####"ناوی سینتاکس"#####,
    b"theme_name" => r#####"ناوی تێم"#####,
    b"true_color" => r#####"ڕەنگی ڕاستەقینەی 24-bit"#####,
    b"visibility" => r#####"دیتنەوەی کۆدی دروستکراو"#####,
    _ => "",
  }
}
