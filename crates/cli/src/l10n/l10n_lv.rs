pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Vai aktivizēt fona"#####,
    b"base_name" => r#####""Highlight-Map" pamatnosaukums"#####,
    b"bincode_suffix" => r#####"bincode failu paplašinājums"#####,
    b"custom_syntax_set" => r#####"Pielāgota sintakses kopas fails"#####,
    b"custom_theme_set" => r#####"Pielāgota tēmu kopas fails"#####,
    b"display_config_dir" => r#####"Rādīt Glossa konfigurācijas direktoriju"#####,
    b"dsl_suffix" => r#####"DSL faila paplašinājums (noklusējums ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Melnā saraksta režīms: Neinicializēt sarakstā esošos valodu ID"#####
    }
    b"exclude_map_names" => {
      r#####"Neinicializēt sarakstā esošos karšu nosaukumus"#####
    }
    b"include_languages" => {
      r#####"Baltā saraksta režīms: Inicializēt tikai sarakstā esošos valodu ID"#####
    }
    b"include_map_names" => {
      r#####"Inicializēt tikai sarakstā esošos karšu nosaukumus"#####
    }
    b"input" => r#####"Lokalizācijas resursu avota direktorija"#####,
    b"list_all_syntaxes" => {
      r#####"Rādīt visus sintakses nosaukumus un paplašinājumus"#####
    }
    b"list_all_themes" => r#####"Rādīt visas tēmu nosaukumus"#####,
    b"mod_prefix" => r#####"mod faila prefikss (noklusējums "l10n_")"#####,
    b"outdir" => r#####"Izvades direktorija"#####,
    b"output_bincode" => {
      r#####"Ģenerēt atsevišķus bincode failus dažādām valodām"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksportēt visu valodu bincode vienā failā"#####
    }
    b"output_locales_fn" => r#####"Eksportēt all_locales funkciju"#####,
    b"output_match_fn" => {
      r#####"Ģenerēt Rust koda failus ar match izteiksmēm (funkcijas)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Eksportēt visus datus vienā match funkcijā (virkne)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija ar valodas nosaukumu kā atslēgu"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match funkcija ar kombinētu atslēgu (valoda + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Lietot tikai map_key kā atslēgu (bez map_name)"#####
    }
    b"output_phf" => r#####"Ģenerēt atsevišķas phf kartes funkcijas valodām"#####,
    b"output_phf_all_in_one" => {
      r#####"Apvienot visas phf kartes vienā funkcijā"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf karte ar parastām virknes atslēgām (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Eksportēt virkni RON formātā"#####,
    b"suffix" => r#####""Highlight-Map" jaunais paplašinājums"#####,
    b"syntax_name" => r#####"Sintakses nosaukums"#####,
    b"theme_name" => r#####"Tēmas nosaukums"#####,
    b"true_color" => r#####"24 bitu īstā krāsa"#####,
    b"visibility" => r#####"Ģenerētā koda redzamība"#####,
    _ => "",
  }
}
