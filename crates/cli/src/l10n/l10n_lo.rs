pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"ເປີດໃຊ້ພື້ນຫຼັງ"#####,
    b"base_name" => r#####"ຊື່ພື້ນຖານ "ແຜນທີ່ເດັ່ນ""#####,
    b"bincode_suffix" => r#####"ນາມສະກຸນໄຟລ໌ bincode"#####,
    b"custom_syntax_set" => r#####"ໄຟລ໌ກຳນົດໂຄງສ້າງສະເພາະ"#####,
    b"custom_theme_set" => r#####"ໄຟລ໌ກຳນົດຫົວຂໍ້ສະເພາະ"#####,
    b"display_config_dir" => r#####"ສະແດງໂຟນເດີຕັ້ງຄ່າ glossa"#####,
    b"dsl_suffix" => r#####"ນາມສະກຸນ DSL (ຄ່າເລີ່ມ ".dsl")"#####,
    b"exclude_languages" => r#####"ໂໝດບັນຊີດຳ: ບໍ່ເລີ່ມ ID ພາສາໃນລາຍຊື່"#####,
    b"exclude_map_names" => r#####"ບໍ່ເລີ່ມຊື່ແຜນທີ່ໃນລາຍຊື່"#####,
    b"include_languages" => r#####"ໂໝດບັນຊີຂາວ: ເລີ່ມພຽງ ID ພາສາໃນລາຍຊື່"#####,
    b"include_map_names" => r#####"ເລີ່ມພຽງຊື່ແຜນທີ່ໃນລາຍຊື່"#####,
    b"input" => r#####"ໂຟນເດີແຫຼ່ງຂໍ້ມູນທ້ອງຖິ່ນ"#####,
    b"list_all_syntaxes" => r#####"ສະແດງຊື່ໂຄງສ້າງໝົດທຸກຊື່ ແລະ ນາມສະກຸນ"#####,
    b"list_all_themes" => r#####"ສະແດງຊື່ຫົວຂໍ້ໝົດທຸກຊື່"#####,
    b"mod_prefix" => r#####"ຄຳນຳໜ້າ mod (ຄ່າເລີ່ມ "l10n_")"#####,
    b"outdir" => r#####"ໂຟນເດີຜົນອອກ"#####,
    b"output_bincode" => r#####"ສ້າງໄຟລ໌ bincode ແຍກຕ່າງກັນສຳລັບພາສາຕ່າງໆ"#####,
    b"output_bincode_all_in_one" => r#####"ສົ່ງ bincode ທຸກພາສາເຂົ້າໄຟລ໌ດຽວ"#####,
    b"output_locales_fn" => r#####"ສົ່ງຟັງຊັ່ນ all_locales"#####,
    b"output_match_fn" => r#####"ສ້າງໄຟລ໌ Rust ພາສາແຍກກັນ (ມີຟັງຊັ່ນ match)"#####,
    b"output_match_fn_all_in_one" => r#####"ສົ່ງຂໍ້ມູນທັງໝົດເຂົ້າໄຟລ໌ match ດຽວ (ສະຕຣິງ)"#####,
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ຟັງຊັ່ນ match ໂດຍໃຊ້ຊື່ພາສາເປັນກະແຈ"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ຟັງຊັ່ນ match ດ້ວຍກະແຈປະສົມ (ຊື່ພາສາ+map_key)"#####
    }
    b"output_match_fn_without_map_name" => r#####"ໃຊ້ map_key ເປັນກະແຈຢ່າງດຽວ (ບໍ່ມີ map_name)"#####,
    b"output_phf" => r#####"ສ້າງຟັງຊັ່ນ phf map ແຍກຕ່າງຫາພາສາ"#####,
    b"output_phf_all_in_one" => r#####"ສົ່ງ phf map ທັງໝົດເຂົ້າໄຟລ໌ດຽວ"#####,
    b"output_phf_without_map_name" => r#####"phf map ດ້ວຍກະແຈສະຕຣິງທຳມະດາ (ບໍ່ແມ່ນ TupleKey)"#####,
    b"output_ron" => r#####"ສົ່ງຂໍ້ມູນເປັນສະຕຣິງ RON"#####,
    b"suffix" => r#####"ນາມສະກຸນໃໝ່ຂອງ "ແຜນທີ່ເດັ່ນ""#####,
    b"syntax_name" => r#####"ຊື່ໂຄງສ້າງ"#####,
    b"theme_name" => r#####"ຊື່ຫົວຂໍ້"#####,
    b"true_color" => r#####"ສີຈິງ 24 ບິດ"#####,
    b"visibility" => r#####"ການເຫັນໄດ້ຂອງລະຫັດທີ່ສ້າງ"#####,
    _ => "",
  }
}
