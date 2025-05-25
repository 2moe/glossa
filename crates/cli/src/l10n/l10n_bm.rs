pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kɔnɔnin ka ɲɛnabɔ"#####,
    b"base_name" => r#####""Highlight-Map" dugukolo tɔgɔ"#####,
    b"bincode_suffix" => r#####"Bincode firi tɔw ka suffixe"#####,
    b"custom_syntax_set" => r#####"Yɛrɛsɛn sɛntaksɛ firi"#####,
    b"custom_theme_set" => r#####"Yɛrɛsɛn tɛmɛ firi"#####,
    b"display_config_dir" => r#####"Glossa ka sigida firi jira"#####,
    b"dsl_suffix" => r#####"DSL firi suffixe (default ".dsl")"#####,
    b"exclude_languages" => r#####"Nɔgɔ list: Kan ID list kɔnɔ dɔw tɛ sɔrɔ"#####,
    b"exclude_map_names" => r#####"Mɛpi tɔw list kɔnɔ dɔw tɛ sɔrɔ"#####,
    b"include_languages" => r#####"Cɛsiri list: Kan ID list kɔnɔ dɔw sɔrɔ"#####,
    b"include_map_names" => r#####"Mɛpi tɔw list kɔnɔ dɔw sɔrɔ"#####,
    b"input" => r#####"Lokalizasiyɔn dafɛlɛw ka firi"#####,
    b"list_all_syntaxes" => r#####"Sɛntaksɛ tɔw bɛɛ jira"#####,
    b"list_all_themes" => r#####"Tɛmɛ tɔw bɛɛ jira"#####,
    b"mod_prefix" => r#####"Mod firi tɔw ka prefixe (default "l10n_")"#####,
    b"outdir" => r#####"Baarakɛlaw ka ɲɛnabɔ"#####,
    b"output_bincode" => r#####"Bincode firiw labɛn don dɔɔnin dɔɔnin kan"#####,
    b"output_bincode_all_in_one" => r#####"Bincode bɛɛ taa firi kelen na"#####,
    b"output_locales_fn" => r#####"All_locales fonksiyɔ bɔ"#####,
    b"output_match_fn" => {
      r#####"Rust kɔdɛ firiw labɛn kan kɔnɔ match sɛbɛnw ye"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Dɔnni bɛɛ taa match fonksiyɔ kelen na (kuru)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match fonksiyɔ kan dɔrɔn dɔ ka kan tɔgɔ ye"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match fonksiyɔ dɔrɔn dɔ (kan tɔgɔ + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Map_key dɔrɔn de bɛ baara kɛ (map_name tɛ kɛ)"#####
    }
    b"output_phf" => r#####"PHF mɛpifiriw labɛn don kan kɔnɔ"#####,
    b"output_phf_all_in_one" => r#####"PHF mɛpi bɛɛ taa fonksiyɔ kelen na"#####,
    b"output_phf_without_map_name" => {
      r#####"PHF mɛpi kuru dɔrɔnw (TupleKey tɛ)"#####
    }
    b"output_ron" => r#####"RON format kɔnɔ kuru bɔ"#####,
    b"suffix" => r#####"Kura "Highlight-Map" ka suffixe"#####,
    b"syntax_name" => r#####"Sɛntaksɛ tɔgɔ"#####,
    b"theme_name" => r#####"Tɛmɛ tɔgɔ"#####,
    b"true_color" => r#####"24-bit nɔgɔlen duman"#####,
    b"visibility" => r#####"Kɔdɛ min sɔrɔ bɛ se ka yɛlɛma"#####,
    _ => "",
  }
}
