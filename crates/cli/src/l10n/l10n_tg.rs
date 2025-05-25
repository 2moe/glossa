pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Фаъол кардани замина"#####,
    b"base_name" => r#####"Номи асосии "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Пасонанди файли bincode"#####,
    b"custom_syntax_set" => r#####"Файли синтаксиси фармоишӣ"#####,
    b"custom_theme_set" => r#####"Файли мавзӯъҳои фармоишӣ"#####,
    b"display_config_dir" => r#####"Намоиши феҳристи танзимоти Glossa"#####,
    b"dsl_suffix" => r#####"Пасонанди файли DSL (пешфарз ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Реҷаи рӯйхати сиёҳ: ID-ҳои забонҳои рӯйхатшуда оғоз намеёбанд"#####
    }
    b"exclude_map_names" => r#####"Номҳои харитаҳои рӯйхатшуда оғоз намеёбанд"#####,
    b"include_languages" => {
      r#####"Реҷаи рӯйхати сафед: Танҳо ID-ҳои забонҳои рӯйхатшуда оғоз меёбанд"#####
    }
    b"include_map_names" => {
      r#####"Танҳо номҳои харитаҳои рӯйхатшуда оғоз меёбанд"#####
    }
    b"input" => r#####"Феҳристи манбаъи захираҳои маҳаллӣ"#####,
    b"list_all_syntaxes" => {
      r#####"Намоиши ҳамаи номҳои синтаксис ва васеъшавиҳо"#####
    }
    b"list_all_themes" => r#####"Намоиши ҳамаи номҳои мавзӯъҳо"#####,
    b"mod_prefix" => r#####"Пешванди файли mod (пешфарз "l10n_")"#####,
    b"outdir" => r#####"Феҳристи баровардаҳо"#####,
    b"output_bincode" => {
      r#####"Эҷоди файлҳои bincode-и алоҳида барои забонҳои гуногун"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Баровардани ҳамаи bincode-ҳо ба як файл"#####
    }
    b"output_locales_fn" => r#####"Баровардани функсияи all_locales"#####,
    b"output_match_fn" => {
      r#####"Эҷоди файлҳои Rust бо функсияҳои match барои забонҳо"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Ҳамаи додаҳоро дар як функсияи match (сатр) ҷамъ кунед"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Функсияи match бо номи забон ҳамчун калид"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Функсияи match бо калиди иловагӣ (забон + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Танҳо map_key-ро ҳамчун калид истифода кунед (бе map_name)"#####
    }
    b"output_phf" => r#####"Эҷоди функсияҳои phf map барои забонҳои гуногун"#####,
    b"output_phf_all_in_one" => {
      r#####"Ҳамаи phf map-ҳоро дар як функсия ҷамъ кунед"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map бо калидҳои сатри оддӣ (на TupleKey)"#####
    }
    b"output_ron" => r#####"Баровардани сатрҳо дар формати RON"#####,
    b"suffix" => r#####"Пасонанди "Highlight-Map"-и нав"#####,
    b"syntax_name" => r#####"Номи синтаксис"#####,
    b"theme_name" => r#####"Номи мавзӯъ"#####,
    b"true_color" => r#####"Ранги аслӣ 24-битӣ"#####,
    b"visibility" => r#####"Намоишаи коди сохташуда"#####,
    _ => "",
  }
}
