pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Фонны активлаштыру"#####,
    b"base_name" => r#####""Highlight-Map" нигез исеме"#####,
    b"bincode_suffix" => r#####"bincode файл суффиксы"#####,
    b"custom_syntax_set" => r#####"Көйләнә торган синтаксис җыелмасы файлы"#####,
    b"custom_theme_set" => r#####"Көйләнә торган тема җыелмасы файлы"#####,
    b"display_config_dir" => r#####"Glossa көйләүләре директориясен күрсәтү"#####,
    b"dsl_suffix" => r#####"DSL файл суффиксы (көйләнмәгән ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Кара исемлек режимы: Исемлектәге тел ID'ларын башламау"#####
    }
    b"exclude_map_names" => r#####"Исемлектәге карта исемнәрен башламау"#####,
    b"include_languages" => {
      r#####"Ак исемлек режимы: Исемлектәге тел ID'ларын гына башлау"#####
    }
    b"include_map_names" => r#####"Исемлектәге карта исемнәрен гына башлау"#####,
    b"input" => r#####"Локализация ресурсларының чыганак директориясе"#####,
    b"list_all_syntaxes" => {
      r#####"Барлык синтаксис исемнәрен һәм киңәйтмәләрне күрсәтү"#####
    }
    b"list_all_themes" => r#####"Барлык тема исемнәрен күрсәтү"#####,
    b"mod_prefix" => r#####"mod файл префиксы (көйләнмәгән "l10n_")"#####,
    b"outdir" => r#####"Чыгыш директориясе"#####,
    b"output_bincode" => r#####"Төрле телләр өчен аерым bincode файллар ясау"#####,
    b"output_bincode_all_in_one" => {
      r#####"Барлык телләрнең bincode'ын бер файлга чыгару"#####
    }
    b"output_locales_fn" => r#####"all_locales функциясен чыгару"#####,
    b"output_match_fn" => {
      r#####"Төрле телләр өчен match функцияле Rust файллар ясау"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Барлык мәгълүматны бер match функциясенә чыгару (юл)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Тел исеме ачкыч булган бердәм match функциясе"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Комбик ачкычлы (тел исеме + map_key) match функциясе"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_key'ны генә ачкыч итеп куллану (map_name керми)"#####
    }
    b"output_phf" => r#####"Төрле телләр өчен аерым phf карта функцияләре ясау"#####,
    b"output_phf_all_in_one" => {
      r#####"Барлык phf карталарны бер функциягә берләштерү"#####
    }
    b"output_phf_by_key" => r#####"Гади юл ачкычлы phf карта (TupleKey түгел)"#####,
    b"output_ron" => r#####"RON форматтагы юлны чыгару"#####,
    b"suffix" => r#####"Яңа "Highlight-Map" суффиксы"#####,
    b"syntax_name" => r#####"Синтаксис исеме"#####,
    b"theme_name" => r#####"Тема исеме"#####,
    b"true_color" => r#####"24-битлы чын төс"#####,
    b"visibility" => r#####"Яратылган кодның күренүчәнлеге"#####,
    _ => "",
  }
}
