# glossa-cli

[![glossa-cli.crate](https://img.shields.io/crates/v/glossa-cli.svg?logo=rust&logoColor=lightsalmon&label=glossa-cli)](https://crates.io/crates/glossa-cli)
[![Documentation](https://docs.rs/glossa-cli/badge.svg)](https://docs.rs/glossa-cli)

[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-cli.svg?logo=apache)](../License)

<!-- Language -->

<details>
<summary>
<a href="Readme-zh.md">
<img alt="Language/语言" src="./svg/language.svg"/>
</a>
</summary>

- en: English
- [zh: 中文](Readme-zh.md)
- [zh-Hant: 繁體中文](Readme-zh-Hant.md)

</details>

<!-- TOC -->

<details open>
<summary>
<img alt="Table of Contents" src="./svg/toc/toc.svg" />
</summary>

</details>

---

## Overview

Run `glossa-cli -h`:

```sh
[INFO  glossa_cli::envs] env:
      GLOSSA_LANG: None
      GLOSSA_L10N_DIR: None
      GLOSSA_CFG_DIR: None
      GLOSSA_LOG: None

Usage: glossa-cli [OPTIONS]

Options:
      --map-type <regular|highlight|dsl|_>  [default: regular]
  -h, --help                                Print help (see more with '--help')
  -V, --version                             Print version

L10nResources:
  -i, --input </path/to/L10nDir>
          source directory of localization resources [default: locales]
      --dsl-suffix <String>
          DSL file suffix (default: ".dsl")
      --include-languages <en,zh,fr,ru,ar,es,etc.>
          allow list mode: only initialize listed language IDs [aliases: --langs]
      --include-map-names <name1,name2,..>
          allow list mode: only initialize listed map names [aliases: --maps]
      --exclude-languages <lang1,lang2,..>
          block list mode: exclude listed language IDs [aliases: --ex-langs]
      --exclude-map-names <name1,name2,..>
          block list mode: exclude listed map names [aliases: --ex-maps]

Generator Core:
  -o, --outdir </path/to/output_dir>
          output directory
      --visibility <pub| pub(crate)| pub(super)| private>
          visibility of generated code [aliases: --vis]
      --mod-visibility <pub| pub(crate)| pub(super)| private>
          [aliases: --mod-vis]
  -b, --bincode-suffix <String>
          bincode file suffix
  -m, --mod-prefix <String>
          mod file prefix (default: "l10n_")
  -f, --feature-prefix <String>


Output Files:
      --output-bincode
          generate separate bincode files per language
      --output-match-fn
          generate Rust code files with match expressions per language
      --output-match-fn-without-map-name
          similar to output_match_fn, but generates functions using map_key only (without map_name)
      --output-phf
          generate PHF map functions per language
      --output-phf-without-map-name
          PHF maps with simple string keys (instead of TupleKey)

Output File:
      --output-bincode-all-in-one
          output all languages' bincode into a single file
      --output-match-fn-all-in-one
          consolidate all language data into a single match function (string)
      --output-match-fn-all-in-one-by-language
          single match function (string) with language name as key
      --output-match-fn-all-in-one-without-map-name
          single match function (string) with composite key (language + map_key)
      --output-phf-all-in-one
          output all PHF maps into a unified function

Output String:
      --output-ron                                    output RON-formatted strings
      --output-locales-fn                             generate all_locales function
      --output-raw-locales
      --output-mod-rs
      --output-cargo-features
      --output-router-for-match-fns
      --output-router-for-match-fns-without-map-name
      --output-router-for-phf-maps
      --output-router-for-phf-maps-without-map-name

HighlightKey:
      --base-name <Vec<String>>  base "Highlight-Map" name
      --suffix <Vec<String>>     suffix for new Highlight-Maps

HighlightValue:
      --true-color <Vec<bool>>
          24-bit true color [possible values: true, false]
      --syntax-name <Vec<String>>
          syntax name
      --theme-name <Vec<String>>
          theme name
      --background <Vec<bool>>
          enable background support [possible values: true, false]
      --custom-syntax-set <Vec<"/path/to/syntaxset-file">>
          custom syntax set file
      --custom-theme-set <Vec<"/path/to/themeset-file">>
          custom theme set file

HighlightValue Debug:
      --list-all-syntaxes  display all syntax names with extensions
      --list-all-themes    display all theme names

Debug:
      --display-config-dir
```

### Argument Type Specifications

- **Enum selection** `(a | b | c | ...)`
  Accepts a single value to specify enum variant.
  Example:
  `--map-type <regular|highlight|dsl|_>`
  - `--map-type regular`=> `MapType::Regular`
  - `--map-type highlight`=> `MapType::Highlight`
  - `--map-type dsl`=> `MapType::DSL`
  - `--map-type _`=> `MapType::RegularAndHighlight`

- **Comma-separated values** (`a,b,c,...`):
  Indicates the parameter accepts multiple values separated by commas.
  Example:
  `--exclude-languages <lang1,lang2,...>  [alias: --ex-langs]`
  For `--ex-langs ar,gsw,en-GB`, the program will internally collect them as `["ar", "gsw", "en-GB"]`.

- **Vector collection** (`Vec<T>`):
  Indicates the parameter requires multiple invocations of the option to collect values.
  Example:
  For `--true-color <Vec<bool>>`, use multiple calls:
  `--true-color true --true-color false`
  The program will collect them as `[true, false]`.

- **Hash map handling** (`ahash::HashMap<K, V>`):
  Though parameters like `HighlightKey` and `HighlightValue` show `Vec<T>` type, they're automatically converted to `ahash::HashMap<K, V>` internally.
  Note:
  - Key-related options can be called more times than Value options
  - Missing Value options will use default values automatically.

## Quick Start

glossa-cli is a localization resource generator that can convert localization resources into Rust code or bincode.

> As the proverb wisely puts it, "Even the cleverest housewife cannot bake bread without flour." Every accomplishment begins with the right preparation.

The prerequisite for using glossa-cli is to prepare resource files first.

If you don't have localization resources, you can directly use glossa's git repository.

> Note: glossa-cli itself uses its own capabilities (glossa-cli) to generate code for localization resources.

POSIX-sh:

```sh
# Enter temporary directory
cd `mktemp -d`

# Download archive directly
curl -LO https://github.com/2moe/glossa/archive/refs/heads/dev.tar.gz

# Extract
tar -xf dev.tar.gz

# Enter glossa-dev directory
cd glossa-dev
```

Try a simple test:

```sh
glossa-cli -i ./locales --output-ron
```

> Since the default value for -i/--input is "locales", we can omit it

You can run all options starting with `--output-` to understand their purposes.

```sh
glossa-cli --output-bincode
ls tmp
```

```sh
glossa-cli --output-match-fn-all-in-one
```

```sh
# ...
# glossa-cli --output-...
# ...
```

When you don't need all maps or languages, use `--include-languages` and `--include-map-names`:

```sh
glossa-cli --output-match-fn-all-in-one-without-map-name --include-languages de,pt,fr,ru --include-map-names cli
```

```sh
glossa-cli --output-match-fn-all-in-one --include-languages en,es --include-map-names yes-no,error
```

## Environment Variables

- GLOSSA_LANG
  - Specifies the language used by glossa_cli itself, e.g., `env GLOSSA_LANG=gsw glossa-cli`
  - Default: Automatically detected from OS language settings
    - For wasi-p1 environments, manual specification is required as auto-detection is unavailable
- GLOSSA_L10N_DIR
  - Specifies the localization resource directory for glossa_cli itself, e.g., `env GLOSSA_L10N_DIR=/path/to/bincode glossa-cli`
  - When empty, uses `$GLOSSA_CFG_DIR/bincode`
- GLOSSA_CFG_DIR
  - Specifies glossa_cli's configuration directory, e.g., `env GLOSSA_CFG_DIR=/path/to/cfg_dir glossa-cli`
  - Default paths by OS:
    - macOS: `"/Users/[username]/Library/Application Support/me.tmoe.glossa"`
    - Linux: `"${XDG_CONFIG_HOME:-$HOME/.config}/glossa"`
    - Windows: `"C:\Users\[username]\AppData\Local\tmoe\glossa\config"`
    - WASI: `tmp/me.tmoe.glossa/config`
- GLOSSA_LOG
  - Logging level
  - Default: info
  - Options: trace, debug, info, warn, error
