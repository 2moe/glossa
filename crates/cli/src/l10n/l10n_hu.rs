pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Háttér aktiválása"#####,
    b"base_name" => r#####""Highlight-Map" alapnév"#####,
    b"bincode_suffix" => r#####"bincode fájl utótag"#####,
    b"custom_syntax_set" => r#####"Egyéni szintaxiskészlet fájl"#####,
    b"custom_theme_set" => r#####"Egyéni témakészlet fájl"#####,
    b"display_config_dir" => {
      r#####"A Glossa konfigurációs könyvtárának megjelenítése"#####
    }
    b"dsl_suffix" => r#####"DSL fájl utótag (alapértelmezett ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Feketelista mód: Ne aktiválja a listán lévő nyelvi ID-kat"#####
    }
    b"exclude_map_names" => r#####"Ne aktiválja a listán lévő térképneveket"#####,
    b"include_languages" => {
      r#####"Fehérlista mód: Csak listán lévő nyelvi ID-k aktiválása"#####
    }
    b"include_map_names" => r#####"Csak listán lévő térképnevek aktiválása"#####,
    b"input" => r#####"Lokalizációs erőforrások forráskönyvtára"#####,
    b"list_all_syntaxes" => {
      r#####"Összes szintaxisnév és kiterjesztés megjelenítése"#####
    }
    b"list_all_themes" => r#####"Összes témanév megjelenítése"#####,
    b"mod_prefix" => r#####"mod fájl előtag (alapértelmezett "l10n_")"#####,
    b"outdir" => r#####"Kimeneti könyvtár"#####,
    b"output_bincode" => {
      r#####"Különálló bincode fájlok generálása különböző nyelvekhez"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Összes nyelv bincode-jának exportálása egyetlen fájlba"#####
    }
    b"output_locales_fn" => r#####"All_locales függvény exportálása"#####,
    b"output_match_fn" => {
      r#####"Rust fájlok generálása match kifejezésekkel nyelvenként"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Összes adat összevonása egy match függvénybe (sztring)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match függvény nyelvi névvel mint kulcs"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match függvény kombinált kulccsal (nyelv + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Csak map_key használata kulcsként (map_name nélkül)"#####
    }
    b"output_phf" => r#####"PHF térképfüggvények generálása nyelvenként"#####,
    b"output_phf_all_in_one" => {
      r#####"Összes PHF térkép összevonása egy függvénybe"#####
    }
    b"output_phf_by_key" => {
      r#####"PHF térkép közönséges sztring kulcsokkal (nem TupleKey)"#####
    }
    b"output_ron" => r#####"Sztring exportálása RON formátumban"#####,
    b"suffix" => r#####"Új "Highlight-Map" utótag"#####,
    b"syntax_name" => r#####"Szintaxis név"#####,
    b"theme_name" => r#####"Téma név"#####,
    b"true_color" => r#####"24 bites valódi szín"#####,
    b"visibility" => r#####"Generált kód láthatósága"#####,
    _ => "",
  }
}
