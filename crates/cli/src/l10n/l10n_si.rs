pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"පසෙක් පිටුපස සක්රිය කරන්නද"#####,
    b"base_name" => r#####""Highlight-Map" පදනම් නාමය"#####,
    b"bincode_suffix" => r#####"bincode ගොනු පසුප්රත්යය"#####,
    b"custom_syntax_set" => r#####"අභිරුචි වාක්ය රචන කට්ටල ගොනුව"#####,
    b"custom_theme_set" => r#####"අභිරුචි තේමා කට්ටල ගොනුව"#####,
    b"display_config_dir" => r#####"Glossa වින්යාස ඩිරෙක්ටරිය පෙන්වන්න"#####,
    b"dsl_suffix" => r#####"DSL ගොනු පසුප්රත්යය (පෙරනිමිය ".dsl")"#####,
    b"exclude_languages" => r#####"කලු ලැයිස්තු ප්රකාරය: ලැයිස්තු භාෂා ID ආරම්භ නොකරන්න"#####,
    b"exclude_map_names" => r#####"ලැයිස්තු සිතියම් නාම ආරම්භ නොකරන්න"#####,
    b"include_languages" => {
      r#####"සුදු ලැයිස්තු ප්රකාරය: ලැයිස්තුවේ භාෂා ID පමණක් ආරම්භ කරන්න"#####
    }
    b"include_map_names" => r#####"ලැයිස්තුවේ සිතියම් නාම පමණක් ආරම්භ කරන්න"#####,
    b"input" => r#####"ස්ථානීයකරණ සම්පත් මූලාශ්ර ඩිරෙක්ටරිය"#####,
    b"list_all_syntaxes" => r#####"සියලු වාක්ය රචන නාම සහ දිගු පෙන්වන්න"#####,
    b"list_all_themes" => r#####"සියලු තේමා නාම පෙන්වන්න"#####,
    b"mod_prefix" => r#####"mod ගොනු පෙරබෙදුම (පෙරනිමිය "l10n_")"#####,
    b"outdir" => r#####"ප්රතිදාන ඩිරෙක්ටරිය"#####,
    b"output_bincode" => r#####"විවිධ භාෂා සඳහා ස්වාධීන bincode ගොනු ජනනය කරන්න"#####,
    b"output_bincode_all_in_one" => {
      r#####"සියලු භාෂාවල bincode එක් ගොනුවකට නිර්යාත කරන්න"#####
    }
    b"output_locales_fn" => r#####"all_locales කාර්යය නිර්යාත කරන්න"#####,
    b"output_match_fn" => r#####"රස්ට් කේත ගොනු ජනනය කරන්න (match ප්රකාශන සහිත කාර්යයන්)"#####,
    b"output_match_fn_all_in_one" => {
      r#####"සියලු දත්ත match කාර්යයකට නිර්යාත කරන්න (පේළිය)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"භාෂා නාමය යතුරු ලෙස භාවිත කරන match කාර්යය"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"සංයුක්ත යතුරු සහිත match කාර්යය (භාෂා+map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key පමණක් යතුරක් ලෙස භාවිතා කරන්න (map_name අඩංගු නොවේ)"#####
    }
    b"output_phf" => r#####"භාෂාවලට වෙන්වූ phf සිතියම් කාර්යයන් ජනනය කරන්න"#####,
    b"output_phf_all_in_one" => r#####"සියලු phf සිතියම් එක් කාර්යයකට එක් කරන්න"#####,
    b"output_phf_by_key" => r#####"සාමාන්ය පේළි යතුරු සහිත phf සිතියම් (TupleKey නොවේ)"#####,
    b"output_ron" => r#####"RON ආකෘතියෙන් පේළිය නිර්යාත කරන්න"#####,
    b"suffix" => r#####""Highlight-Map" අලුත් පසුප්රත්යය"#####,
    b"syntax_name" => r#####"වාක්ය රචනයේ නාමය"#####,
    b"theme_name" => r#####"තේමා නාමය"#####,
    b"true_color" => r#####"24-බිට් සත්ය වර්ණ"#####,
    b"visibility" => r#####"ජනනය කරන ලද කේතයේ දෘශ්යතාව"#####,
    _ => "",
  }
}
