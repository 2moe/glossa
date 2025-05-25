pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ota tausta käyttöön"#####,
    b"base_name" => r#####"Highlight-Mapin perusnimi"#####,
    b"bincode_suffix" => r#####"bincode-tiedoston pääte"#####,
    b"custom_syntax_set" => r#####"Mukautettu syntaksikokoelmantiedosto"#####,
    b"custom_theme_set" => r#####"Mukautettu teemakokoelmantiedosto"#####,
    b"display_config_dir" => r#####"Näytä glossan asetushakemisto"#####,
    b"dsl_suffix" => r#####"DSL-tiedoston pääte (oletus ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Musta lista: Älä alusta listalla olevia kielitunnisteita"#####
    }
    b"exclude_map_names" => r#####"Älä alusta listalla olevia karttanimiä"#####,
    b"include_languages" => {
      r#####"Valkoinen lista: Alusta vain listalla olevat kielitunnisteet"#####
    }
    b"include_map_names" => r#####"Alusta vain listalla olevat karttanimet"#####,
    b"input" => r#####"Lokalisointiresurssien lähdehakemisto"#####,
    b"list_all_syntaxes" => r#####"Näytä kaikki syntaksien nimet ja päätteet"#####,
    b"list_all_themes" => r#####"Näytä kaikki teemojen nimet"#####,
    b"mod_prefix" => r#####"mod-tiedoston etuliite (oletus "l10n_")"#####,
    b"outdir" => r#####"Tulostushakemisto"#####,
    b"output_bincode" => r#####"Luo erilliset bincode-tiedostot eri kielille"#####,
    b"output_bincode_all_in_one" => {
      r#####"Vie kaikki kielten bincodet yhteen tiedostoon"#####
    }
    b"output_locales_fn" => r#####"Vie all_locales-funktio"#####,
    b"output_match_fn" => {
      r#####"Luo erilliset Rust-kooditiedostot (sisältää match-lausekkeet) kielille"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Yhdistä kaikki tiedot yhteen match-funktioon (merkkijono)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funktio kielten nimillä avaimina"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match-funktio yhdistetyillä avaimilla (kieli + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Käytä vain map_key:ta avaimena (ei map_name)"#####
    }
    b"output_phf" => r#####"Luo erilliset phf-karttafunktiot kielille"#####,
    b"output_phf_all_in_one" => {
      r#####"Yhdistä kaikki phf-kartat yhteen funktioon"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf-kartta tavallisilla merkkijonoavaimilla (ei TupleKey)"#####
    }
    b"output_ron" => r#####"Vie merkkijono RON-muodossa"#####,
    b"suffix" => r#####"Uuden Highlight-Mapin pääte"#####,
    b"syntax_name" => r#####"Syntaksin nimi"#####,
    b"theme_name" => r#####"Teeman nimi"#####,
    b"true_color" => r#####"24-bittinen todellinen väri"#####,
    b"visibility" => r#####"Luodun koodin näkyvyys"#####,
    _ => "",
  }
}
