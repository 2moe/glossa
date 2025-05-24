pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Cuir cùlaibh an gnìomh"#####,
    b"base_name" => r#####"Ainm bunait "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Iar-leasachan faidhle bincode"#####,
    b"custom_syntax_set" => r#####"Faidhle seata co-chòdadh gnàthaichte"#####,
    b"custom_theme_set" => r#####"Faidhle seata cuspairean gnàthaichte"#####,
    b"display_config_dir" => r#####"Seall clàr rèiteachaidh glossa"#####,
    b"dsl_suffix" => r#####"Iar-leasachan faidhle DSL (bun-suidhte ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modh liosta dubh: Na stèidhich ID cànain às a' liosta"#####
    }
    b"exclude_map_names" => r#####"Na stèidhich ainmean mapa às a' liosta"#####,
    b"include_languages" => {
      r#####"Modh liosta geal: Dìth stèidheachadh ID cànain às a' liosta"#####
    }
    b"include_map_names" => {
      r#####"Dìth stèidheachadh ainmean mapa às a' liosta"#####
    }
    b"input" => r#####"Clàr stòiridh goireasan ionadail"#####,
    b"list_all_syntaxes" => r#####"Seall ainmean co-chòdadh is leudachain"#####,
    b"list_all_themes" => r#####"Seall ainmean cuspairean uile"#####,
    b"mod_prefix" => r#####"Ro-leasachan faidhle mod (bun-suidhte "l10n_")"#####,
    b"outdir" => r#####"Clàr nan Tìoraidh"#####,
    b"output_bincode" => {
      r#####"Gin faidhlichean bincode neo-eisimeileach airson diofar chànanan"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Cuir a-mach bincode nan cànan uile gu aon fhaidhle"#####
    }
    b"output_locales_fn" => r#####"Cuir a-mach gnìomh all_locales"#####,
    b"output_match_fn" => {
      r#####"Gin faidhlichean Rust le gnìomhan match airson gach cànan"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Cuir a-mach dàta uile ann an aon ghnìomh match (sreang)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Gnìomh match le ainm cànain mar iuchair"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Gnìomh match le iuchair còmhla (ainm cànain + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Cleachd map_key mar iuchair a-mhàin (gun map_name)"#####
    }
    b"output_phf" => r#####"Gin gnìomhan phf map airson diofar chànanan"#####,
    b"output_phf_all_in_one" => {
      r#####"Cuir a-mach phf map uile ann an aon ghnìomh"#####
    }
    b"output_phf_by_key" => {
      r#####"phf map le iuchraichean sreang àbhaisteach (chan e TupleKey)"#####
    }
    b"output_ron" => r#####"Cuir a-mach sreang ann an cruth RON"#####,
    b"suffix" => r#####"Iar-leasachan ùr "Highlight-Map""#####,
    b"syntax_name" => r#####"Ainm co-chòdadh"#####,
    b"theme_name" => r#####"Ainm cuspair"#####,
    b"true_color" => r#####"Dath fìor 24-bit"#####,
    b"visibility" => r#####"So-fhaicsinneachd còd ginte"#####,
    _ => "",
  }
}
