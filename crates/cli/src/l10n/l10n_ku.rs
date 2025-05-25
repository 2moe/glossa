pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Rûxarê Çalak Bike"#####,
    b"base_name" => r#####"Navê Binyatê yê "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Paşpirtika Pelê Bincode"#####,
    b"custom_syntax_set" => r#####"Pelê Seta Sîntaksa Xweser"#####,
    b"custom_theme_set" => r#####"Pelê Seta Tema Xweser"#####,
    b"display_config_dir" => r#####"Dîrektorîya Veavakirina Glossa Nîşan Bide"#####,
    b"dsl_suffix" => r#####"Paşpirtika Pelê DSL (Default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Moda Lîsteya Reş: IDyên Zimanên di Lîsteyê de Bikar Neîne"#####
    }
    b"exclude_map_names" => r#####"Navên Mapên di Lîsteyê de Bikar Neîne"#####,
    b"include_languages" => {
      r#####"Moda Lîsteya Spî: Tenê IDyên Zimanên di Lîsteyê de Bikar Bîne"#####
    }
    b"include_map_names" => r#####"Tenê Navên Mapên di Lîsteyê de Bikar Bîne"#####,
    b"input" => r#####"Dîrektorîya Çavkaniyên Lokalîzekirî"#####,
    b"list_all_syntaxes" => {
      r#####"Hemû Navên Sîntaksê û Berfirehiyan Nîşan Bide"#####
    }
    b"list_all_themes" => r#####"Hemû Navên Tema Nîşan Bide"#####,
    b"mod_prefix" => r#####"Pêşpirtika Pelê Mod (Default "l10n_")"#####,
    b"outdir" => r#####"Dîrektorîya Derketinê"#####,
    b"output_bincode" => r#####"Pelên Bincodeyê yên Cuda ji bo Zimanan Çêke"#####,
    b"output_bincode_all_in_one" => {
      r#####"Hemû Bincodeyên Zimanan di Pelê Yekane de Derxîne"#####
    }
    b"output_locales_fn" => r#####"Fonksiyona all_locales Derxîne"#####,
    b"output_match_fn" => {
      r#####"Pelên Kodê Rust yên Cuda bi Fonksiyonên Match-ê Çêke"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Hemû Daneyan di Fonksiyonek Match-ê de Derxîne (Zincîre)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonksiyona Match bi Navê Zimanê wekî Key"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fonksiyona Match bi Keya Pêkhatî (Ziman + Map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Tenê Map_key wekî Key Bikar Bîne (Map_name Tune)"#####
    }
    b"output_phf" => r#####"Fonksiyonên PHF Map yên Cuda ji bo Zimanan Çêke"#####,
    b"output_phf_all_in_one" => {
      r#####"Hemû PHF Map-ê di Fonksiyonekê de Bihevgire"#####
    }
    b"output_phf_without_map_name" => {
      r#####"PHF Map bi Keyên Zincîreya Normal (Ne TupleKey)"#####
    }
    b"output_ron" => r#####"Zincîreya Formata RON Derxîne"#####,
    b"suffix" => r#####"Paşpirtika Nû ya "Highlight-Map""#####,
    b"syntax_name" => r#####"Navê Sîntaksê"#####,
    b"theme_name" => r#####"Navê Tema"#####,
    b"true_color" => r#####"Rengê Rastîn ê 24-Bîtî"#####,
    b"visibility" => r#####"Dîtbarîya Kodê Çêkirî"#####,
    _ => "",
  }
}
