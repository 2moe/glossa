pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"पृष्ठभूमि सक्रिय करू"#####,
    b"base_name" => r#####"मूल "हाइलाइट-मैप" क नाम"#####,
    b"bincode_suffix" => r#####"bincode फाइल क अंत"#####,
    b"custom_syntax_set" => r#####"कस्टम वाक्य रचना सेट फाइल"#####,
    b"custom_theme_set" => r#####"कस्टम विषय सेट फाइल"#####,
    b"display_config_dir" => r#####"glossa क कॉन्फ़िग डायरेक्टरी देखाऊ"#####,
    b"dsl_suffix" => r#####"DSL फाइल क अंत (डिफ़ॉल्ट ".dsl")"#####,
    b"exclude_languages" => r#####"कालो सूची मोड: सूची मे भाषा ID प्रारंभ नहि करू"#####,
    b"exclude_map_names" => r#####"सूची मे map_names प्रारंभ नहि करू"#####,
    b"include_languages" => r#####"श्वेत सूची मोड: सूची मे भाषा ID केवल प्रारंभ करू"#####,
    b"include_map_names" => r#####"सूची मे map_names केवल प्रारंभ करू"#####,
    b"input" => r#####"स्थानीयकरण संसाधन क स्रोत डायरेक्टरी"#####,
    b"list_all_syntaxes" => r#####"सभी वाक्य रचना नाम आ विस्तार प्रदर्शित करू"#####,
    b"list_all_themes" => r#####"सभी विषय नाम प्रदर्शित करू"#####,
    b"mod_prefix" => r#####"mod फाइल क उपसर्ग (डिफ़ॉल्ट "l10n_")"#####,
    b"outdir" => r#####"आउटपुट डायरेक्टरी"#####,
    b"output_bincode" => r#####"भिन्न भाषा क लेल अलग bincode फाइल निर्माण करू"#####,
    b"output_bincode_all_in_one" => {
      r#####"सभी भाषा क bincode एक फाइल मे निर्यात करू"#####
    }
    b"output_locales_fn" => r#####"all_locales फंक्शन निर्यात करू"#####,
    b"output_match_fn" => {
      r#####"भिन्न भाषा क लेल अलग रस्ट कोड फाइल (match एक्सप्रेसन फंक्शन सहित)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"सभी भाषा क डेटा एक match फंक्शन मे निर्यात करू (स्ट्रिंग)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"भाषा नाम key के रूप मे match फंक्शन"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"भाषा नाम + map_key क संयुक्त key सहित match फंक्शन"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key केवल key के रूप मे, map_name शामिल नहि"#####
    }
    b"output_phf" => r#####"भिन्न भाषा क लेल अलग phf map फंक्शन निर्माण करू"#####,
    b"output_phf_all_in_one" => r#####"सभी phf map एक फंक्शन मे निर्यात करू"#####,
    b"output_phf_by_key" => {
      r#####"सामान्य स्ट्रिंग key सहित phf map (TupleKey नहि)"#####
    }
    b"output_ron" => r#####"RON फॉर्मेट स्ट्रिंग के रूप मे निर्यात करू"#####,
    b"suffix" => r#####"नव निर्मित "हाइलाइट-मैप" क प्रत्यय"#####,
    b"syntax_name" => r#####"वाक्य रचना क नाम"#####,
    b"theme_name" => r#####"विषय क नाम"#####,
    b"true_color" => r#####"24-बिट असली रंग"#####,
    b"visibility" => r#####"जेनरेट कएल कोड क दृश्यता"#####,
    _ => "",
  }
}
