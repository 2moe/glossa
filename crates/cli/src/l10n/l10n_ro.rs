pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Activează fundal"#####,
    b"base_name" => r#####"Numele de bază al "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufix fișier bincode"#####,
    b"custom_syntax_set" => r#####"Fișier set sintaxă personalizat"#####,
    b"custom_theme_set" => r#####"Fișier set teme personalizate"#####,
    b"display_config_dir" => r#####"Afișează directorul de config glossa"#####,
    b"dsl_suffix" => r#####"Sufix fișier DSL (implicit ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mod listă neagră: Nu inițializa ID-uri din listă"#####
    }
    b"exclude_map_names" => r#####"Nu inițializa nume de mapă din listă"#####,
    b"include_languages" => {
      r#####"Mod listă albă: Inițializează doar ID-uri din listă"#####
    }
    b"include_map_names" => r#####"Inițializează doar nume de mapă din listă"#####,
    b"input" => r#####"Director sursă pentru resurse localizate"#####,
    b"list_all_syntaxes" => {
      r#####"Afișează toate numele de sintaxă și extensiile"#####
    }
    b"list_all_themes" => r#####"Afișează toate numele de teme"#####,
    b"mod_prefix" => r#####"Prefix fișier mod (implicit "l10n_")"#####,
    b"outdir" => r#####"Director ieșire"#####,
    b"output_bincode" => {
      r#####"Generează fișiere bincode separate pentru limbi diferite"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportă toate bincode-urile într-un singur fișier"#####
    }
    b"output_locales_fn" => r#####"Exportă funcția all_locales"#####,
    b"output_match_fn" => {
      r#####"Generează fișiere Rust cu funcții match pentru fiecare limbă"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolidează toate datele într-o funcție match (șir)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funcție match cu numele limbii ca cheie"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Funcție match cu cheie compusă (limbă + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Folosește doar map_key ca cheie (fără map_name)"#####
    }
    b"output_phf" => r#####"Generează funcții phf map pentru limbi diferite"#####,
    b"output_phf_all_in_one" => {
      r#####"Consolidează toate phf map-urile într-o funcție"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map cu chei șir obișnuite (nu TupleKey)"#####
    }
    b"output_ron" => r#####"Exportă șir în format RON"#####,
    b"suffix" => r#####"Sufixul noului "Highlight-Map""#####,
    b"syntax_name" => r#####"Nume sintaxă"#####,
    b"theme_name" => r#####"Nume temă"#####,
    b"true_color" => r#####"Culoare adevărată 24-biți"#####,
    b"visibility" => r#####"Vizibilitate cod generat"#####,
    _ => "",
  }
}
