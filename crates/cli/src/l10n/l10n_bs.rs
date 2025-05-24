pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktiviraj pozadinu"#####,
    b"base_name" => r#####"Osnovni naziv "Highlight-Map""#####,
    b"bincode_suffix" => r#####"sufiks bincode datoteke"#####,
    b"custom_syntax_set" => r#####"Prilagođeni set sintakse"#####,
    b"custom_theme_set" => r#####"Prilagođeni set tema"#####,
    b"display_config_dir" => r#####"Prikaži konfiguracioni direktorij Glossa"#####,
    b"dsl_suffix" => r#####"sufiks DSL datoteke (podrazumijevano ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Crna lista: Ne inicijalizuj ID-jezike iz liste"#####
    }
    b"exclude_map_names" => r#####"Ne inicijalizuj imena mapa iz liste"#####,
    b"include_languages" => {
      r#####"Bijela lista: Inicijalizuj samo ID-jezike iz liste"#####
    }
    b"include_map_names" => r#####"Inicijalizuj samo imena mapa iz liste"#####,
    b"input" => r#####"Izvorni direktorij lokalizacionih resursa"#####,
    b"list_all_syntaxes" => r#####"Prikaži sve nazive sintaksi i ekstenzija"#####,
    b"list_all_themes" => r#####"Prikaži sve nazive tema"#####,
    b"mod_prefix" => r#####"prefiks mod datoteke (podrazumijevano "l10n_")"#####,
    b"outdir" => r#####"Izlazni direktorij"#####,
    b"output_bincode" => {
      r#####"Generiši odvojene bincode datoteke za različite jezike"#####
    }
    b"output_bincode_all_in_one" => r#####"Izvezi sve bincode u jednu datoteku"#####,
    b"output_locales_fn" => r#####"Izvezi all_locales funkciju"#####,
    b"output_match_fn" => {
      r#####"Generiši Rust datoteke sa match funkcijama za jezike"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Izvezi sve podatke u jednu match funkciju (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija sa imenom jezika kao ključem"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funkcija sa kombinovanim ključem (jezik + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Koristi samo map_key kao ključ (bez map_name)"#####
    }
    b"output_phf" => r#####"Generiši odvojene phf map funkcije za jezike"#####,
    b"output_phf_all_in_one" => r#####"Spoji sve phf mape u jednu funkciju"#####,
    b"output_phf_by_key" => {
      r#####"phf map sa običnim string ključevima (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Izvezi string u RON formatu"#####,
    b"suffix" => r#####"Novi sufiks za "Highlight-Map""#####,
    b"syntax_name" => r#####"Naziv sintakse"#####,
    b"theme_name" => r#####"Naziv teme"#####,
    b"true_color" => r#####"24-bitna prava boja"#####,
    b"visibility" => r#####"Vidljivost generisanog koda"#####,
    _ => "",
  }
}
