pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Activer l'arrière-plan"#####,
    b"base_name" => r#####"Nom de base "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Suffixe de fichier bincode"#####,
    b"custom_syntax_set" => r#####"Fichier de syntaxe personnalisé"#####,
    b"custom_theme_set" => r#####"Fichier de thèmes personnalisés"#####,
    b"display_config_dir" => {
      r#####"Afficher le répertoire de configuration de glossa"#####
    }
    b"dsl_suffix" => r#####"Suffixe de fichier DSL (par défaut ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode liste noire : Ne pas initialiser les ID de langue listés"#####
    }
    b"exclude_map_names" => {
      r#####"Ne pas initialiser les noms de cartes listés"#####
    }
    b"include_languages" => {
      r#####"Mode liste blanche : Initialiser uniquement les ID de langue listés"#####
    }
    b"include_map_names" => {
      r#####"Initialiser uniquement les noms de cartes listés"#####
    }
    b"input" => r#####"Répertoire source des ressources de localisation"#####,
    b"list_all_syntaxes" => {
      r#####"Afficher tous les noms de syntaxe et extensions"#####
    }
    b"list_all_themes" => r#####"Afficher tous les noms de thèmes"#####,
    b"mod_prefix" => r#####"Préfixe de fichier mod (par défaut "l10n_")"#####,
    b"outdir" => r#####"Répertoire de sortie"#####,
    b"output_bincode" => {
      r#####"Générer des fichiers bincode indépendants par langue"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exporter tous les bincode dans un seul fichier"#####
    }
    b"output_locales_fn" => r#####"Exporter la fonction all_locales"#####,
    b"output_match_fn" => {
      r#####"Générer des fichiers Rust avec fonctions match par langue"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Consolider toutes les données dans une fonction match (chaîne)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonction match avec nom de langue comme clé"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fonction match avec clé composite (langue + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Utilise uniquement map_key comme clé (exclut map_name)"#####
    }
    b"output_phf" => r#####"Générer des fonctions phf map par langue"#####,
    b"output_phf_all_in_one" => {
      r#####"Fusionner toutes les phf map dans une fonction"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map avec clés chaînes simples (pas TupleKey)"#####
    }
    b"output_ron" => r#####"Exporter en chaîne format RON"#####,
    b"suffix" => r#####"Suffixe du nouveau "Highlight-Map""#####,
    b"syntax_name" => r#####"Nom de syntaxe"#####,
    b"theme_name" => r#####"Nom du thème"#####,
    b"true_color" => r#####"Couleur vraie 24 bits"#####,
    b"visibility" => r#####"Visibilité du code généré"#####,
    _ => "",
  }
}
