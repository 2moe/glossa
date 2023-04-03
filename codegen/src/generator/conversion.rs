pub(crate) fn convert_to_fn_name(s: &str) -> String {
    s.to_ascii_lowercase()
        .chars()
        .map(|c| match c.is_ascii_alphanumeric() {
            false => '_',
            _ => c,
        })
        .fold(
            (false, String::with_capacity(s.len())),
            |(prev_underscore, mut acc), c| {
                if prev_underscore && c == '_' {
                    (true, acc)
                } else {
                    acc.push(c);
                    (c == '_', acc)
                }
            },
        )
        .1
}
