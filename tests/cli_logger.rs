pub(crate) fn init_logger(trace: bool) {
  let level = {
    use log::LevelFilter::*;
    if trace { Trace } else { Debug }
  };

  env_logger::builder()
    .filter_level(level)
    .init()
}
