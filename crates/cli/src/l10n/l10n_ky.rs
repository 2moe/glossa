pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Фонду иштетүү"#####,
    b"base_name" => r#####""Highlight-Map" негизги аталышы"#####,
    b"bincode_suffix" => r#####"bincode файл суффикси"#####,
    b"custom_syntax_set" => r#####"Ыңгайлаштырылган синтаксис топтому"#####,
    b"custom_theme_set" => r#####"Ыңгайлаштырылган темалар топтому"#####,
    b"display_config_dir" => r#####"Glossa конфигурация директориясын көрсөтүү"#####,
    b"dsl_suffix" => r#####"DSL файл суффикси (демейки ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Кара тизме режими: Тизме тилдерин ишке киргизбөө"#####
    }
    b"exclude_map_names" => r#####"Тизмедеги карта аттарын ишке киргизбөө"#####,
    b"include_languages" => {
      r#####"Ак тизме режими: Тизмедеги тил ID'лерин гана ишке киргизүү"#####
    }
    b"include_map_names" => r#####"Тизмедеги карта аттарын гана ишке киргизүү"#####,
    b"input" => r#####"Локалдаштырылган ресурсттардын башкы директориясы"#####,
    b"list_all_syntaxes" => r#####"Бардык синтаксис аталыштарын көрсөтүү"#####,
    b"list_all_themes" => r#####"Бардык темаларды көрсөтүү"#####,
    b"mod_prefix" => r#####"mod файл префикси (демейки "l10n_")"#####,
    b"outdir" => r#####"Чыгуу директориясы"#####,
    b"output_bincode" => r#####"Тилдер үчүн өз алдынча bincode файлдарды түзүү"#####,
    b"output_bincode_all_in_one" => {
      r#####"Бардык тилдердин bincode'ун бир файлга чыгаруу"#####
    }
    b"output_locales_fn" => r#####"all_locales функциясын чыгаруу"#####,
    b"output_match_fn" => {
      r#####"Rust коду файлдарын түзүү (match функциялары менен)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Бардык маалыматты бир match функцияга чыгаруу (сап)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Тил атын ачкыч кылган match функция"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Комбинацияланган ачкыч (тил+map_key) менен match функция"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key гана ачкыч катары колдонуу (map_name жок)"#####
    }
    b"output_phf" => r#####"Тилдер үчүн phf карта функцияларын түзүү"#####,
    b"output_phf_all_in_one" => {
      r#####"Бардык phf карталарды бир функцияга бириктирүү"#####
    }
    b"output_phf_by_key" => {
      r#####"Кадимки сап ачкычтары менен phf карта (TupleKey эмес)"#####
    }
    b"output_ron" => r#####"RON форматында сап чыгаруу"#####,
    b"suffix" => r#####""Highlight-Map" жаңы суффикси"#####,
    b"syntax_name" => r#####"Синтаксис аталышы"#####,
    b"theme_name" => r#####"Тема аталышы"#####,
    b"true_color" => r#####"24-бит чыныгы түс"#####,
    b"visibility" => r#####"Жасалган коддун көрүнүшү"#####,
    _ => "",
  }
}
