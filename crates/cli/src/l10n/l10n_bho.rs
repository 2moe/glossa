pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"बैकग्राउंड चालू करव"#####,
    b"base_name" => r#####""हाइलाइट-मैप" के मूल नाम"#####,
    b"bincode_suffix" => r#####"bincode फाइल प्रत्यय"#####,
    b"custom_syntax_set" => r#####"कस्टम सिंटैक्स सेट फाइल"#####,
    b"custom_theme_set" => r#####"कस्टम थीम सेट फाइल"#####,
    b"display_config_dir" => r#####"glossa के कॉन्फिग डायरेक्टरी देखावल"#####,
    b"dsl_suffix" => r#####"DSL फाइल प्रत्यय (डिफॉल्ट ".dsl")"#####,
    b"exclude_languages" => {
      r#####"ब्लैकलिस्ट मोड: लिस्ट के भाषा ID के इनिशियलाइज ना करव"#####
    }
    b"exclude_map_names" => r#####"लिस्ट के मैप नाम के इनिशियलाइज ना करव"#####,
    b"include_languages" => {
      r#####"व्हाइटलिस्ट मोड: लिस्ट के भाषा ID के सिर्फ इनिशियलाइज करव"#####
    }
    b"include_map_names" => r#####"लिस्ट के मैप नाम के सिर्फ इनिशियलाइज करव"#####,
    b"input" => r#####"लोकलाइजेशन संसाधन के मूल डायरेक्टरी"#####,
    b"list_all_syntaxes" => r#####"सभ सिंटैक्स नाम आ एक्सटेंशन देखावल"#####,
    b"list_all_themes" => r#####"सभ थीम नाम देखावल"#####,
    b"mod_prefix" => r#####"mod फाइल उपसर्ग (डिफॉल्ट "l10n_")"#####,
    b"outdir" => r#####"आउटपुट डायरेक्टरी"#####,
    b"output_bincode" => r#####"अलग-अलग भाषा खातिर अलग bincode फाइल बनावल"#####,
    b"output_bincode_all_in_one" => {
      r#####"सभ भाषा के bincode के एकही फाइल में निर्यात करव"#####
    }
    b"output_locales_fn" => r#####"all_locales फंक्शन निर्यात करव"#####,
    b"output_match_fn" => {
      r#####"रस्ट कोड के फाइल बनावल (match एक्सप्रेशन वाला फंक्शन)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"सभ डेटा के एकही match फंक्शन में निर्यात करव (स्ट्रिंग)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"भाषा नाम के चाबी वाला match फंक्शन"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"मिलावट चाबी (भाषा+map_key) वाला match फंक्शन"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"सिर्फ map_key के चाबी बनावल (map_name ना राखल)"#####
    }
    b"output_phf" => r#####"अलग-अलग भाषा खातिर phf मैप फंक्शन बनावल"#####,
    b"output_phf_all_in_one" => r#####"सभ phf मैप के एकही फंक्शन में मिलावल"#####,
    b"output_phf_without_map_name" => {
      r#####"सादा स्ट्रिंग चाबी वाला phf मैप (TupleKey ना)"#####
    }
    b"output_ron" => r#####"RON फॉर्मेट में स्ट्रिंग निर्यात करव"#####,
    b"suffix" => r#####""हाइलाइट-मैप" के नया प्रत्यय"#####,
    b"syntax_name" => r#####"सिंटैक्स नाम"#####,
    b"theme_name" => r#####"थीम नाम"#####,
    b"true_color" => r#####"24-बिट असली रंग"#####,
    b"visibility" => r#####"बनल कोड के दृश्यता"#####,
    _ => "",
  }
}
