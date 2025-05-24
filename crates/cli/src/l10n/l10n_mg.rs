pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Hamerina ny background"#####,
    b"base_name" => r#####"Anarana fototra "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffixe fichier bincode"#####,
    b"custom_syntax_set" => r#####"Fichier syntaxe personnalisé"#####,
    b"custom_theme_set" => r#####"Fichier lohahevitra personnalisé"#####,
    b"display_config_dir" => r#####"Asehoy ny dossier config glossa"#####,
    b"dsl_suffix" => r#####"Suffixe fichier DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode lisitra mainty: Aza manomboka ny ID fiteny ao @lisitra"#####
    }
    b"exclude_map_names" => {
      r#####"Aza manomboka ny anaran'ny sarintany ao @lisitra"#####
    }
    b"include_languages" => {
      r#####"Mode lisitra fotsiny: Manomboka ny ID fiteny ao @lisitra"#####
    }
    b"include_map_names" => {
      r#####"Manomboka ny anaran'ny sarintany ao @lisitra"#####
    }
    b"input" => r#####"Dossier loharanon'ny ressources localisés"#####,
    b"list_all_syntaxes" => {
      r#####"Asehoy ny anarana syntaxe sy extension rehetra"#####
    }
    b"list_all_themes" => r#####"Asehoy ny anarana lohahevitra rehetra"#####,
    b"mod_prefix" => r#####"Prefixe fichier mod (default "l10n_")"#####,
    b"outdir" => r#####"Dossier famoahana"#####,
    b"output_bincode" => {
      r#####"Mamorona fichier bincode samihafa ho an'ny fiteny"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Mamoaka bincode rehetra ao anaty fichier iray"#####
    }
    b"output_locales_fn" => r#####"Mamoaka fonction all_locales"#####,
    b"output_match_fn" => {
      r#####"Mamorona fichier Rust misy fonction match ho an'ny fiteny"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Mamorona fonction match iray ho an'ny angon-drakitra (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonction match miaraka amin'ny anaran'ny fiteny ho solokely"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Fonction match miaraka amin'ny solokely tambatra (fiteny + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Mampiasa map_key ho solokely (tsy misy map_name)"#####
    }
    b"output_phf" => r#####"Mamorona fonction phf map samihafa ho an'ny fiteny"#####,
    b"output_phf_all_in_one" => {
      r#####"Mamorona fonction iray ho an'ny phf map rehetra"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map miaraka amin'ny string solokely (fa tsy TupleKey)"#####
    }
    b"output_ron" => r#####"Mamoaka string amin'ny format RON"#####,
    b"suffix" => r#####"Suffixe vaovao "Highlight-Map""#####,
    b"syntax_name" => r#####"Anarana syntaxe"#####,
    b"theme_name" => r#####"Anarana lohahevitra"#####,
    b"true_color" => r#####"Loko marina 24-bit"#####,
    b"visibility" => r#####"Fahitana ny kaody novokarina"#####,
    _ => "",
  }
}
