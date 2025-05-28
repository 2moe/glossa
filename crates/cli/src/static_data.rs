use std::path::{Path, PathBuf};

use glossa::sys::new_once_lock;
use glossa_codegen::glossa_shared::{fmt_compact, tap::Tap};
use log::{debug, trace};

/// Retrieves the static configuration directory path.
///
/// Initialization sequence:
/// 1. Checks `GLOSSA_CFG_DIR` environment variable (via
///    [`crate::envs::static_glossa_cfg_dir`])
/// 2. Attempts OS-standard config directory using `ProjectDirs`
/// 3. Falls back to platform-specific temporary directory:
///   - Unix/Windows: `<system_temp_dir>/me.tmoe.glossa/config`
///   - Other platforms(e.g., wasi): `tmp/me.tmoe.glossa/config`
///
/// Example resolved paths:
///
/// - Linux: `/home/[user]/.config/glossa/config`
/// - macOS: `/Users/[user]/Library/Application Support/me.tmoe.glossa`
/// - Windows: `C:\Users\[user]\AppData\Local\tmoe\glossa\config`
/// - Fallback: `tmp/me.tmoe.glossa/config`
pub fn config_dir() -> &'static Path {
  new_once_lock!(V: PathBuf);

  V.get_or_init(|| {
    trace!("Initializing config_dir...");

    let proj_dir = || {
      trace!("Attempting to get ProjectDirs...");
      let (qual, org, app) = ("me", "tmoe", "glossa");

      match directories::ProjectDirs::from(qual, org, app)
        .map(|d| d.config_local_dir().into())
        .inspect(|p| debug!("ProjectDirs found, using config_local_dir: {p:?}"))
      {
        Some(v) => v,
        _ => {
          let cfg_dir = fmt_compact!("{qual}.{org}.{app}/config");
          match () {
            #[cfg(any(unix, windows))]
            () => std::env::temp_dir().join(cfg_dir),
            #[cfg(not(any(unix, windows)))]
            () => {
              use glossa_codegen::glossa_shared::tap::Pipe;
              fmt_compact!("tmp/{cfg_dir}").pipe(PathBuf::from)
            }
          }
          .tap(|f| debug!("ProjectDirs not found, falling back to temp dir: {f:?}"))
        }
      }
    };

    match crate::envs::static_glossa_cfg_dir() {
      Some(v) => v.into(),
      _ => proj_dir(),
    }
  })
}

pub fn bincode_dir() -> Option<&'static Path> {
  new_once_lock!(P: Option<PathBuf>);

  P.get_or_init(|| {
    let dir = config_dir();
    dir
      .exists()
      .then(|| dir.join("bincode"))
      .filter(|x| x.exists())
  })
  .as_deref()
}
