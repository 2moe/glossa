pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ṣàmúlò àbáwọ̀lẹ̀"#####,
    b"base_name" => r#####"Orúkọ ipilẹ̀ "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Àkẹ́yìn fáìlì bincode"#####,
    b"custom_syntax_set" => r#####"Fáìlì èròngbà àṣà"#####,
    b"custom_theme_set" => r#####"Fáìlì àwọn àkọ́lé àṣà"#####,
    b"display_config_dir" => r#####"Fihàn àtòjọ ìṣàkóso glossa"#####,
    b"dsl_suffix" => r#####"Àkẹ́yìn fáìlì DSL (aṣẹ̀dá ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Àpèjú àwòrán dúdú: Má ṣàtúnṣe ID èdè tí ó wà nínú àtòjọ"#####
    }
    b"exclude_map_names" => {
      r#####"Má ṣàtúnṣe àwọn orúkọ map tí ó wà nínú àtòjọ"#####
    }
    b"include_languages" => {
      r#####"Àpèjú àwòrán funfun: Ṣàtúnṣe ID èdè tí ó wà nínú àtòjọ"#####
    }
    b"include_map_names" => r#####"Ṣàtúnṣe àwọn orúkọ map tí ó wà nínú àtòjọ"#####,
    b"input" => r#####"Àtòjọ orísun ìṣàkóso èdè"#####,
    b"list_all_syntaxes" => r#####"Fihàn gbogbo orúkọ èròngbà àti àfikún"#####,
    b"list_all_themes" => r#####"Fihàn gbogbo orúkọ àkọ́lé"#####,
    b"mod_prefix" => r#####"Àtẹ̀yìn fáìlì mod (aṣẹ̀dá "l10n_")"#####,
    b"outdir" => r#####"Àpótí Ìṣejáde"#####,
    b"output_bincode" => r#####"Ṣẹ̀dá fáìlì bincode oríṣiríṣi fún èdè"#####,
    b"output_bincode_all_in_one" => {
      r#####"Ṣe ìṣejáde gbogbo bincode èdè sí fáìlì kan"#####
    }
    b"output_locales_fn" => r#####"Ṣe ìṣejáde iṣẹ́ all_locales"#####,
    b"output_match_fn" => r#####"Ṣẹ̀dá fáìlì Rust pẹ̀lú iṣẹ́ match fún èdè"#####,
    b"output_match_fn_all_in_one" => {
      r#####"Ṣe ìṣejáde gbogbo dátà sí iṣẹ́ match kan (ọ̀rọ̀-ńtàkùn)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Iṣẹ́ match pẹ̀lú orúkọ èdè gẹ́gẹ́ bí key"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Iṣẹ́ match pẹ̀lú key (orúkọ èdè + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Jẹ́rìí output_match_fn, ṣùgbọ́n map_key nìkan ló jẹ́ key"#####
    }
    b"output_phf" => r#####"Ṣẹ̀dá iṣẹ́ phf map oríṣiríṣi fún èdè"#####,
    b"output_phf_all_in_one" => r#####"Ṣe ìṣejáde gbogbo phf map sí iṣẹ́ kan"#####,
    b"output_phf_by_key" => {
      r#####"phf map pẹ̀lú ọ̀rọ̀-ńtàkùn deede (kì í ṣe TupleKey)"#####
    }
    b"output_ron" => r#####"Ṣe ìṣejáde ọ̀rọ̀-ńtàkùn fọ́rmátì ron"#####,
    b"suffix" => r#####"Àkẹ́yìn fún "Highlight-Map" tuntun"#####,
    b"syntax_name" => r#####"Orúkọ èròngbà"#####,
    b"theme_name" => r#####"Orúkọ àkọ́lé"#####,
    b"true_color" => r#####"Àwọ̀ títọ́ 24-bítì"#####,
    b"visibility" => r#####"Ìfihàn kóòdù tí a ṣẹ̀dá"#####,
    _ => "",
  }
}
