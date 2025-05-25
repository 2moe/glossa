pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Povolit pozadí"#####,
    b"base_name" => r#####"Základní název "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Přípona souboru bincode"#####,
    b"custom_syntax_set" => r#####"Vlastní soubor syntaxových sad"#####,
    b"custom_theme_set" => r#####"Vlastní soubor sad motivů"#####,
    b"display_config_dir" => r#####"Zobrazit konfigurační adresář Glossa"#####,
    b"dsl_suffix" => r#####"Přípona DSL souboru (výchozí ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Režim černé listiny: Neinicializovat ID jazyků ze seznamu"#####
    }
    b"exclude_map_names" => r#####"Neinicializovat názvy map ze seznamu"#####,
    b"include_languages" => {
      r#####"Režim bílé listiny: Inicializovat pouze ID jazyků ze seznamu"#####
    }
    b"include_map_names" => r#####"Inicializovat pouze názvy map ze seznamu"#####,
    b"input" => r#####"Zdrojový adresář lokalizačních prostředků"#####,
    b"list_all_syntaxes" => r#####"Zobrazit všechny názvy syntaxí a rozšíření"#####,
    b"list_all_themes" => r#####"Zobrazit všechny názvy motivů"#####,
    b"mod_prefix" => r#####"Předpona mod souboru (výchozí "l10n_")"#####,
    b"outdir" => r#####"Výstupní adresář"#####,
    b"output_bincode" => {
      r#####"Generovat samostatné soubory bincode pro různé jazyky"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportovat všechny bincode do jednoho souboru"#####
    }
    b"output_locales_fn" => r#####"Exportovat funkci all_locales"#####,
    b"output_match_fn" => {
      r#####"Generovat Rust soubory s match funkcemi pro jazyky"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Exportovat všechna data do jedné match funkce (řetězec)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkce s názvem jazyka jako klíčem"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match funkce s kombinovaným klíčem (jazyk+map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Použít pouze map_key jako klíč (bez map_name)"#####
    }
    b"output_phf" => r#####"Generovat samostatné phf map funkce pro jazyky"#####,
    b"output_phf_all_in_one" => {
      r#####"Sloučit všechny phf mapy do jedné funkce"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map s obyčejnými řetězcovými klíči (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Exportovat řetězec ve formátu RON"#####,
    b"suffix" => r#####"Nová přípona pro "Highlight-Map""#####,
    b"syntax_name" => r#####"Název syntaxe"#####,
    b"theme_name" => r#####"Název motivu"#####,
    b"true_color" => r#####"24bitová skutečná barva"#####,
    b"visibility" => r#####"Viditelnost generovaného kódu"#####,
    _ => "",
  }
}
