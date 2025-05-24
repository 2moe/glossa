pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ar įjungti foną"#####,
    b"base_name" => r#####""Highlight-Map" bazinis pavadinimas"#####,
    b"bincode_suffix" => r#####"bincode failo plėtinys"#####,
    b"custom_syntax_set" => r#####"Pasirinktinis sintaksės rinkinio failas"#####,
    b"custom_theme_set" => r#####"Pasirinktinis temų rinkinio failas"#####,
    b"display_config_dir" => r#####"Rodyti Glossa konfigūracijos katalogą"#####,
    b"dsl_suffix" => r#####"DSL failo plėtinys (numatytasis ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Juodasis sąrašas: Neinicijuoti sąraše esančių kalbų ID"#####
    }
    b"exclude_map_names" => {
      r#####"Neinicijuoti sąraše esančių žemėlapių pavadinimų"#####
    }
    b"include_languages" => {
      r#####"Baltasis sąrašas: Inicijuoti tik sąraše esančius kalbų ID"#####
    }
    b"include_map_names" => {
      r#####"Inicijuoti tik sąraše esančius žemėlapių pavadinimus"#####
    }
    b"input" => r#####"Lokalizuotų resursų šaltinio katalogas"#####,
    b"list_all_syntaxes" => {
      r#####"Rodyti visus sintaksės pavadinimus ir plėtinius"#####
    }
    b"list_all_themes" => r#####"Rodyti visas temas"#####,
    b"mod_prefix" => r#####"Mod failo prefiksas (numatytasis "l10n_")"#####,
    b"outdir" => r#####"Išvesties katalogas"#####,
    b"output_bincode" => {
      r#####"Generuoti atskirus bincode failus skirtingoms kalboms"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eksportuoti visų kalbų bincode į vieną failą"#####
    }
    b"output_locales_fn" => r#####"Eksportuoti all_locales funkciją"#####,
    b"output_match_fn" => {
      r#####"Generuoti Rust kodo failus su match išraiškomis kiekvienai kalbai"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Visų kalbų duomenis eksportuoti į vieną match funkciją (eilutė)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija su kalbos pavadinimu kaip raktu"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funkcija su kombinuotu raktu (kalba + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Naudoti tik map_key kaip raktą (be map_name)"#####
    }
    b"output_phf" => {
      r#####"Generuoti atskiras phf žemėlapio funkcijas kiekvienai kalbai"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Visus phf žemėlapius sujungti į vieną funkciją"#####
    }
    b"output_phf_by_key" => {
      r#####"phf žemėlapis su paprastais eilutės raktais (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Eksportuoti eilutę RON formatu"#####,
    b"suffix" => r#####""Highlight-Map" naujas plėtinys"#####,
    b"syntax_name" => r#####"Sintaksės pavadinimas"#####,
    b"theme_name" => r#####"Temos pavadinimas"#####,
    b"true_color" => r#####"24-bitų tikros spalvos"#####,
    b"visibility" => r#####"Sugeneruoto kodo matomumas"#####,
    _ => "",
  }
}
