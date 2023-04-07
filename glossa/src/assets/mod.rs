pub(crate) mod localisation;

use crate::LangID;
#[cfg(feature = "ahash")]
pub use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
pub use std::collections::HashMap;

#[doc(inline)]
pub use lang_id::consts as lang_id_consts;

pub use lang_id;

#[cfg(feature = "log")]
pub use log;

pub use once_cell::sync::OnceCell;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_l10n_map() {
        let map = localisation::locale_map();
        // dbg!(map);
        for k in map.keys() {
            println!("{k}")
        }
    }
}
