pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Pèmèt background"#####,
    b"base_name" => r#####"Non Baz "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sifiks Dosye Bincode"#####,
    b"custom_syntax_set" => r#####"Dosye Sentaks Personnalize"#####,
    b"custom_theme_set" => r#####"Dosye Tèm Personnalize"#####,
    b"display_config_dir" => r#####"Afiche Dosye Konfigirasyon Glossa"#####,
    b"dsl_suffix" => r#####"Sifiks Dosye DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mòd Lis Nwa: Pa inisyalize ID lang nan lis la"#####
    }
    b"exclude_map_names" => r#####"Pa inisyalize non kat nan lis la"#####,
    b"include_languages" => {
      r#####"Mòd Lis Blan: Inisyalize sèlman ID lang nan lis la"#####
    }
    b"include_map_names" => r#####"Inisyalize sèlman non kat nan lis la"#####,
    b"input" => r#####"Dosye Sous Resous Lokalizasyon"#####,
    b"list_all_syntaxes" => r#####"Afiche tout non sentaks ak ekstansyon"#####,
    b"list_all_themes" => r#####"Afiche tout non tèm"#####,
    b"mod_prefix" => r#####"Prefiks Dosye Mod (default "l10n_")"#####,
    b"outdir" => r#####"Dosye Sòti"#####,
    b"output_bincode" => r#####"Jenere dosye bincode endepandan pou chak lang"#####,
    b"output_bincode_all_in_one" => {
      r#####"Ekspòte tout bincode nan yon sèl dosye"#####
    }
    b"output_locales_fn" => r#####"Ekspòte fonksyon all_locales"#####,
    b"output_match_fn" => {
      r#####"Jenere dosye Rust ak fonksyon match pou chak lang"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolide tout done nan yon sèl fonksyon match (chenn)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fonksyon match ak non lang kòm kle"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fonksyon match ak kle konpoze (non lang + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Sèlman itilize map_key kòm kle (san map_name)"#####
    }
    b"output_phf" => r#####"Jenere fonksyon phf map pou chak lang"#####,
    b"output_phf_all_in_one" => {
      r#####"Konsolide tout phf map nan yon sèl fonksyon"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map ak kle chenn òdinè (pa TupleKey)"#####
    }
    b"output_ron" => r#####"Ekspòte chenn nan fòma RON"#####,
    b"suffix" => r#####"Sifiks Nouvo "Highlight-Map""#####,
    b"syntax_name" => r#####"Non Sentaks"#####,
    b"theme_name" => r#####"Non Tèm"#####,
    b"true_color" => r#####"Koulè Vre 24-bit"#####,
    b"visibility" => r#####"Vizibilite Kòd Jenere"#####,
    _ => "",
  }
}
