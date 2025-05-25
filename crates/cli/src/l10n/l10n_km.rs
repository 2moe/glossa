pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"បើកប្រើផ្ទៃខាងក្រោយ"#####,
    b"base_name" => r#####"ឈ្មោះមូលដ្ឋាននៃ "Highlight-Map""#####,
    b"bincode_suffix" => r#####"ចុងក្រោយរបស់ឯកសារ bincode"#####,
    b"custom_syntax_set" => r#####"ឯកសារកំណត់សិន្តាករ"#####,
    b"custom_theme_set" => r#####"ឯកសារកំណត់អត្ថបទ"#####,
    b"display_config_dir" => r#####"បង្ហាញថតកំណត់រចនាសម្ព័ន្ធ glossa"#####,
    b"dsl_suffix" => r#####"ចុងក្រោយឯកសារ DSL (លំនាំដើម ".dsl")"#####,
    b"exclude_languages" => r#####"របៀបបញ្ជីខ្មៅ៖ មិនចាប់ផ្តើម ID ភាសាក្នុងបញ្ជី"#####,
    b"exclude_map_names" => r#####"មិនចាប់ផ្តើម map_names ក្នុងបញ្ជី"#####,
    b"include_languages" => r#####"របៀបបញ្ជីសោភ័ណ្ឌ៖ ចាប់ផ្តើមតែ ID ភាសាក្នុងបញ្ជី"#####,
    b"include_map_names" => r#####"ចាប់ផ្តើមតែ map_names ក្នុងបញ្ជី"#####,
    b"input" => r#####"ថតប្រភពនៃធនធានក្នុងតំបន់"#####,
    b"list_all_syntaxes" => r#####"បង្ហាញឈ្មោះសិន្តាករ និងផ្នែកបន្ថែមទាំងអស់"#####,
    b"list_all_themes" => r#####"បង្ហាញឈ្មោះអត្ថបទទាំងអស់"#####,
    b"mod_prefix" => r#####"បុព្វបទឯកសារ mod (លំនាំដើម "l10n_")"#####,
    b"outdir" => r#####"ថតលទ្ធផល"#####,
    b"output_bincode" => r#####"បង្កើតឯកសារ bincode ដាច់ដោយឡែកសម្រាប់ភាសាផ្សេងៗ"#####,
    b"output_bincode_all_in_one" => r#####"នាំចេញ bincode ទាំងអស់ទៅឯកសារតែមួយ"#####,
    b"output_locales_fn" => r#####"នាំចេញមុខងារ all_locales"#####,
    b"output_match_fn" => {
      r#####"បង្កើតឯកសារ Rust ដាច់ដោយឡែកជាមួយមុខងារ match expression"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"បញ្ចូលទិន្នន័យទាំងអស់ទៅក្នុងមុខងារ match តែមួយ (ខ្សែអក្សរ)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"មុខងារ match ដោយប្រើឈ្មោះភាសាជា key"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"មុខងារ match ដោយប្រើ key រួម (ភាសា + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"ប្រើតែ map_key ជា key ដោយមិនបញ្ចូល map_name"#####
    }
    b"output_phf" => r#####"បង្កើតមុខងារ phf map ដាច់ដោយឡែកសម្រាប់ភាសា"#####,
    b"output_phf_all_in_one" => r#####"បញ្ចូល phf map ទាំងអស់ទៅក្នុងមុខងារតែមួយ"#####,
    b"output_phf_without_map_name" => r#####"phf map ដោយប្រើខ្សែអក្សរធម្មតា (មិនមែន TupleKey)"#####,
    b"output_ron" => r#####"នាំចេញជាខ្សែអក្សរ RON"#####,
    b"suffix" => r#####"ចុងក្រោយសម្រាប់ Highlight-Map ថ្មី"#####,
    b"syntax_name" => r#####"ឈ្មោះសិន្តាករ"#####,
    b"theme_name" => r#####"ឈ្មោះអត្ថបទ"#####,
    b"true_color" => r#####"ពណ៌ពិត 24-bit"#####,
    b"visibility" => r#####"ការមើលឃើញនៃកូដដែលបានបង្កើត"#####,
    _ => "",
  }
}
