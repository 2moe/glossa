pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"เปิดใช้งานพื้นหลัง"#####,
    b"base_name" => r#####"ชื่อพื้นฐานของ "Highlight-Map""#####,
    b"bincode_suffix" => r#####"นามสกุลไฟล์ bincode"#####,
    b"custom_syntax_set" => r#####"ไฟล์ชุดไวยากรณ์กำหนดเอง"#####,
    b"custom_theme_set" => r#####"ไฟล์ชุดธีมกำหนดเอง"#####,
    b"display_config_dir" => r#####"แสดงไดเรกทอรีการตั้งค่า glossa"#####,
    b"dsl_suffix" => r#####"นามสกุลไฟล์ DSL (ค่าเริ่มต้น ".dsl")"#####,
    b"exclude_languages" => r#####"โหมดแบล็คลิสต์: ไม่เริ่มต้น ID ภาษาในรายการ"#####,
    b"exclude_map_names" => r#####"ไม่เริ่มต้นชื่อแผนที่ในรายการ"#####,
    b"include_languages" => r#####"โหมดไวท์ลิสต์: เริ่มต้นเฉพาะ ID ภาษาในรายการ"#####,
    b"include_map_names" => r#####"เริ่มต้นเฉพาะชื่อแผนที่ในรายการ"#####,
    b"input" => r#####"ไดเรกทอรีต้นทางของทรัพยากรท้องถิ่น"#####,
    b"list_all_syntaxes" => r#####"แสดงชื่อไวยากรณ์และนามสกุลทั้งหมด"#####,
    b"list_all_themes" => r#####"แสดงชื่อธีมทั้งหมด"#####,
    b"mod_prefix" => r#####"คำนำหน้าไฟล์ mod (ค่าเริ่มต้น "l10n_")"#####,
    b"outdir" => r#####"ไดเรกทอรีผลลัพธ์"#####,
    b"output_bincode" => r#####"สร้างไฟล์ bincode แยกสำหรับแต่ละภาษา"#####,
    b"output_bincode_all_in_one" => r#####"ส่งออก bincode ทุกภาษาเป็นไฟล์เดียว"#####,
    b"output_locales_fn" => r#####"ส่งออกฟังก์ชัน all_locales"#####,
    b"output_match_fn" => r#####"สร้างไฟล์ Rust แยกพร้อมฟังก์ชัน match expression"#####,
    b"output_match_fn_all_in_one" => {
      r#####"ส่งออกข้อมูลทั้งหมดเป็นฟังก์ชัน match เดียว (สตริง)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ฟังก์ชัน match ใช้ชื่อภาษาเป็นคีย์"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"ฟังก์ชัน match ใช้คีย์รวม (ชื่อภาษา + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"ใช้เฉพาะ map_key เป็นคีย์ (ไม่มี map_name)"#####
    }
    b"output_phf" => r#####"สร้างฟังก์ชัน phf map แยกสำหรับแต่ละภาษา"#####,
    b"output_phf_all_in_one" => r#####"รวม phf map ทั้งหมดเป็นฟังก์ชันเดียว"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map ใช้คีย์สตริงธรรมดา (ไม่ใช้ TupleKey)"#####
    }
    b"output_ron" => r#####"ส่งออกสตริงในรูปแบบ RON"#####,
    b"suffix" => r#####"นามสกุลใหม่ของ "Highlight-Map""#####,
    b"syntax_name" => r#####"ชื่อไวยากรณ์"#####,
    b"theme_name" => r#####"ชื่อธีม"#####,
    b"true_color" => r#####"สีจริง 24-bit"#####,
    b"visibility" => r#####"การมองเห็นของโค้ดที่สร้าง"#####,
    _ => "",
  }
}
