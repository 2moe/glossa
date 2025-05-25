pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kích hoạt nền hay không"#####,
    b"base_name" => r#####"Tên cơ sở của "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Hậu tố tập tin bincode"#####,
    b"custom_syntax_set" => r#####"Tập tin bộ cú pháp tùy chỉnh"#####,
    b"custom_theme_set" => r#####"Tập tin bộ chủ đề tùy chỉnh"#####,
    b"display_config_dir" => r#####"Hiển thị thư mục cấu hình glossa"#####,
    b"dsl_suffix" => r#####"Hậu tố tập tin DSL (mặc định ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Chế độ danh sách đen: Không khởi tạo ID ngôn ngữ trong danh sách"#####
    }
    b"exclude_map_names" => r#####"Không khởi tạo tên bản đồ trong danh sách"#####,
    b"include_languages" => {
      r#####"Chế độ danh sách trắng: Chỉ khởi tạo ID ngôn ngữ trong danh sách"#####
    }
    b"include_map_names" => r#####"Chỉ khởi tạo tên bản đồ trong danh sách"#####,
    b"input" => r#####"Thư mục nguồn tài nguyên bản địa hóa"#####,
    b"list_all_syntaxes" => r#####"Hiển thị tất cả tên cú pháp và phần mở rộng"#####,
    b"list_all_themes" => r#####"Hiển thị tất cả tên chủ đề"#####,
    b"mod_prefix" => r#####"Tiền tố tập tin mod (mặc định "l10n_")"#####,
    b"outdir" => r#####"Thư mục đầu ra"#####,
    b"output_bincode" => {
      r#####"Tạo các tập tin bincode độc lập cho các ngôn ngữ khác nhau"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Xuất tất cả bincode vào một tập tin duy nhất"#####
    }
    b"output_locales_fn" => r#####"Xuất hàm all_locales"#####,
    b"output_match_fn" => {
      r#####"Tạo tập tin Rust với hàm match cho từng ngôn ngữ"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kết hợp tất cả dữ liệu vào một hàm match (chuỗi)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Hàm match sử dụng tên ngôn ngữ làm khóa"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Hàm match với khóa kết hợp (tên ngôn ngữ + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Chỉ sử dụng map_key làm khóa (không chứa map_name)"#####
    }
    b"output_phf" => r#####"Tạo hàm phf map riêng cho từng ngôn ngữ"#####,
    b"output_phf_all_in_one" => r#####"Kết hợp tất cả phf map vào một hàm"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map sử dụng chuỗi thường làm khóa (không phải TupleKey)"#####
    }
    b"output_ron" => r#####"Xuất chuỗi theo định dạng RON"#####,
    b"suffix" => r#####"Hậu tố mới của "Highlight-Map""#####,
    b"syntax_name" => r#####"Tên cú pháp"#####,
    b"theme_name" => r#####"Tên chủ đề"#####,
    b"true_color" => r#####"Màu thật 24-bit"#####,
    b"visibility" => r#####"Khả năng hiển thị của mã được tạo"#####,
    _ => "",
  }
}
