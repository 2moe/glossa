pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Yambitsani chithunzi chakumbuyo"#####,
    b"base_name" => r#####"Dzina loyambira la "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Tsatoli la fayilo ya bincode"#####,
    b"custom_syntax_set" => r#####"Fayilo yokhazikika ya syntax"#####,
    b"custom_theme_set" => r#####"Fayilo yokhazikika ya mitheme"#####,
    b"display_config_dir" => r#####"Onetsani diresi ya kasamalidwe ka glossa"#####,
    b"dsl_suffix" => r#####"Tsatoli la fayilo ya DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Njira ya mndanda wakuda: Musayambitse ID za zilankhulo"#####
    }
    b"exclude_map_names" => r#####"Musayambitse map_names"#####,
    b"include_languages" => {
      r#####"Njira ya mndanda woyera: Yambitsani ID za zilankhulo zokha"#####
    }
    b"include_map_names" => r#####"Yambitsani map_names zokha"#####,
    b"input" => r#####"Diresi yoyambira ya zida za localization"#####,
    b"list_all_syntaxes" => r#####"Onetsani mainsyntax onse ndi zowonjezera"#####,
    b"list_all_themes" => r#####"Onetsani mathemu onse"#####,
    b"mod_prefix" => r#####"Chiyambi cha fayilo ya mod (default "l10n_")"#####,
    b"outdir" => r#####"Diresi Yokhazikika"#####,
    b"output_bincode" => {
      r#####"Pangani mafayilo a bincode okhala ndi mitundu ya zilankhulo"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Tulutsani ma bincode onse mu fayilo limodzi"#####
    }
    b"output_locales_fn" => r#####"Tulutsani ntchito ya all_locales"#####,
    b"output_match_fn" => {
      r#####"Pangani mafayilo a Rust ndi ntchito za match pazilankhulo"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Gwirizanitsani zonse mu ntchito imodzi ya match (mawu)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Ntchito ya match yokhala ndi dzina la chiyankhulo ngati khodi"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Ntchito ya match ndi khodi yophatikiza (dzina + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Gwiritsani map_key kokha ngati khodi (osaphimba map_name)"#####
    }
    b"output_phf" => r#####"Pangani ntchito za phf map pazilankhulo"#####,
    b"output_phf_all_in_one" => {
      r#####"Gwirizanitsani ma phf map onse mu ntchito imodzi"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map ndi mawu owoneka (osati TupleKey)"#####
    }
    b"output_ron" => r#####"Tulutsani mawu mu RON format"#####,
    b"suffix" => r#####"Tsatoli latsopano la "Highlight-Map""#####,
    b"syntax_name" => r#####"Dzina la syntax"#####,
    b"theme_name" => r#####"Dzina la theme"#####,
    b"true_color" => r#####"Mtundu weniweni wa 24-bit"#####,
    b"visibility" => r#####"Kuwonekera kwa code yomwe yapangidwa"#####,
    _ => "",
  }
}
