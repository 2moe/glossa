pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Gbaa ọsọ ndabere"#####,
    b"base_name" => r#####"Aha isi nke "Map Na-egbuke egbuke""#####,
    b"bincode_suffix" => r#####"Ụda ngwụcha faịlụ bincode"#####,
    b"custom_syntax_set" => r#####"Faịlụ nhazi nke ahaziri"#####,
    b"custom_theme_set" => r#####"Faịlụ isiokwu nke ahaziri"#####,
    b"display_config_dir" => r#####"Gosi nchekwa nhazi Glossa"#####,
    b"dsl_suffix" => r#####"Ụda ngwụcha faịlụ DSL (ndabere ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Ụdị ndepụta ojii: Ejila ID asụsụ dị na ndepụta malite"#####
    }
    b"exclude_map_names" => r#####"Ejila aha map dị na ndepụta malite"#####,
    b"include_languages" => {
      r#####"Ụdị ndepụta ọcha: Malite naanị ID asụsụ dị na ndepụta"#####
    }
    b"include_map_names" => r#####"Malite naanị aha map dị na ndepụta"#####,
    b"input" => r#####"Nchekwa isi mmalite nke akụrụngwa obodo"#####,
    b"list_all_syntaxes" => r#####"Gosi aha nhazi niile na mgbatị ha"#####,
    b"list_all_themes" => r#####"Gosi aha isiokwu niile"#####,
    b"mod_prefix" => r#####"Mmalite faịlụ mod (ndabere "l10n_")"#####,
    b"outdir" => r#####"Nchekwa mmepụta"#####,
    b"output_bincode" => {
      r#####"Mepụta faịlụ bincode nọọrọ onwe ya maka asụsụ dị iche iche"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Wepụta bincode nke asụsụ niile n'otu faịlụ"#####
    }
    b"output_locales_fn" => r#####"Wepụta ọrụ all_locales"#####,
    b"output_match_fn" => {
      r#####"Mepụta faịlụ Rust nwere ọrụ match maka asụsụ dị iche iche"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Wepụta data niile n'otu ọrụ match (eriri)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Ọrụ match jiri aha asụsụ dị ka isi"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Ọrụ match jiri isi jikọtara (aha asụsụ + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Jiri naanị map_key dị ka isi (enweghị map_name)"#####
    }
    b"output_phf" => r#####"Mepụta ọrụ phf map dị iche iche maka asụsụ"#####,
    b"output_phf_all_in_one" => r#####"Jikọta phf map niile n'otu ọrụ"#####,
    b"output_phf_by_key" => {
      r#####"phf map nwere isi eriri nkịtị (abụghị TupleKey)"#####
    }
    b"output_ron" => r#####"Wepụta eriri n'ụdị RON"#####,
    b"suffix" => r#####"Ụda ngwụcha ọhụrụ nke "Map Na-egbuke egbuke""#####,
    b"syntax_name" => r#####"Aha nhazi"#####,
    b"theme_name" => r#####"Aha isiokwu"#####,
    b"true_color" => r#####"Agba ezi 24-bit"#####,
    b"visibility" => r#####"Nhụta koodu emepụtara"#####,
    _ => "",
  }
}
