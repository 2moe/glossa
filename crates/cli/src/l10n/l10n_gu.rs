pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"બેકગ્રાઉન્ડ સક્રિય કરવું"#####,
    b"base_name" => r#####""હાઇલાઇટ-મેપ" નું બેઝ નામ"#####,
    b"bincode_suffix" => r#####"bincode ફાઇલ સફિક્સ"#####,
    b"custom_syntax_set" => r#####"કસ્ટમ સિન્ટેક્સ સેટ ફાઇલ"#####,
    b"custom_theme_set" => r#####"કસ્ટમ થીમ સેટ ફાઇલ"#####,
    b"display_config_dir" => r#####"Glossa કોન્ફિગ ડિરેક્ટરી બતાવો"#####,
    b"dsl_suffix" => r#####"DSL ફાઇલ સફિક્સ (ડિફૉલ્ટ ".dsl")"#####,
    b"exclude_languages" => r#####"બ્લેકલિસ્ટ મોડ: લિસ્ટના ભાષા ID સક્રિય ન કરો"#####,
    b"exclude_map_names" => r#####"લિસ્ટના મેપ નામો સક્રિય ન કરો"#####,
    b"include_languages" => {
      r#####"વ્હાઇટલિસ્ટ મોડ: ફક્ત લિસ્ટમાંના ભાષા ID સક્રિય કરો"#####
    }
    b"include_map_names" => r#####"ફક્ત લિસ્ટમાંના મેપ નામો સક્રિય કરો"#####,
    b"input" => r#####"લોકલાઇઝેશન સ્રોતોની સોર્સ ડિરેક્ટરી"#####,
    b"list_all_syntaxes" => r#####"બધા સિન્ટેક્સ નામો અને એક્સ્ટેન્શન્સ બતાવો"#####,
    b"list_all_themes" => r#####"બધી થીમ્સ બતાવો"#####,
    b"mod_prefix" => r#####"mod ફાઇલ પ્રિફિક્સ (ડિફૉલ્ટ "l10n_")"#####,
    b"outdir" => r#####"આઉટપુટ ડિરેક્ટરી"#####,
    b"output_bincode" => r#####"વિવિધ ભાષાઓ માટે સ્વતંત્ર bincode ફાઇલો જનરેટ કરો"#####,
    b"output_bincode_all_in_one" => {
      r#####"બધી ભાષાઓના bincode એક જ ફાઇલમાં એક્સપોર્ટ કરો"#####
    }
    b"output_locales_fn" => r#####"all_locales ફંક્શન એક્સપોર્ટ કરો"#####,
    b"output_match_fn" => r#####"રસ્ટ કોડ ફાઇલો match એક્સપ્રેશન સાથે જનરેટ કરો"#####,
    b"output_match_fn_all_in_one" => {
      r#####"બધા ડેટાને એક match ફંક્શનમાં એક્સપોર્ટ કરો (સ્ટ્રિંગ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ભાષાના નામને કી તરીકે વાપરતી match ફંક્શન"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"કોમ્બિનેશન કી (ભાષા+map_key) સાથે match ફંક્શન"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"માત્ર map_key ને કી તરીકે વાપરો (map_name નહીં)"#####
    }
    b"output_phf" => r#####"ભાષા-આધારિત phf મેપ ફંક્શન્સ જનરેટ કરો"#####,
    b"output_phf_all_in_one" => r#####"બધી phf મેપ્સને એક ફંક્શનમાં એકત્રિત કરો"#####,
    b"output_phf_without_map_name" => r#####"સાદા સ્ટ્રિંગ કી સાથે phf મેપ (TupleKey નહીં)"#####,
    b"output_ron" => r#####"RON ફોર્મેટમાં સ્ટ્રિંગ એક્સપોર્ટ કરો"#####,
    b"suffix" => r#####""હાઇલાઇટ-મેપ" નું નવું સફિક્સ"#####,
    b"syntax_name" => r#####"સિન્ટેક્સ નામ"#####,
    b"theme_name" => r#####"થીમ નામ"#####,
    b"true_color" => r#####"24-બિટ ટ્રૂ કલર"#####,
    b"visibility" => r#####"જનરેટ થયેલ કોડની દૃશ્યમાનતા"#####,
    _ => "",
  }
}
