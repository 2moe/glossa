use glossa_shared::display::puts;

use super::*;
use crate::{Visibility, generator::dbg_generator::new_generator};

#[ignore]
#[test]
fn test_output_router_map() -> io::Result<()> {
  new_generator()
    .with_visibility(Visibility::Pub)
    .output_router_for_match_fns(MapType::Regular, false)?
    .pipe_ref(puts)
    .pipe(Ok)
}
