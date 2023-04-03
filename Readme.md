# Glossa

[![crates.io](https://img.shields.io/crates/v/glossa.svg)](https://crates.io/crates/glossa)

[![Documentation](https://docs.rs/glossa/badge.svg)](https://docs.rs/glossa)

[![Apache-2 licensed](https://img.shields.io/crates/l/glossa.svg)](./License)

[中文](Readme-zh.md)

Glossa is a language localisation library.

## Functionality

By functional type, it can be divided into two categories.

- Compile-time: Converts the configuration file into constant (`const fn`) Rust code to achieve efficient localisation.
  - Pros: High efficiency
  - Cons:
    - Requires codegen, which may result in some redundant code expansion.
    - Currently only supports simple key-value (K-V) pairs.
- Runtime: Manages `fluent` resources.
  - Pros: Fluent syntax may be more suitable for localisation.
  - Cons: Occupies more resources than `const fn`.

Note: Fluent also supports loading localisation resources (localised files) at compile time, but data needs to be parsed at runtime.  
The former is just the simple K-V pair that uses some const maps from phf to store data. Because it's simple, it's efficient.

The two types of functionalities are independent of each other. For the latter, please read [Fluent.md](Fluent.md).

## Codegen

Use a code generator to generate code.

`glossa-codegen` has the following features:

- yaml
  - Enabled by default.
  - The default file extension is ".yaml" or ".yml"
- ron
  - The default ext is ".ron"
- toml
  - The ext is ".toml"
- json
  - ext: ".json"

This corresponds to different types of configuration files. You can enable all features or add them as needed.

By default, the file type is determined based on the file name suffix, and the **map name** (table name) is set based on the file name. Whether deserialisation is needed at compile-time is determined by the enabled feature.

Assuming there are two files under the directory `assets/l10n/en`, named `test.yaml` and `test.yml`, then we can consider them to have the same name.  
Resulting in two tables(maps):

- test
- test.yml

> When using `.get()` with `MapLoader`, you need to pass in the map-name

To avoid this situation, it is recommended to use different names for different files.

### Preparations

Before writing `build.rs`, we need to prepare the localisation resource files.

de (Deutsch, Lateinisch, Deutschland):

- "assets/l10n/de/error.yaml"

```yaml
text-not-found: Kein lokalisierter Text gefunden
```

en (English, Latin, United States):

- "assets/l10n/en/error.yaml"

```yaml
text-not-found: No localized text found
```

en-GB (English, Latin, Great Britain):

- `assets/l10n/en-GB/error.yaml`

```yaml
text-not-found: No localised text found
```

es (español, latino, España):

- `assets/l10n/es/error.yaml`

```yaml
text-not-found: No se encontró texto localizado
```

pt (português, latim, Brasil)

- `assets/l10n/pt/error.yaml`

> Note: "pt" refers to "Portuguese (Brazil)", not "Portuguese (Portugal)" (português, Portugal).

```yaml
text-not-found: Nenhum texto localizado encontrado
```

### build.rs

First, add the compile-time dependency:

```sh
cargo add --build glossa-codegen
```

Then start creating `build.rs`.

> This file is at the same level as Cargo.toml.

For a simple single-project structure:

```js
.
├── assets
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── src
└── target
```

A slightly more complex multi-project structure:

```js
.
├── assets
│   └── l10n
├── Cargo.lock
├── Cargo.toml
├── codegen
│   ├── Cargo.toml
│   ├── src
│   └── tests
├── glossa
│   ├── build.rs
│   ├── Cargo.toml
│   └── src
├── target
└── tests
```

Of course, you can also specify the path to `build.rs` manually, instead of using the default.

---

`build.rs`：

```rust
use glossa_codegen::{consts::*, prelude::*};
use std::{
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    // Specify the version as the current package version to avoid repetitive compilation for the same version.
    let version = Some(get_pkg_version!());
    // During development, we can set it to None.
    // let version = None;

    // This is a constant array: ["src", "assets", "localisation.rs"], which is converted into a path for storing automatically generated Rust code related to localisation.
    // On Windows, the path is 'src\assets\localisation.rs'.
    // On Unix, the path is "src/assets/localisation.rs".
    // Note: this is a relative path!
    let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());

    // If it's the same version, then exit.
    if is_same_version(&path, version)? {
        return Ok(());
    }

    // If the path is "src/assets/localisation.rs", then it will append `mod localisation;` and related `use` statements to "src/assets/mod.rs".
    append_to_l10n_mod(&path)?;

    // This creates a new file: "src/assets/localisation.rs".
    // Unlike append, if only create is used, the file will be cleared.
    let mut file = BufWriter::new(File::create(&path)?);

    // default_l10n_dir_arr() is also a constant array: ["assets", "l10n"].
    // If the current localisation resource path is at the parent level, then you can use `path = PathBuf::from_iter([".."].into_iter().chain(default_l10n_dir_arr()));`.
    path = PathBuf::from_iter(default_l10n_dir_arr());

    // Here, the l10n file is deserialised into a map and written to the rs file.
    // file: "src/assets/localisation.rs"
    // path: "assets/l10n"
    // visibility: Used to set the visibility of the generated `fn`. If it is None, then Some("pub(crate)") is used. You can use `Some("pub(in path)")` or `Some("pub")`
    deser_cfg_to_map(&mut file, &mut path, Some("pub(crate)"), version)
}
```

## Get Text

Now that the code has been generated, let's write a function to test it!

But before that, we need to add some dependencies.

```sh
cargo add phf glossa
```

The test function is as follows:

```rust
    #[test]
    fn new_loader() {
        use crate::assets::localisation::locale_hashmap;
        use glossa::{fallback::FallbackChain, GetText, MapLoader};

        let loader = MapLoader::new(locale_hashmap());
        loader.show_chain();
        // Here, for simplicity, `get_or_default()` is used.
        // Actually, the usage of `.get()` is the same, but it returns Result<&str>, not Cow<str>.
        let msg = loader.get_or_default("error", "text-not-found");
        assert_eq!(msg, "No localized text found");
    }
```

If your system language is "en", the test should pass.

Note that `locale_hashmap()` is not a `const fn` but a regular function.
However, this does not mean that it is particularly expensive.

The time complexity of HashMap query operation is **O(1)**.

Its value points to a sub-map, and all sub-maps and their sub-maps are `consts`.

In addition, if the `ahash` feature is enabled, the RandomState of ahash will be used by default instead of the `std::collections`.

You can also use OnceCell to create global static data, creating data only once.

```rust
pub(crate) fn locales() -> &'static MapLoader {
    static RES: OnceCell<MapLoader> = OnceCell::new();
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}
```

> Wait a minute, don't waste time on these things, our previous test failed.

All right, let's revisit what we did before.  
We have previously created localisation resource files for German, Spanish and Portuguese.

Firstly, it will automatically detect the system language. If the localisation resource does not exist, it will automatically use a fallback chain.
If the localisation resource exists and your system language is not English, then the above test will fail.

Let's continue to test:

```rust
let loader = locales();
let msg = loader.get("error", "text-not-found")?;
```

Assuming your language is German (de-Latn-DE)

```rust
assert_eq!(msg, "Kein lokalisierter Text gefunden");
```

Spanish (es-Latn-ES)

```rust
assert_eq!(msg, "No se encontró texto localizado");
```

Portuguese (pt-Latn-BR)

```rust
assert_eq!(msg, "Nenhum texto localizado encontrado");
```
