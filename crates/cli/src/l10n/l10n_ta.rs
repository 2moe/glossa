pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"பின்னணியை இயக்கவும்"#####,
    b"base_name" => r#####"அடிப்படை "Highlight-Map" பெயர்"#####,
    b"bincode_suffix" => r#####"bincode கோப்பு பின்னொட்டு"#####,
    b"custom_syntax_set" => r#####"தனிப்பயன் தொடரியல் தொகுப்பு கோப்பு"#####,
    b"custom_theme_set" => r#####"தனிப்பயன் தீம் தொகுப்பு கோப்பு"#####,
    b"display_config_dir" => r#####"Glossa உள்ளமைவு அடைவை காட்டு"#####,
    b"dsl_suffix" => r#####"DSL கோப்பு பின்னொட்டு (இயல்புநிலை ".dsl")"#####,
    b"exclude_languages" => {
      r#####"கருப்பு பட்டியல்: பட்டியலில் உள்ள மொழி IDகள் துவக்கப்படாது"#####
    }
    b"exclude_map_names" => r#####"பட்டியலில் உள்ள வரைபட பெயர்கள் துவக்கப்படாது"#####,
    b"include_languages" => {
      r#####"வெள்ளை பட்டியல்: பட்டியலில் உள்ள மொழி IDகள் மட்டுமே துவக்கப்படும்"#####
    }
    b"include_map_names" => r#####"பட்டியலில் உள்ள வரைபட பெயர்கள் மட்டுமே துவக்கப்படும்"#####,
    b"input" => r#####"மொழியாக்க வளங்களின் மூல அடைவு"#####,
    b"list_all_syntaxes" => r#####"அனைத்து தொடரியல் பெயர்கள் & நீட்சிகளை காட்டு"#####,
    b"list_all_themes" => r#####"அனைத்து தீம் பெயர்களை காட்டு"#####,
    b"mod_prefix" => r#####"mod கோப்பு முன்னொட்டு (இயல்புநிலை "l10n_")"#####,
    b"outdir" => r#####"வெளியீட்டு அடைவு"#####,
    b"output_bincode" => {
      r#####"வேறுபட்ட மொழிகளுக்கு தனி bincode கோப்புகளை உருவாக்கு"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"அனைத்து மொழி bincodeகளை ஒரு கோப்பில் வெளியிடு"#####
    }
    b"output_locales_fn" => r#####"all_locales செயல்பாட்டை வெளியிடு"#####,
    b"output_match_fn" => {
      r#####"match வெளிப்பாடு செயல்பாடுகளுடன் Rust கோப்புகளை உருவாக்கு"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"அனைத்து தரவுகளை ஒரு match செயல்பாடாக வெளியிடு (சரம்)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"மொழி பெயரை சாவியாக கொண்ட match செயல்பாடு"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"மொழி + map_key சாவியுடன் match செயல்பாடு"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"map_key மட்டுமே சாவியாக பயன்படுத்து (map_name இல்லை)"#####
    }
    b"output_phf" => r#####"மொழிகளுக்கு தனி phf வரைபட செயல்பாடுகள்"#####,
    b"output_phf_all_in_one" => r#####"அனைத்து phf வரைபடங்களை ஒரு செயல்பாடாக இணைக்க"#####,
    b"output_phf_without_map_name" => {
      r#####"எளிய சரம் சாவிகள் கொண்ட phf வரைபடங்கள் (TupleKey அல்ல)"#####
    }
    b"output_ron" => r#####"RON வடிவத்தில் சரம் வெளியிடு"#####,
    b"suffix" => r#####"புதிய "Highlight-Map" பின்னொட்டு"#####,
    b"syntax_name" => r#####"தொடரியல் பெயர்"#####,
    b"theme_name" => r#####"தீம் பெயர்"#####,
    b"true_color" => r#####"24-பிட் உண்மை நிறம்"#####,
    b"visibility" => r#####"உருவாக்கிய குறியீட்டின் காண்பித்தல்"#####,
    _ => "",
  }
}
