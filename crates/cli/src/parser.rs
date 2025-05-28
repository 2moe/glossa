use std::fs;

use clap::Parser;
use glossa::fallback::dbg_ref;
use glossa_codegen::{
  AnyResult,
  glossa_shared::{
    display::puts_dbg,
    tap::{Pipe, Tap},
  },
};
use log::{debug, info, trace};

mod collect;
mod init;
mod output;
mod resources;

use crate::{options::Cli, parser::output::output_data, static_data};

impl Cli {
  pub fn run() -> AnyResult<()> {
    trace!("Parsing CLI...");
    let args = Cli::parse();
    trace!("Parsed CLI arguments");
    dbg_ref!(args);

    if *args
      .get_dbg_options()
      .get_display_config_dir()
    {
      static_data::config_dir() //
        .tap(puts_dbg);
    }

    debug!("Initializing generator...");
    let generator = init::init_generator(&args)
      .with_resources(init::init_resources(&args))
      .pipe(|res| {
        #[cfg(not(feature = "highlight"))]
        {
          trace!("Highlight feature disabled");
          return res;
        }

        #[cfg(feature = "highlight")]
        match init::init_highlight_cfg_map(&args) {
          Some(x) => res.with_highlight(x),
          _ => {
            debug!("Highlight config not found, proceeding without it");
            res
          }
        }
      });

    dbg_ref!(generator);

    if let Some(dir) = generator.get_outdir() {
      info!("output dir: {dir:?}");
      trace!("Creating output directory if it doesn't exist...");
      fs::create_dir_all(dir)?;
    }

    debug!("Generating output data...");
    output_data(&args, &generator)?;

    info!("CLI run completed successfully");
    Ok(())
  }
}
