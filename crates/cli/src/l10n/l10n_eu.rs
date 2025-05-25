pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Atzeko planoa aktibatu"#####,
    b"base_name" => r#####""Highlight-Map" oinarrizko izena"#####,
    b"bincode_suffix" => r#####"bincode fitxategi luzapena"#####,
    b"custom_syntax_set" => r#####"Ohiko sintaxi multzoa fitxategia"#####,
    b"custom_theme_set" => r#####"Ohiko gai multzoa fitxategia"#####,
    b"display_config_dir" => {
      r#####"Erakutsi Glossaren konfigurazio direktorioa"#####
    }
    b"dsl_suffix" => r#####"DSL fitxategi luzapena (lehenetsia ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Zerrenda beltza modua: Ez hasieratu zerrendako hizkuntza IDak"#####
    }
    b"exclude_map_names" => r#####"Ez hasieratu zerrendako mapa izenak"#####,
    b"include_languages" => {
      r#####"Zerrenda zuri modua: Zerrendako hizkuntza IDak soilik hasieratu"#####
    }
    b"include_map_names" => r#####"Zerrendako mapa izenak soilik hasieratu"#####,
    b"input" => r#####"Lokalizazio baliabideen iturburu direktorioa"#####,
    b"list_all_syntaxes" => r#####"Erakutsi sintaxi izen eta luzapen guztiak"#####,
    b"list_all_themes" => r#####"Erakutsi gai izen guztiak"#####,
    b"mod_prefix" => r#####"mod fitxategi aurrizkia (lehenetsia "l10n_")"#####,
    b"outdir" => r#####"Irteerako direktorioa"#####,
    b"output_bincode" => {
      r#####"Hizkuntza ezberdinetarako bincode fitxategi independenteak sortu"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Hizkuntza guztien bincodeak fitxategi bakar batean esportatu"#####
    }
    b"output_locales_fn" => r#####"all_locales funtzioa esportatu"#####,
    b"output_match_fn" => {
      r#####"Rust kodea fitxategiak sortu (match adierazpenak dituzten funtzioekin)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Datu guztiak match funtzio bakar batean esportatu (katea)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Hizkuntza izena gako gisa duen match funtzioa"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Konbinatutako gakoa (hizkuntza + map_key) duen match funtzioa"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"map_key bakarrik erabili gako gisa (map_name ez da sartzen)"#####
    }
    b"output_phf" => {
      r#####"Hizkuntza ezberdinetarako phf mapa funtzio independenteak sortu"#####
    }
    b"output_phf_all_in_one" => {
      r#####"phf mapa guztiak funtzio bakar batean konbinatu"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf mapa kate arrunteko gakorekin (ez TupleKey)"#####
    }
    b"output_ron" => r#####"Esportatu katea RON formatuan"#####,
    b"suffix" => r#####""Highlight-Map" berriaren luzapena"#####,
    b"syntax_name" => r#####"Sintaxiaren izena"#####,
    b"theme_name" => r#####"Gaiaren izena"#####,
    b"true_color" => r#####"24 biteko kolore benetakoa"#####,
    b"visibility" => r#####"Sortutako kodea ikusgaitasuna"#####,
    _ => "",
  }
}
