pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Whakahohe Papamuri"#####,
    b"base_name" => r#####"Ingoa Taketake mō "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Pīmuri Kōnae Bincode"#####,
    b"custom_syntax_set" => r#####"Kōnae Wetereo Whaiaronga"#####,
    b"custom_theme_set" => r#####"Kōnae Kaupapa Whaiaronga"#####,
    b"display_config_dir" => r#####"Whakaatu Kōpaki Whirihora mō Glossa"#####,
    b"dsl_suffix" => r#####"Pīmuri Kōnae DSL (Taunoa ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Aratau Rārangi Pango: Kaua e Whakahohe ID Reo"#####
    }
    b"exclude_map_names" => r#####"Kaua e Whakahohe Ingoa Map kei Rārangi"#####,
    b"include_languages" => {
      r#####"Aratau Rārangi Mā: Whakahohe ID Reo kei Rārangi"#####
    }
    b"include_map_names" => r#####"Whakahohe Ingoa Map kei Rārangi"#####,
    b"input" => r#####"Kōpaki Pūtake mō Ngā Rauemi Reo"#####,
    b"list_all_syntaxes" => {
      r#####"Whakaatu Ngā Ingoa Wetereo me Ngā Whakawhānui"#####
    }
    b"list_all_themes" => r#####"Whakaatu Ngā Ingoa Kaupapa Katoa"#####,
    b"mod_prefix" => r#####"Kōmuri Kōnae Mod (Taunoa "l10n_")"#####,
    b"outdir" => r#####"Kōpaki Putanga"#####,
    b"output_bincode" => r#####"Hanga Kōnae Bincode Wehe mō Ngā Reo"#####,
    b"output_bincode_all_in_one" => {
      r#####"Tuku Ngā Bincode Katoa ki Kōnae Kotahi"#####
    }
    b"output_locales_fn" => r#####"Tuku Mahi All_Locales"#####,
    b"output_match_fn" => r#####"Hanga Kōnae Rust me Ngā Mahi Match mō Reo"#####,
    b"output_match_fn_all_in_one" => {
      r#####"Hōputu Katoa ki Mahi Match Kotahi (Tūtohi)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Mahi Match me Ingoa Reo hei Kī"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Mahi Match me Kī Honohono (Reo + Map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Whakamahi Map_key Anake hei Kī (Kāore ko Map_name)"#####
    }
    b"output_phf" => r#####"Hanga Mahi PHF Map mō Ngā Reo"#####,
    b"output_phf_all_in_one" => r#####"Hōputu Katoa ki Mahi PHF Kotahi"#####,
    b"output_phf_by_key" => {
      r#####"PHF Map me Kī Tūtohi Noa (Ehara i te TupleKey)"#####
    }
    b"output_ron" => r#####"Tuku Tūtohi ki Hōputu RON"#####,
    b"suffix" => r#####"Pīmuri Hou mō "Highlight-Map""#####,
    b"syntax_name" => r#####"Ingoa Wetereo"#####,
    b"theme_name" => r#####"Ingoa Kaupapa"#####,
    b"true_color" => r#####"Tae Māramatanga 24-bit"#####,
    b"visibility" => r#####"Te Kitea o te Waehere Hanga"#####,
    _ => "",
  }
}
