use std::path::{Path, PathBuf};

use glossa::sys::new_once_lock;

pub fn config_dir() -> &'static Path {
  new_once_lock!(V: PathBuf);

  V.get_or_init(|| {
    let proj_dir =
      || match directories::ProjectDirs::from("me", "tmoe", "glossa-cli")
        .map(|d| d.config_local_dir().into())
      {
        Some(v) => v,
        _ => std::env::temp_dir().join("me.tmoe.glossa-cli"),
      };

    match crate::envs::static_glossa_cfg_dir() {
      Some(v) => v.into(),
      _ => proj_dir(),
    }
  })
}
