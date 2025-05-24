pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktiviraj pozadinu"#####,
    b"base_name" => r#####"Osnovno ime "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Nastavak bincode datoteke"#####,
    b"custom_syntax_set" => r#####"Prilagođeni skup sintakse"#####,
    b"custom_theme_set" => r#####"Prilagođeni skup tema"#####,
    b"display_config_dir" => r#####"Prikaži konfiguracijski direktorij Glossa"#####,
    b"dsl_suffix" => r#####"Nastavak DSL datoteke (zadano ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Crni popis: Nemoj inicijalizirati ID-jeve jezika s popisa"#####
    }
    b"exclude_map_names" => r#####"Nemoj inicijalizirati imena mapa s popisa"#####,
    b"include_languages" => {
      r#####"Bijeli popis: Inicijaliziraj samo ID-jeve jezika s popisa"#####
    }
    b"include_map_names" => r#####"Inicijaliziraj samo imena mapa s popisa"#####,
    b"input" => r#####"Izvorni direktorij lokalizacijskih resursa"#####,
    b"list_all_syntaxes" => r#####"Prikaži sve nazive sintaksi i proširenja"#####,
    b"list_all_themes" => r#####"Prikaži sve nazive tema"#####,
    b"mod_prefix" => r#####"Prefiks mod datoteke (zadano "l10n_")"#####,
    b"outdir" => r#####"Izlazni direktorij"#####,
    b"output_bincode" => {
      r#####"Generiraj zasebne bincode datoteke za različite jezike"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Izvezi sve bincode jezika u jednu datoteku"#####
    }
    b"output_locales_fn" => r#####"Izvezi all_locales funkciju"#####,
    b"output_match_fn" => {
      r#####"Generiraj Rust datoteke s match funkcijama za jezike"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Izvezi sve podatke u jednu match funkciju (niz)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija s imenom jezika kao ključem"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funkcija s kombiniranim ključem (jezik + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Koristi samo map_key kao ključ (bez map_name)"#####
    }
    b"output_phf" => r#####"Generiraj zasebne phf map funkcije za jezike"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombiniraj sve phf mape u jednu funkciju"#####
    }
    b"output_phf_by_key" => {
      r#####"phf mapa s običnim nizovnim ključevima (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Izvezi niz u RON formatu"#####,
    b"suffix" => r#####"Novi nastavak za "Highlight-Map""#####,
    b"syntax_name" => r#####"Naziv sintakse"#####,
    b"theme_name" => r#####"Naziv teme"#####,
    b"true_color" => r#####"24-bitna prava boja"#####,
    b"visibility" => r#####"Vidljivost generiranog koda"#####,
    _ => "",
  }
}
