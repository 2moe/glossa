# glossa-cli

[![glossa-cli.crate](https://img.shields.io/crates/v/glossa-cli.svg?logo=rust&logoColor=lightsalmon&label=glossa-cli)](https://crates.io/crates/glossa-cli)

[![Documentation](https://docs.rs/glossa-cli/badge.svg)](https://docs.rs/glossa-cli)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-cli.svg?logo=apache)](../License)

<!-- Language -->

<details open>
<summary>
<img alt="Language/語言" src="./svg/language.svg" />
</summary>

- [zh-Hant: 繁體中文](Readme-zh-Hant.md)
- [en: English](Readme.md)
- [zh: 簡體中文](Readme-zh.md)

</details>

<!-- TOC -->

<details open>
<summary>
<img alt="目錄" src="./svg/toc/目錄.svg"/>
</summary>

- [下載與安裝](#下載與安裝)
- [Overview](#overview)
  - [引數型別說明](#引數型別說明)
- [Quick Start](#quick-start)
- [環境變數](#環境變數)

</details>

---

## 下載與安裝

TODO

## Overview

glossa-cli 本質上是 glossa-codegen 的包裝器，如需瞭解更多內容，則請閱讀 glossa-codegen 的文件。

執行 `glossa-cli -h`，我們可以得到如下內容：

```sh
[INFO  glossa_cli::envs] env:
      GLOSSA_LANG: None
      GLOSSA_L10N_DIR: None
      GLOSSA_CFG_DIR: None
      GLOSSA_LOG: None

Usage: glossa-cli [OPTIONS]

Options:
      --map-type <regular|highlight|dsl|_>  "_" 表示 regular-and-highlight [default: regular]
  -h, --help                                Print help (see more with '--help')
  -V, --version                             Print version

L10nResources:
  -i, --input </path/to/L10nDir>
          本地化資源的源目錄 [default: locales]
      --dsl-suffix <String>
          DSL 檔案字尾(預設 ".dsl")
      --include-languages <en,zh,fr,ru,ar,es,etc.>
          白名單模式。當其不為空時，只有位於列表中的語言 id 才會被初始化 [aliases: --langs]
      --include-map-names <name1,name2,..>
          當其不為空時，只有位於列表中的 map_names 才會被初始化 [aliases: --maps]
      --exclude-languages <lang1,lang2,..>
          黑名單模式。位於黑名單中的語言 id 不會被初始化 [aliases: --ex-langs]
      --exclude-map-names <name1,name2,..>
          位於列表中的 map_names 不會被初始化 [aliases: --ex-maps]

Generator Core:
  -o, --outdir </path/to/output_dir>                           輸出的目錄
      --visibility <pub| pub(crate)| pub(super)| private>      生成的程式碼的可見性 [aliases: --vis]
      --mod-visibility <pub| pub(crate)| pub(super)| private>  [aliases: --mod-vis]
  -b, --bincode-suffix <String>                                bincode檔案字尾
  -m, --mod-prefix <String>                                    mod 檔案字首 (預設 "l10n_")
  -f, --feature-prefix <String>

Output Files:
      --output-bincode
          為不同的語言生成獨立的 bincode 檔案
      --output-match-fn
          為不同的語言生成獨立的 rust 程式碼的檔案 (包含match表示式的函式)
      --output-match-fn-without-map-name
          類似於 output_match_fn，但生成的函式只以 map_key 作為 key, 不包含 map_name
      --output-phf
          為不同語言生成獨立的 phf map 函式
      --output-phf-without-map-name
          類似於 output_phf, 但生成的函式的key 為普通字串，而不是 TupleKey

Output File:
      --output-bincode-all-in-one
          將所有語言的 bincode 輸出到同一個檔案
      --output-match-fn-all-in-one
          將所有語言的資料都輸出為一個match函式（字串）
      --output-match-fn-all-in-one-by-language
          將所有語言的資料輸出為同一個match 函式（字串），key 為語言名
      --output-match-fn-all-in-one-without-map-name
          將所有語言的資料輸出為同一個 match 函式（字串），key 為語言名和 map_key
      --output-phf-all-in-one
          將所有語言的 phf map 輸出到同一個函式

Output String:
      --output-ron                                    輸出為 ron 格式的字串
      --output-locales-fn                             輸出 all_locales 函式
      --output-raw-locales
      --output-mod-rs
      --output-cargo-features
      --output-router-for-match-fns
      --output-router-for-match-fns-without-map-name
      --output-router-for-phf-maps
      --output-router-for-phf-maps-without-map-name

HighlightKey:
      --base-name <Vec<String>>  基礎"高亮Map"的名稱
      --suffix <Vec<String>>     新生成的"高亮Map"的字尾

HighlightValue:
      --true-color <Vec<bool>>
          24位真彩色 [possible values: true, false]
      --syntax-name <Vec<String>>
          語法名稱
      --theme-name <Vec<String>>
          主題名稱
      --background <Vec<bool>>
          是否啟用背景 [possible values: true, false]
      --custom-syntax-set <Vec<"/path/to/syntaxset-file">>
          自定義語法集檔案
      --custom-theme-set <Vec<"/path/to/themeset-file">>
          自定義主題集檔案

HighlightValue Debug:
      --list-all-syntaxes  顯示所有語法名稱及其副檔名
      --list-all-themes    顯示所有主題名稱

Debug:
      --display-config-dir  顯示glossa的配置目錄
```

### 引數型別說明

- enum `(a | b | c | ...)`
  - 透過單個引數值來指定列舉變體。
  - 如：`--map-type <regular|highlight|dsl|_>`
    - `--map-type regular` => `MapType::Regular`
    - `--map-type highlight` => `MapType::Highlight`
    - `--map-type dsl` => `MapType::DSL`
    - `--map-type _` => `MapType::RegularAndHighlight`

- 逗號分隔的引數 (a,b,c,..)
  - 引數型別為 `a,b,c,..`，表示引數可以接受多個值，以逗號分隔。
    - 如： `--exclude-languages <lang1,lang2,..>  [alias: --ex-langs]`
      - 對於 `--ex-langs ar,gsw,en-GB`，程式內部會將其收集為 `["ar", "gsw", "en-GB"]`。
- `Vec<T>`
  - 引數型別為 `Vec<T>`, 表示引數可以接受多個值，但需要多次呼叫相關選項。
    - 對於 `--true-color <Vec<bool>>`, 我們需要多次呼叫 `--true-color`, e.g., `--true-color true --true-color false`，程式內部會將其收集為 `[true, false]`。
- `HashMap<K, V>`
  - 對於 HighlightKey 與 HighlightValue 相關的選項，儘管其型別皆為 `Vec<T>`， 但實際上程式內部會自動將其收集為 `ahash::HashMap<K, V>`。
    - 注：Key 的相關選項的呼叫次數可以比 Value 更多，當 Value 的相關選項不存在時，程式會自動使用預設值。

## Quick Start

glossa-cli 是一個本地化資源生成器，它可以將本地化資源轉換為 Rust 程式碼或者是 bincode。

> 所謂“巧婦難為無米之炊”，若要以米成炊，必先備足薪米之資。

使用 glossa-cli 的前提條件是：先準備好資原始檔。

如果沒有本地化資源的話，我們可以直接用 glossa 的 git 倉庫。

> 注：glossa-cli 自身使用了自身 (glossa-cli) 來生成本地化資源的程式碼。

POSIX-sh:

```sh
# 進入臨時目錄
cd `mktemp -d`

# 直接下載壓縮包
curl -LO https://github.com/2moe/glossa/archive/refs/heads/dev.tar.gz

# 解壓
tar -xf dev.tar.gz

# 進入 glossa-dev 目錄
cd glossa-dev
```

先來個簡單的測試

```sh
glossa-cli -i ./locales --output-ron
```

> 由於 -i/--input 的預設引數是 locales，因此我們可以省略

我們可以將所有 `--output-` 開頭的選項都跑一遍，以瞭解其作用。

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

當無需處理所有 maps 及語言時，`--include-languages` 與`--include-map-names` 引數即可精準施策。

按需配置處理範圍，構建最小化的資源集合。

```sh
glossa-cli --output-match-fn-all-in-one-without-map-name --include-languages zh,zh-Hant,en --include-map-names cli
```

```sh
glossa-cli --output-match-fn-all-in-one --include-languages en,es --include-map-names yes-no,error
```

## 環境變數

- GLOSSA_LANG
  - 用於指定 glossa_cli 自身所使用的語言，e.g., `env GLOSSA_LANG=gsw glossa-cli`
  - 預設會根據當前作業系統的語言來自動設定 glossa-cli 自身的語言
    - 對於 wasi-p1 環境，由於無法自動檢測，因此需要手動指定。
- GLOSSA_L10N_DIR
  - 用於指定 glossa_cli 自身所使用的本地化資源目錄，e.g., `env GLOSSA_L10N_DIR=/path/to/bincode glossa-cli`
  - 若其值為空，則會使用 `$GLOSSA_CFG_DIR/bincode` 目錄
- GLOSSA_CFG_DIR
  - 指定 glossa_cli 自身的配置目錄，e.g., `env GLOSSA_CFG_DIR=/path/to/cfg_dir glossa-cli`
  - 預設會根據不同作業系統的規範，來自動設定。
    - macOS: `"/Users/[username]/Library/Application Support/me.tmoe.glossa"`
    - Linux: `"${XDG_CONFIG_HOME:-$HOME/.config}/glossa"`
    - Windows: `"C:\Users\[username]\AppData\Local\tmoe\glossa\config"`
- GLOSSA_LOG
  - 日誌輸出級別
  - 預設為 info
  - 可選： trace, debug, info, warn, error
