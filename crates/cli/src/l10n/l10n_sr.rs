pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktiviraj pozadinu"#####,
    b"base_name" => r#####"Osnovno ime "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Nastavak bincode datoteke"#####,
    b"custom_syntax_set" => r#####"Prilagođeni sintaksni skup datoteka"#####,
    b"custom_theme_set" => r#####"Prilagođeni tematski skup datoteka"#####,
    b"display_config_dir" => r#####"Prikaži konfiguracioni direktorijum glossa"#####,
    b"dsl_suffix" => r#####"Nastavak DSL datoteke (podrazumevano ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Crna lista: Ne inicijalizuj ID-jeve jezika sa liste"#####
    }
    b"exclude_map_names" => r#####"Ne inicijalizuj map_names sa liste"#####,
    b"include_languages" => {
      r#####"Bela lista: Inicijalizuj samo ID-jeve jezika sa liste"#####
    }
    b"include_map_names" => r#####"Inicijalizuj samo map_names sa liste"#####,
    b"input" => r#####"Izvorni direktorijum lokalizacionih resursa"#####,
    b"list_all_syntaxes" => r#####"Prikaži sve nazive sintaksi i ekstenzije"#####,
    b"list_all_themes" => r#####"Prikaži sve nazive tema"#####,
    b"mod_prefix" => r#####"Prefiks mod datoteke (podrazumevano "l10n_")"#####,
    b"outdir" => r#####"Izlazni direktorijum"#####,
    b"output_bincode" => {
      r#####"Generisanje nezavisnih bincode datoteka za različite jezike"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Izvezi sve bincode jezika u jednu datoteku"#####
    }
    b"output_locales_fn" => r#####"Izvezi all_locales funkciju"#####,
    b"output_match_fn" => {
      r#####"Generisanje Rust datoteka sa match funkcijama za jezike"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsoliduj sve podatke u jednu match funkciju (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija sa imenom jezika kao ključem"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match funkcija sa kombinovanim ključem (jezik + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Koristi samo map_key kao ključ (bez map_name)"#####
    }
    b"output_phf" => r#####"Generisanje phf mapa za različite jezike"#####,
    b"output_phf_all_in_one" => {
      r#####"Konsoliduj sve phf mape u jednu funkciju"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map sa običnim string ključevima (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Izvezi string u RON formatu"#####,
    b"suffix" => r#####"Nastavak za novi "Highlight-Map""#####,
    b"syntax_name" => r#####"Naziv sintakse"#####,
    b"theme_name" => r#####"Naziv teme"#####,
    b"true_color" => r#####"24-bitna autentična boja"#####,
    b"visibility" => r#####"Vidljivost generisanog koda"#####,
    _ => "",
  }
}
