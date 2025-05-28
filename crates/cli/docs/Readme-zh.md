# glossa-cli

[![glossa-cli.crate](https://img.shields.io/crates/v/glossa-cli.svg?logo=rust&logoColor=lightsalmon&label=glossa-cli)](https://crates.io/crates/glossa-cli)

[![Documentation](https://docs.rs/glossa-cli/badge.svg)](https://docs.rs/glossa-cli)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-cli.svg?logo=apache)](../License)

<!-- Language -->

<details open>
<summary>
<img alt="Language/语言" src="./svg/language.svg" />
</summary>

- [zh-Hant: 繁體中文](Readme-zh-Hant.md)
- [en: English](Readme.md)
- [zh: 简体中文](Readme-zh.md)

</details>

<!-- TOC -->

<details open>
<summary>
<img alt="目录" src="./svg/toc/目录.svg"/>
</summary>

- [下载与安装](#下载与安装)
- [Overview](#overview)
  - [参数类型说明](#参数类型说明)
- [Quick Start](#quick-start)
- [环境变量](#环境变量)

</details>

---

## 下载与安装

TODO

## Overview

glossa-cli 本质上是 glossa-codegen 的包装器，如需了解更多内容，则请阅读 glossa-codegen 的文档。

运行 `glossa-cli -h`，我们可以得到如下内容：

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
          本地化资源的源目录 [default: locales]
      --dsl-suffix <String>
          DSL 文件后缀(默认 ".dsl")
      --include-languages <en,zh,fr,ru,ar,es,etc.>
          白名单模式。当其不为空时，只有位于列表中的语言 id 才会被初始化 [aliases: --langs]
      --include-map-names <name1,name2,..>
          当其不为空时，只有位于列表中的 map_names 才会被初始化 [aliases: --maps]
      --exclude-languages <lang1,lang2,..>
          黑名单模式。位于黑名单中的语言 id 不会被初始化 [aliases: --ex-langs]
      --exclude-map-names <name1,name2,..>
          位于列表中的 map_names 不会被初始化 [aliases: --ex-maps]

Generator Core:
  -o, --outdir </path/to/output_dir>                           输出的目录
      --visibility <pub| pub(crate)| pub(super)| private>      生成的代码的可见性 [aliases: --vis]
      --mod-visibility <pub| pub(crate)| pub(super)| private>  [aliases: --mod-vis]
  -b, --bincode-suffix <String>                                bincode文件后缀
  -m, --mod-prefix <String>                                    mod 文件前缀 (默认 "l10n_")
  -f, --feature-prefix <String>

Output Files:
      --output-bincode
          为不同的语言生成独立的 bincode 文件
      --output-match-fn
          为不同的语言生成独立的 rust 代码的文件 (包含match表达式的函数)
      --output-match-fn-without-map-name
          类似于 output_match_fn，但生成的函数只以 map_key 作为 key, 不包含 map_name
      --output-phf
          为不同语言生成独立的 phf map 函数
      --output-phf-without-map-name
          类似于 output_phf, 但生成的函数的key 为普通字符串，而不是 TupleKey

Output File:
      --output-bincode-all-in-one
          将所有语言的 bincode 输出到同一个文件
      --output-match-fn-all-in-one
          将所有语言的数据都输出为一个match函数（字符串）
      --output-match-fn-all-in-one-by-language
          将所有语言的数据输出为同一个match 函数（字符串），key 为语言名
      --output-match-fn-all-in-one-without-map-name
          将所有语言的数据输出为同一个 match 函数（字符串），key 为语言名和 map_key
      --output-phf-all-in-one
          将所有语言的 phf map 输出到同一个函数

Output String:
      --output-ron                                    输出为 ron 格式的字符串
      --output-locales-fn                             输出 all_locales 函数
      --output-raw-locales
      --output-mod-rs
      --output-cargo-features
      --output-router-for-match-fns
      --output-router-for-match-fns-without-map-name
      --output-router-for-phf-maps
      --output-router-for-phf-maps-without-map-name

HighlightKey:
      --base-name <Vec<String>>  基础"高亮Map"的名称
      --suffix <Vec<String>>     新生成的"高亮Map"的后缀

HighlightValue:
      --true-color <Vec<bool>>
          24位真彩色 [possible values: true, false]
      --syntax-name <Vec<String>>
          语法名称
      --theme-name <Vec<String>>
          主题名称
      --background <Vec<bool>>
          是否启用背景 [possible values: true, false]
      --custom-syntax-set <Vec<"/path/to/syntaxset-file">>
          自定义语法集文件
      --custom-theme-set <Vec<"/path/to/themeset-file">>
          自定义主题集文件

HighlightValue Debug:
      --list-all-syntaxes  显示所有语法名称及其扩展名
      --list-all-themes    显示所有主题名称

Debug:
      --display-config-dir  显示glossa的配置目录
```

### 参数类型说明

- enum `(a | b | c | ...)`
  - 通过单个参数值来指定枚举变体。
  - 如：`--map-type <regular|highlight|dsl|_>`
    - `--map-type regular` => `MapType::Regular`
    - `--map-type highlight` => `MapType::Highlight`
    - `--map-type dsl` => `MapType::DSL`
    - `--map-type _` => `MapType::RegularAndHighlight`

- 逗号分隔的参数 (a,b,c,..)
  - 参数类型为 `a,b,c,..`，表示参数可以接受多个值，以逗号分隔。
    - 如： `--exclude-languages <lang1,lang2,..>  [alias: --ex-langs]`
      - 对于 `--ex-langs ar,gsw,en-GB`，程序内部会将其收集为 `["ar", "gsw", "en-GB"]`。
- `Vec<T>`
  - 参数类型为 `Vec<T>`, 表示参数可以接受多个值，但需要多次调用相关选项。
    - 对于 `--true-color <Vec<bool>>`, 我们需要多次调用 `--true-color`, e.g., `--true-color true --true-color false`，程序内部会将其收集为 `[true, false]`。
- `HashMap<K, V>`
  - 对于 HighlightKey 与 HighlightValue 相关的选项，尽管其类型皆为 `Vec<T>`， 但实际上程序内部会自动将其收集为 `ahash::HashMap<K, V>`。
    - 注：Key 的相关选项的调用次数可以比 Value 更多，当 Value 的相关选项不存在时，程序会自动使用默认值。

## Quick Start

glossa-cli 是一个本地化资源生成器，它可以将本地化资源转换为 Rust 代码或者是 bincode。

> 所谓“巧妇难为无米之炊”，若要以米成炊，必先备足薪米之资。

使用 glossa-cli 的前提条件是：先准备好资源文件。

如果没有本地化资源的话，我们可以直接用 glossa 的 git 仓库。

> 注：glossa-cli 自身使用了自身 (glossa-cli) 来生成本地化资源的代码。

POSIX-sh:

```sh
# 进入临时目录
cd `mktemp -d`

# 直接下载压缩包
curl -LO https://github.com/2moe/glossa/archive/refs/heads/dev.tar.gz

# 解压
tar -xf dev.tar.gz

# 进入 glossa-dev 目录
cd glossa-dev
```

先来个简单的测试

```sh
glossa-cli -i ./locales --output-ron
```

> 由于 -i/--input 的默认参数是 locales，因此我们可以省略

我们可以将所有 `--output-` 开头的选项都跑一遍，以了解其作用。

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

当无需处理所有 maps 及语言时，`--include-languages` 与`--include-map-names` 参数即可精准施策。

按需配置处理范围，构建最小化的资源集合。

```sh
glossa-cli --output-match-fn-all-in-one-without-map-name --include-languages zh,zh-Hant,en --include-map-names cli
```

```sh
glossa-cli --output-match-fn-all-in-one --include-languages en,es --include-map-names yes-no,error
```

## 环境变量

- GLOSSA_LANG
  - 用于指定 glossa_cli 自身所使用的语言，e.g., `env GLOSSA_LANG=gsw glossa-cli`
  - 默认会根据当前操作系统的语言来自动设定 glossa-cli 自身的语言
    - 对于 wasi-p1 环境，由于无法自动检测，因此需要手动指定。
- GLOSSA_L10N_DIR
  - 用于指定 glossa_cli 自身所使用的本地化资源目录，e.g., `env GLOSSA_L10N_DIR=/path/to/bincode glossa-cli`
  - 若其值为空，则会使用 `$GLOSSA_CFG_DIR/bincode` 目录
- GLOSSA_CFG_DIR
  - 指定 glossa_cli 自身的配置目录，e.g., `env GLOSSA_CFG_DIR=/path/to/cfg_dir glossa-cli`
  - 默认会根据不同操作系统的规范，来自动设置。
    - macOS: `"/Users/[username]/Library/Application Support/me.tmoe.glossa"`
    - Linux: `"${XDG_CONFIG_HOME:-$HOME/.config}/glossa"`
    - Windows: `"C:\Users\[username]\AppData\Local\tmoe\glossa\config"`
    - WASI: `tmp/me.tmoe.glossa/config`
- GLOSSA_LOG
  - 日志输出级别
  - 默认为 info
  - 可选： trace, debug, info, warn, error
