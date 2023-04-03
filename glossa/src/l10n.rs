use crate::{
    assets::{localisation::locale_hashmap, OnceCell},
    MapLoader,
};

/// The function creates a `OnceCell` named `RES` that can hold an instance of `MapLoader`. The `OnceCell` is used to ensure that the `MapLoader` instance is created only once and subsequent calls to `locales` return the same instance.
pub fn locales() -> &'static MapLoader {
    // Create a new `OnceCell` that can hold a `MapLoader` instance.
    static RES: OnceCell<MapLoader> = OnceCell::new();
    // Retrieve the `MapLoader` instance from the `OnceCell`, or create a new one if it has not been initialized yet.
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn static_loader() {
        use crate::map_loader::get::GetText;
        let loader = locales();
        let msg = loader.get("map-name", "key-name");
        dbg!(&msg);
    }
}
