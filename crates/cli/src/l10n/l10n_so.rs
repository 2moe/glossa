pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Hawl-geli background"#####,
    b"base_name" => r#####"Magaca aasaasiga ah ee "Highlight-Map""#####,
    b"bincode_suffix" => r#####"suffixka faylka bincode"#####,
    b"custom_syntax_set" => r#####"Faylka syntax-ka gaarka ah"#####,
    b"custom_theme_set" => r#####"Faylka mawduucyada gaarka ah"#####,
    b"display_config_dir" => r#####"Tus directoorka qaabeynta glossa"#####,
    b"dsl_suffix" => r#####"Suffixka faylka DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Qaabka liistada madow: Ha bilaabin ID-yaasha luqadaha"#####
    }
    b"exclude_map_names" => r#####"Ha bilaabin magacyada khariidad liistada"#####,
    b"include_languages" => {
      r#####"Qaabka liistada cad: Kaliya ID-yaasha luqadaha liistada la bilaabiyo"#####
    }
    b"include_map_names" => {
      r#####"Kaliya magacyada khariidad liistada la bilaabiyo"#####
    }
    b"input" => r#####"Directoorka asalka khayraadka gobolaysiinta"#####,
    b"list_all_syntaxes" => {
      r#####"Tus dhammaan magacyada syntax iyo kordhintooda"#####
    }
    b"list_all_themes" => r#####"Tus dhammaan magacyada mawduucyada"#####,
    b"mod_prefix" => r#####"Prefixka faylka mod (default "l10n_")"#####,
    b"outdir" => r#####"Directoorka soo-saarka"#####,
    b"output_bincode" => r#####"Abuur faylal bincode kala duwan luqadaha"#####,
    b"output_bincode_all_in_one" => {
      r#####"Dhammaan bincode-yada luqadaha ku dar fayl hal ah"#####
    }
    b"output_locales_fn" => r#####"Soo saar shaqada all_locales"#####,
    b"output_match_fn" => {
      r#####"Abuur faylal Rust leh shaqooyin match kala duwan"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Wadaag dhammaan xogta mid ka mid ah shaqo match (xarriiq)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Shaqo match leh magaca luqada sida fure"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Shaqo match leh fure isku-dhafan (luqad+map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Kaliya isticmaal map_key sida fure (aan la socon map_name)"#####
    }
    b"output_phf" => r#####"Abuur shaqooyin phf map kala duwan luqadaha"#####,
    b"output_phf_all_in_one" => {
      r#####"Dhammaan phf map-yada ku dar shaqo hal ah"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map leh fureyo caadi ah (maahan TupleKey)"#####
    }
    b"output_ron" => r#####"Soo saar xarriiq qaabka RON"#####,
    b"suffix" => r#####"Suffixka cusub ee "Highlight-Map""#####,
    b"syntax_name" => r#####"Magaca syntax-ka"#####,
    b"theme_name" => r#####"Magaca mawduuca"#####,
    b"true_color" => r#####"Midab run ah 24-bit"#####,
    b"visibility" => r#####"Muujinta koodhka la abuuray"#####,
    _ => "",
  }
}
