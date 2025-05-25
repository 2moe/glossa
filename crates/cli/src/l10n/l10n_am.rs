pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"ዳቦሳ አገልግል"#####,
    b"base_name" => r#####""ብሩህ-ካርታ" መሰረታዊ ስም"#####,
    b"bincode_suffix" => r#####"bincode ፋይል ቅጥያ"#####,
    b"custom_syntax_set" => r#####"ብጉር የሰዋሰው ስብስብ ፋይል"#####,
    b"custom_theme_set" => r#####"ብጉር የገጽታ ስብስብ ፋይል"#####,
    b"display_config_dir" => r#####"የglossa ውቅር ማውጫ አሳይ"#####,
    b"dsl_suffix" => r#####"DSL ፋይል ቅጥያ (ነባሪ ".dsl")"#####,
    b"exclude_languages" => r#####"የጥቁር ዝርዝር ሁነታ፡ በዝርዝሩ ውስጥ ያሉ የቋንቋ IDዎችን አትነሳ"#####,
    b"exclude_map_names" => r#####"በዝርዝሩ ውስጥ ያሉ የካርታ ስሞችን አትነሳ"#####,
    b"include_languages" => {
      r#####"የነጭ ዝርዝር ሁነታ፡ ዝርዝሩ ባዶ ካልሆነ ብቻ የቋንቋ IDዎችን አስጀምር"#####
    }
    b"include_map_names" => r#####"ዝርዝሩ ባዶ ካልሆነ ብቻ የካርታ ስሞችን አስጀምር"#####,
    b"input" => r#####"የአካባቢ ሀብቶች ምንጭ ማውጫ"#####,
    b"list_all_syntaxes" => r#####"ሁሉንም የሰዋሰው ስሞች እና ቅጥያዎች አሳይ"#####,
    b"list_all_themes" => r#####"ሁሉንም የገጽታ ስሞች አሳይ"#####,
    b"mod_prefix" => r#####"mod ፋይል ቅድመ ቅጥያ (ነባሪ "l10n_")"#####,
    b"outdir" => r#####"የውጤት ማውጫ"#####,
    b"output_bincode" => r#####"ለተለያዩ ቋንቋዎች የተለዩ bincode ፋይሎችን ፍጠር"#####,
    b"output_bincode_all_in_one" => {
      r#####"ሁሉንም ቋንቋዎች የbincode ፋይሎችን በአንድ ፋይል ላይ አስገባ"#####
    }
    b"output_locales_fn" => r#####"all_locales ፋይልን አስገባ"#####,
    b"output_match_fn" => {
      r#####"ለተለያዩ ቋንቋዎች Rust ኮድ ፋይሎችን ፍጠር (match አባባሎች ያሉት)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"ሁሉንም ውሂብ በአንድ match ፋይል ውስጥ አስገባ (ገለፃ ገበታ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ቋንቋ ስም እንደ ቁልፍ ያለው match ፋይል"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ተዋህዶ ቁልፍ (ቋንቋ+map_key) ያለው match ፋይል"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"map_key ብቻ እንደ ቁልፍ ይጠቀሙ (map_name አይጨምር)"#####
    }
    b"output_phf" => r#####"ለተለያዩ ቋንቋዎች phf ካርታ ፋይሎችን ፍጠር"#####,
    b"output_phf_all_in_one" => r#####"ሁሉንም phf ካርታዎች በአንድ ፋይል ውስጥ አስገባ"#####,
    b"output_phf_without_map_name" => {
      r#####"መደበኛ ገለፃ ቁልፍ ያለው phf ካርታ (TupleKey አይደለም)"#####
    }
    b"output_ron" => r#####"RON ፎርማት ያለውን ገለፃ አስገባ"#####,
    b"suffix" => r#####""ብሩህ-ካርታ" አዲስ ቅጥያ"#####,
    b"syntax_name" => r#####"የሰዋሰው ስም"#####,
    b"theme_name" => r#####"የገጽታ ስም"#####,
    b"true_color" => r#####"24-ቢት እውነተኛ ቀለም"#####,
    b"visibility" => r#####"የተፈጠረው ኮድ ታይነት"#####,
    _ => "",
  }
}
