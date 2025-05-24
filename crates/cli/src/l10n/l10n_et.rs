pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kas taust on aktiveeritud"#####,
    b"base_name" => r#####""Highlight-Map" alusnimi"#####,
    b"bincode_suffix" => r#####"bincode faili laiend"#####,
    b"custom_syntax_set" => r#####"Kohandatud süntaksikogumi fail"#####,
    b"custom_theme_set" => r#####"Kohandatud teemadekogumi fail"#####,
    b"display_config_dir" => r#####"Kuva Glossa seadistuskaust"#####,
    b"dsl_suffix" => r#####"DSL faili laiend (vaikimisi ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Musta nimekirja režiim: Ära initsialiseeri loetelus olevaid keele ID-sid"#####
    }
    b"exclude_map_names" => {
      r#####"Ära initsialiseeri loetelus olevaid kaartide nimesid"#####
    }
    b"include_languages" => {
      r#####"Valge nimekirja režiim: Initsialiseeri ainult loetelus olevad keele ID-d"#####
    }
    b"include_map_names" => {
      r#####"Initsialiseeri ainult loetelus olevad kaartide nimed"#####
    }
    b"input" => r#####"Lokaliseeritud ressursside lähtekaust"#####,
    b"list_all_syntaxes" => r#####"Kuva kõik süntaksinimed ja laiendid"#####,
    b"list_all_themes" => r#####"Kuva kõik teemanimed"#####,
    b"mod_prefix" => r#####"mod faili eesliide (vaikimisi "l10n_")"#####,
    b"outdir" => r#####"Väljundkaust"#####,
    b"output_bincode" => r#####"Loo eraldi bincode failid erinevatele keeltele"#####,
    b"output_bincode_all_in_one" => {
      r#####"Ekspordi kõikide keelte bincode ühte faili"#####
    }
    b"output_locales_fn" => r#####"Ekspordi all_locales funktsioon"#####,
    b"output_match_fn" => {
      r#####"Loo Rusti faile koos match avaldistega keelte kaupa"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolideeri kõik andmed ühte match funktsiooni (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funktsioon keele nimega võtmena"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funktsioon kombineeritud võtmega (keel + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Kasuta ainult map_key võtmena (välja arvatud map_name)"#####
    }
    b"output_phf" => r#####"Loo keelekaupa eraldi phf kaardi funktsioonid"#####,
    b"output_phf_all_in_one" => {
      r#####"Konsolideeri kõik phf kaardid ühte funktsiooni"#####
    }
    b"output_phf_by_key" => {
      r#####"phf kaart tavaliste stringi võtmetega (mitte TupleKey)"#####
    }
    b"output_ron" => r#####"Ekspordi string RON-vormingus"#####,
    b"suffix" => r#####"Uue "Highlight-Map" laiend"#####,
    b"syntax_name" => r#####"Süntaksi nimi"#####,
    b"theme_name" => r#####"Teema nimi"#####,
    b"true_color" => r#####"24-bitine tõeline värv"#####,
    b"visibility" => r#####"Loodud koodi nähtavus"#####,
    _ => "",
  }
}
