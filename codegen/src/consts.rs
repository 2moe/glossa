pub const fn default_l10n_dir_arr() -> [&'static str; 2] {
    ["assets", "l10n"]
}

pub const fn parent_l10n_dir_arr() -> [&'static str; 3] {
    ["..", "assets", "l10n"]
}

pub const fn default_l10n_rs_file_arr() -> [&'static str; 3] {
    ["src", "assets", "localisation.rs"]
}

#[macro_export]
macro_rules! get_pkg_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
