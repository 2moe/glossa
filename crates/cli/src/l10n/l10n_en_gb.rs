pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"include_languages" => {
      r#####"allow list mode: only initialise listed language IDs"#####
    }
    b"include_map_names" => {
      r#####"allow list mode: only initialise listed map names"#####
    }
    b"input" => r#####"source directory of localisation resources"#####,
    b"true_color" => r#####"24-bit true colour"#####,
    _ => "",
  }
}
