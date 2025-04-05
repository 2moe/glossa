# glossa-codegen

[![glossa-codegen.crate](https://img.shields.io/crates/v/glossa-codegen.svg?logo=rust&logoColor=lightsalmon&label=glossa-codegen)](https://crates.io/crates/glossa-codegen)

[![Documentation](https://docs.rs/glossa-codegen/badge.svg)](https://docs.rs/glossa-codegen)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-codegen.svg?logo=apache)](./License)

<!-- https://img.shields.io/badge/Language/%E8%AA%9E%E8%A8%80-7D4698?logo=googletranslate&logoColor=white -->

<details open>
<summary>
<img alt="Language/语言" src="./svg/language.svg" />
</summary>

- [繁體中文](Readme-zh-Hant.md)
- [English](Readme.md)
- [简体中文](Readme-zh.md)

</details>

<!-- https://img.shields.io/badge/目录-2CA5E0.svg?logo=readme&logoColor=white -->
<details open>
<summary>
<img alt="目录" src="./svg/toc/目录.svg"/>
</summary>

- [基本概念](#基本概念)
  - [语言 id 与 map\_name](#语言-id-与-map_name)
  - [L10n 数据](#l10n-数据)
  - [Raw L10n 文本语法](#raw-l10n-文本语法)
    - [常规 K-V pairs](#常规-k-v-pairs)
    - [glossa-DSL](#glossa-dsl)
      - [1. 最基本的 **key = "value"**](#1-最基本的-key--value)
      - [2. 引用](#2-引用)
      - [3. 外部传入的参数](#3-外部传入的参数)
        - [`{ 🐱 }` 与 `{ $🐱 }` 的区别](#---与----的区别)
      - [4. 选择器（条件控制语法）](#4-选择器条件控制语法)
      - [5. 转义语法](#5-转义语法)
  - [MapType](#maptype)
- [L10nResources (本地化资源)](#l10nresources-本地化资源)
- [Generator (生成器)](#generator-生成器)
  - [构造 Generator](#构造-generator)
  - [输出](#输出)
    - [生成代码: 包含 match-expr 的 const 函数](#生成代码-包含-match-expr-的-const-函数)
      - [**output\_match\_fn()**](#output_match_fn)
      - [**output\_match\_fn\_all\_in\_one()**](#output_match_fn_all_in_one)
      - [**output\_match\_fn\_all\_in\_one\_by\_language\_and\_key()**](#output_match_fn_all_in_one_by_language_and_key)
    - [生成代码: 包含 phf map 的 const 函数](#生成代码-包含-phf-map-的-const-函数)
      - [**output\_phf()**](#output_phf)
      - [**output\_phf\_all\_in\_one()**](#output_phf_all_in_one)
    - [bincode](#bincode)
      - [**output\_bincode()**](#output_bincode)
- [高级用法](#高级用法)
  - [语法高亮](#语法高亮)
    - [数据结构](#数据结构)
    - [剖析](#剖析)
      - [Key](#key)
      - [Value](#value)
    - [Example](#example)

</details>

---

glossa-codegen 能够用来生成 (包含本地化文本的) rust 代码，以及 bincode。

> 注：尽管 glossa-codegen 需要 std，但是 glossa 和 glossa-shared 都支持 no-std 环境。
>
> - glossa-codegen 用于生成**正式**代码。
> - glossa 用于生成 fallback chain。
> - glossa-shared 提供**正式**代码所需的各种数据类型。
>
> 您只需要在 `#[test]` 测试代码或 `build.rs` 中引入 glossa-codegen，并不需要在正式代码中引入。

## 基本概念

### 语言 id 与 map_name

假设存在 locales 目录，其结构如下所示。

```plaintext
locales
  ├── ar
  │   └── error.yaml
  ├── en
  │   ├── error.yaml
  │   └── yes-no.toml
  ├── es
  │   └── yes-no.toml
  ├── fr
  │   └── yes-no.toml
  ├── ru
  │   └── error.yaml
  └── zh
      ├── error.yaml
      └── yes-no.toml
```

其中， `"ar", "en", "es", "fr", "ru", "zh"` 为 **语言 ID**。

"error" 和 "yes-no" 为 map 名称。

> 不同 file_stem (e.g., a.toml, b.json) 对应不同的 map 名称。
>
> 那么相同的呢？(e.g., a.toml, a.json)

Q：假设同时存在 "error.yaml"、 "error.yml"、 "error.toml"、 "error.json5"、 "error.json"、 "error.ron"，那么哪一个才是真正的 "error" map 呢？

A:
  如果所有文件内容都是有效且不为空的 K-V String Pairs，那么靠运气！
  否则的话，第一个**有效**的“同 stem” 文件将成为真正的 map。

> 注：a.toml => a 与 a.dsl.toml => a.dsl 不是同 stem 文件。
>
> en/a.toml => a 与 en/subdir/a.json => a 是同 stem 文件

🍀🍀🍀

Q：为什么要靠运气呢？

A:
  因为在初始化本地化资源的时候，会调用 rayon 来进行多线程反序列化（多个文件多个线程同时读取并解析）。
  其中的顺序并不是有序的。

### L10n 数据

| L10n 类型          | 描述                                   |
| ------------------ | -------------------------------------- |
| Raw 文本文件       | 未经处理的原始文件，比如 en/hello.toml |
| 转换出来的rust代码 | 支持 const fn，直接硬编码到程序中      |
| bincode            | 支持高效反序列化的二进制文件           |

我们可以简单将 Raw 文件理解为源代码，其他东西都是用 Raw 文件编译出来的。

### Raw L10n 文本语法

#### 常规 K-V pairs

这是最基本的类型。

以 toml 为例：

`key = "value"`，其中 key 和 value 都为字符串。

```toml
hello = "你好"
"🐱" = "喵 ฅ(°ω°ฅ)"
```

以 json5 为例：

```json
{
  // json5 可以用注释
  "hello": "你好",
  "🐱": "喵 ฅ(°ω°ฅ)", /* 可以尾随逗号 "," */
}
```

#### glossa-DSL

[![glossa-dsl.crate](https://img.shields.io/crates/v/glossa-dsl.svg?logo=rust&logoColor=lightsalmon&label=glossa-dsl)](https://github.com/2moe/glossa-dsl)

> DSL: 领域特定语言

我们可以在 5 分钟内，掌握 glossa-dsl 的 5 种语法。

##### 1. 最基本的 **key = "value"**

- toml: **name = "Tom"**
- json: `{"name": "Tom"}`

##### 2. 引用

toml:

```toml
name = "Tom"
hello = "Hello { name }"
```

①. hello 引用了 `{ name }` （注：`{ name }` 与 `{name}` 本质上是一样的）
②. 展开 hello
③. `"Hello {name}"` =>  `"Hello Tom"`

rust:

```rust
let text = res.try_get("hello")?;
assert_eq!(text, "Hello Tom");
```

---

json5:

```json
{
  "hello": "Hello {🐱}",
  "🐱": "ฅ(°ω°ฅ)",
}
```

①. `hello` 引用了 `{🐱}`
②. 展开 `hello`
③. 得到了 `"Hello ฅ(°ω°ฅ)"`.

rust:

```rust
let text = res.try_get("hello")?;
assert_eq!(text, "Hello ฅ(°ω°ฅ)");
```

##### 3. 外部传入的参数

toml:

```toml
"打招呼" = "早安喵 { $🐱 }"
greeting = "{ 打招呼 }，{ $name }！"
```

> `{ $🐱 }` 和 `{ $name }` 依赖外部传入的参数

rust:

```rust
let ctx = [("name", "Moe"), ("🐱", "ฅ(°ω°ฅ)")];

let text = res.get_with_context("greeting", &ctx)?;
assert_eq!(text, "早安喵 ฅ(°ω°ฅ)，Moe！");
```

---

###### `{ 🐱 }` 与 `{ $🐱 }` 的区别

重点是有没有加 `$`，加了 `$` 就依赖于外部参数，没加就是内部引用。

内部引用：

```toml
"🐱" = "ฅ(°ω°ฅ)"
meow = "{ 🐱 }"
```

依赖外部传入参数:

```toml
meow = "{ $🐱 }"
```

##### 4. 选择器（条件控制语法）

zh/unread.toml:

```toml
"阿拉伯数字转汉字" = """
  $num ->
    [0] 〇
    [1] 一
    [2] 二
    [3] 三
    [10] 十
    *[其他] {$num}
"""

"未读msg" = "未读消息"

"显示未读消息数量" = """
  $num ->
      [0] 没有{ 未读msg }
      [2] 您有两条{ 未读msg }
     *[其他] 您有{ 阿拉伯数字转汉字 }条{ 未读msg }
"""

show-unread-messages-count = "{显示未读消息数量}。"
```

rust:

```rust
let get_text = |num_str| res.get_with_context("show-unread-messages-count", &[("num", num_str)]);

assert_eq!(get_text("0")?, "没有未读消息。");
assert_eq!(get_text("1")?, "您有一条未读消息。");
assert_eq!(get_text("2")?, "您有两条未读消息。");
assert_eq!(get_text("10")?, "您有十条未读消息。");
assert_eq!(get_text("100")?, "您有100条未读消息。");
```

我们可以将 "显示未读消息数量" 理解为一个函数， `$num` 理解为函数的参数。

若将上文的 toml 文本理解为 rust 代码，则其会是如此：

```rust
let 未读msg = "未读消息";
let 显示未读消息数量 = |num| match num {
  "0" => fmt!("没有{未读msg}"),
  "2" => fmt!("您有两条{未读msg}"),
  _ => fmt!("您有{n}条{未读msg}", n = 阿拉伯数字转汉字(num)),
};
```

---

en/unread.toml:

```toml
num-to-en = """
  $num ->
    [0] zero
    [1] one
    [2] two
    [3] three
    *[other] {$num}
"""

unread_msg = "unread message"

unread-count = """
  $num ->
    [0] No {unread_msg}s.
    [1] You have { num-to-en } {unread_msg}.
    *[other] You have { num-to-en } {unread_msg}s.
"""

show-unread-messages-count = "{unread-count}"
```

rust:

```rust
let get_text = |num_str| res.get_with_context("show-unread-messages-count", &[("num", num_str)]);

assert_eq!(get_text("0")?, "No unread messages.");
assert_eq!(get_text("1")?, "You have one unread message.");
assert_eq!(get_text("2")?, "You have two unread messages.");
assert_eq!(get_text("100")?, "You have 100 unread messages.");
```

##### 5. 转义语法

在上文中，我们了解到 `{ a }` 就是内部引用，而 `{ $a }` 依赖于外部传入的 `a` 参数。

Q：如果需要得到使用原始的 `{a  }`，避免其自动解析，那该怎么办呢？

A：使用转义语法。

- 如果需要得到原始的 `{a  }`，那么外部至少需要包裹两层 `{}`, 也就是 `{{  {a  }   }}`。
- 如果需要得到原始的 `{{a  }`，那么外部至少需要包裹三层 `{}`, 也就是 `{{{  {{a  }     }}}`。

---

- `"{{ a   }}"` => `"a"`
- `"{{{a}}}"` => `"a"`
- `"{{{{  a  }}}}"` => `"a"`
- `"{{    {a}    }}"` => `"{a}"`
- `"{{a}"` => ❌ nom Error, code: take_until
- `"{{{    {{a}}    }}}"` => `"{{a}}"`
- `"{{{    {{ a }}    }}}"` => `"{{ a }}"`
- `"{{{ {{a} }}}"` => `"{{a}"`

### MapType

```rust
enum MapType {
  Regular,
  Highlight,
  RegularAndHighlight,
  DSL,
}
```

- Regular：K-V pairs
- Highlight：带有语法高亮的 K-V pairs
- RegularAndHighlight： 融合了 Regular 和 Highlight。
- DSL：glossa-DSL。由于 MapType 一般配合 `.output_*` 使用，因此当 MapType 为 DSL 时，输出的Map为 glossa-DSL 的 AST，而不是 Raw glossa-DSL。

> AST：抽象语法树

从本质上来上说，Regular 与 Highlight 使用相同的数据结构。
之所以将它们分开，是为了更“细粒度”的控制。

## L10nResources (本地化资源)

```rust
pub struct SmallList<const N: usize>(pub SmallVec<MiniStr, N>);

pub struct L10nResources {
  dir: PathBuf,
  dsl_suffix: MiniStr,

  include_languages: SmallList<3>,
  include_map_names: SmallList<2>,

  exclude_languages: SmallList<1>,
  exclude_map_names: SmallList<1>,

  /// get data: [Self::get_or_init_data]
  lazy_data: OnceLock<L10nResMap>,
}
```

- dir： 本地化资源所在的目录，例如 "./locales"
- dsl_suffix
  - glossa-DSL 文件的后缀，默认为 ".dsl"
    - 当其值为 ".dsl" 时
      - "a.dsl.toml" 会被识别为 **glossa-DSL** 文件
      - "b.dsl.json" 也会被识别为 **glossa-DSL** 文件
      - "a.toml" 为常规文件
- include_languages
  - 白名单模式，当其不为空时，只有位于列表中的语言 id 才会被初始化
    - 假设所有语言 id 为: "de", "en", "es", "pt", "ru", "zh"
    - `.with_include_language(["en", "zh"])` => 只有 "en" 和 "zh" 的本地化资源才会被初始化
- include_map_names
  - 当其不为空时，只有位于列表中的 map_names 才会被初始化。
    - 假设存在: "en/a.toml", "en/b.json", "zh/a.json", "zh/b.ron"
    - 不难看出，所有 map_names 为 `["a", "b"]`
    - `.with_include_map_names(["a"])` => 只有 "en/a.toml" 和 "zh/a.json" 会被初始化
- exclude_languages
  - 黑名单模式。位于黑名单中的语言 id 不会被初始化
    - 假设存在: "de", "en", "es", "pt", "ru", "zh"
      - `.with_exclude_languages(["en", "es", "ru"])` => `["de", "pt", "zh"]`
      - `.with_include_languages(["en", "es"]).with_exclude_languages(["en"])` => `["es"]`
- exclude_map_names
  - 位于列表中的 map_names 不会被初始化
  - 假设存在:
    - "en/a.toml"
    - "en/b.json"
    - "zh/a.json"
    - "zh/b.ron"
    - "zh/c.toml"
  - `.with_exclude_map_names(["a"])` => "en/b.json", "zh/b.ron", "zh/c.toml"
  - `.with_include_map_names(["b", "c"]).with_exclude_map_names(["b"])` => "zh/c.toml"
  - `.with_include_language(["en"]).with_exclude_map_names(["a"])` => "en/b.json"
- lazy_data
  - 在运行期间**延迟**初始化的数据
  - 通过 `.get_or_init_data()` 来获取数据，相当于缓存

| 方法                                     | 描述                                             |
| ---------------------------------------- | ------------------------------------------------ |
| `.get_dir()`                             | 获取 dir                                         |
| `.with_dir("/path/to/new_dir".into())`   | 设置 dir                                         |
| `.get_dsl_suffix()`                      | 获取 dsl_suffix                                  |
| `.with_dsl_suffix(".new_suffix".into())` | 设置 dsl_suffix                                  |
| `.with_include_languages([])`            | 设置 include_languages                           |
| `.with_include_map_names([])`            | 设置 include_map_names                           |
| `.with_exclude_languages([])`            | 设置 exclude_languages                           |
| `.with_exclude_map_names([])`            | 设置 exclude_map_names                           |
| `.get_or_init_data()`                    | 获取 `&HashMap<KString, Vec<L10nMapEntry>>`      |
| `.with_lazy_data(OnceLock::new())`       | 设置 lazy_data，可以将OnceLock重置为未初始化状态 |

Q: 如何构造一个新的 L10nResources 结构体呢？

A：

```rust
use glossa_codegen::L10nResources;
let _res = L10nResources::new("locales");
// 相当于 L10nResources::default().with_dir("locales".into())
```

"locales" 可以改成其他目录，比如 "../../l10n/"

## Generator (生成器)

```rust
pub struct Generator<'h> {
  resources: Box<L10nResources>,

  visibility: Visibility,

  outdir: Option<PathBuf>,

  bincode_suffix: MiniStr,
  mod_prefix: MiniStr,

  highlight: Option<Box<HighlightCfgMap<'h>>>,

  /// get: `Self::get_or_init_*maps`
  lazy_maps: Box<LazyMaps>,
}
```

- resources: 本地化资源
- visibility
  - 生成的 rust 代码的可见性, 默认为 PubCrate
    - > `glossa_codegen::Visibility { Private, PubCrate, Pub, PubSuper }`
  - `.with_visibility(Visibility::Pub)` => `pub const fn xxx`
  - `.with_visibility(Visibility::PubCrate)` => `pub(crate) const fn xxx`
- outdir
  - 输出 rust 代码以及 bincode 的目录
- bincode_suffix: bincode文件后缀，默认为 ".bincode"
- mod_prefix
  - 生成的 rust 代码的模块前缀，默认为 "l10n_"
- highlight: 语法高亮的配置，这个稍微有点复杂，我们将会在高级用法中提到。
- lazy_maps
  - 延迟初始化的maps
  - 相关方法：
    - `.get_or_init_maps()`  // Regular
    - `.get_or_init_highlight_maps()` // Highlight
    - `.get_or_init_merged_maps()` // RegularAndHighlight
    - `.get_or_init_dsl_maps()` // Template

### 构造 Generator

```rust
use glossa_codegen::{Generator, L10nResources};

let resources = L10nResources::new("locales");

let generator = Generator::default()
  .with_resources(resources)
  .with_outdir("tmp");
```

### 输出

- 内部是 match 表达式的 const 函数
  - 调用 Generator 的 `.output_match_fn(MapType::Regular)` 会生成 rust 代码
    - `const fn map(map_name: &[u8], key: &[u8]) -> &'static str { match (map_name, key) {...} }`
- phf map 函数
  - 调用 Generator 的 `.output_phf(MapType::Regular)` 会生成 rust 代码
    - `const fn map() -> super::PhfL10nOrderedMap { ... }`
- bincode
  - 调用 Generator 的 `.output_bincode(MapType::Regular)` 会生成 bincode 二进制文件

MapType::DSL 只能输出为 bincode，而其他 MapType 支持所有的输出类型。

> 您可以将 DSL 指定为 Regular Map（可能需要修改 L10nResources 的 dsl_suffix），不过这样做并不会带来性能优势。因为解析 DSL 的 AST 要比解析 Raw DSL 更快。
>
> 当将 DSL 指定为 Regular 时，生成的代码是 Raw K-V pairs。在运行期间需要先将其解析为 AST，再进行处理。
>
> 而若将 MapType::DSL 直接输出为 bincode，那输出的结果就是 DSL 的 AST 的 bincode，而不是 Raw K-V pairs。

#### 生成代码: 包含 match-expr 的 const 函数

相关方法有：

- `.output_match_fn()`
  - 为不同的语言生成独立的 rust 代码文件
  - => `{outdir}/{mod_prefix}{snake_case_language}.rs`
    - 比如
      - en => tmp/l10n_en.rs
      - en-GB => tmp/l10n_en_gb.rs
- `.output_match_fn_all_in_one()`
  - 将所有语言的本地化资源都收集为一个字符串
    - 其内容为 `const fn map(lang: &[u8], map_name:&[u8], key:&[u8]) -> &'static str {...}`
- `.output_match_fn_all_in_one_by_language()`
  - 将所有语言的本地化资源都收集为一个字符串
    - 其内容为 `const fn map(language: &[u8]) -> &'static str {...}`
    - 只有当 map_name 和 key 都只有唯一一个时，您才能使用此函数，否则 map_name 和 key 会出现冲突。
- `.output_match_fn_all_in_one_by_language_and_key()`
  - 将所有语言的本地化资源都收集为一个字符串
    - 其内容为 `const fn map(language: &[u8], key: &[u8]) -> &'static str {...}`
    - 只有当 map_name 只有唯一一个时，您才能使用此函数，否则 key 会出现冲突。

##### **output_match_fn()**

假设存在如下两个文件：

l10n/en-GB/error.toml

```toml
text-not-found = "No localised text found"
```

l10n/de/error.yml

```yaml
text-not-found: Kein lokalisierter Text gefunden
```

我们可以调用 `.output_match_fn(Regular)` 来生成常规类型的 Map 的代码。

```rust
use glossa_codegen::{generator::MapType, Generator, L10nResources};

let resources = L10nResources::new("l10n");

Generator::default()
  .with_resources(resources)
  .with_outdir("tmp")
  .output_match_fn(MapType::Regular)?;
```

输出结果:

tmp/l10n_en_gb.rs

```rust
pub(crate) const fn map(map_name: &[u8], key: &[u8]) -> &'static str {
  match (map_name, key) {
    (b"error", b"text-not-found") => r#####"No localised text found"#####,
    _ => "",
  }
}
```

tmp/l10n_de.rs

```rust
pub(crate) const fn map(map_name: &[u8], key: &[u8]) -> &'static str {
  match (map_name, key) {
    (b"error", b"text-not-found") => r#####"Kein lokalisierter Text gefunden"#####,
    _ => "",
  }
}
```

##### **output_match_fn_all_in_one()**

Q: 我们如果使用 `output_match_fn_all_in_one()` ，那么会得到什么呢？
A: 会得到一个包含函数数据的 String。

> 所有语言的本地化资源都在同一个函数中

```rust
let function_data = generator.output_match_fn_all_in_one(MapType::Regular)?;
```

function_data:

```rust
pub(crate) const fn map(lang: &[u8], map_name: &[u8], key: &[u8]) -> &'static str {
  match (lang, map_name, key) {
    (b"en-GB", b"error", b"text-not-found") => r#####"Kein lokalisierter Text gefunden"#####,
    (b"de", b"error", b"text-not-found") => r#####"Kein lokalisierter Text gefunden"#####,
    _ => "",
  }
}
```

##### **output_match_fn_all_in_one_by_language_and_key()**

当 map_name 只有唯一一个时，我们可以省略它，以此来达到性能优化的目的。

```rust
match (lang, key) { ... }
```

```rust
match (lang, map_name, key) { ... }
```

将两段 match 表达式进行对比：由于前者少匹配了一个项，所以从理论上来说，前者会更快。

`output_match_fn_all_in_one_by_language_and_key()` 会生成类似于前者的代码。

您如果不关心纳秒级别的性能优化，那么完全不用在意这一小节的内容。

---

举个例子：

- `en/yes-no { yes: "Yes", no: "No"}`
- `de/yes-no { yes: "Ja", no: "Nein" }`

在本例中，唯一的 map_name 是 yes-no，因此我们可以省略它。

调用 `.output_match_fn_all_in_one_by_language_and_key(Regular)?` 会生成如下代码：

```rust
pub(crate) const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match (language, key) {
    (b"en", b"yes") => r#####"Yes"#####,
    (b"en", b"no") => r#####"No"#####,
    (b"de", b"yes") => r#####"Ja"#####,
    (b"de", b"no") => r#####"Nein"#####,
    _ => "",
  }
}
```

当 map_name 不是唯一时，比如: 新增一个 `en/yes-no2 { yes: "YES", no: "NO", ok: "OK"}`。

此时不同的 map_names 有相同的 keys ("yes", "no")，这会产生冲突，我们就不能省略 map_name 了。
在这种情况下，我们应该用 `output_match_fn_all_in_one()`。

#### 生成代码: 包含 phf map 的 const 函数

- `.output_phf()`
  - 为不同的语言生成独立的 rust 代码文件
- `.output_phf_all_in_one()`
  - 将所有语言的本地化资源都收集为一个包含 phf map 的函数数据的字符串

##### **output_phf()**

```rust
use glossa_codegen::{generator::MapType, Generator, L10nResources};

pub(crate) fn es_generator<'h>() -> Generator<'h> {
  let data = L10nResources::new("locales").with_include_languages(["es", "es-419"]);
  Generator::default().with_resources(data).with_outdir("tmp")
}

es_generator().output_phf(MapType::Regular)?;
```

tmp/l10n_es.rs

```rust
pub(crate) const fn map() -> super::PhfL10nOrderedMap {
  use super::PhfTupleKey as Key;
  super::phf::OrderedMap {
    key: 12913932095322966823,
    disps: &[(0, 0)],
    idxs: &[1, 3, 2, 4, 0],
    entries: &[
      (
        Key(r#"error"#, r##"text-not-found"##),
        r#####"No se encontró texto localizado"#####,
      ),
      (Key(r#"yes-no"#, r##"cancel"##), r#####"Cancelar"#####),
      (Key(r#"yes-no"#, r##"no"##), r#####"No"#####),
      (Key(r#"yes-no"#, r##"ok"##), r#####"Aceptar"#####),
      (Key(r#"yes-no"#, r##"yes"##), r#####"Sí"#####),
    ],
  }
}
```

Q：等等，PhfL10nOrderedMap 和 PhfTupleKey 都是哪来的？

A: [![glossa-shared.crate](https://img.shields.io/crates/v/glossa-shared.svg?logo=rust&logoColor=lightsalmon&label=glossa-shared)](https://crates.io/crates/glossa-shared) 里包含了相关的数据类型。

##### **output_phf_all_in_one()**

```rust
let data = L10nResources::new("locales")
   .with_include_languages(["de", "en", "fr", "pt", "zh"])
   .with_include_map_names(["yes-no"]);
let function_data = Generator::default().with_resources(data).output_phf_all_in_one(MapType::Regular)?;
```

function_data:

```rust
pub(crate) const fn map() -> super::PhfL10nAllInOneMap {
  use super::PhfTripleKey as Key;
  super::phf::OrderedMap {
    key: 6767243246500575252,
    disps: &[(0, 0), (0, 2), (4, 12), (15, 9)],
    idxs: &[
      4, 7, 13, 19, 9, 14, 3, 17, 10, 18, 5, 12, 16, 1, 8, 6, 2, 15, 0, 11,
    ],
    entries: &[
      (
        Key(r#"de"#, r##"yes-no"##, r###"cancel"###),
        r#####"Abbrechen"#####,
      ),
      (Key(r#"de"#, r##"yes-no"##, r###"no"###), r#####"Nein"#####),
      (Key(r#"de"#, r##"yes-no"##, r###"ok"###), r#####"OK"#####),
      (Key(r#"de"#, r##"yes-no"##, r###"yes"###), r#####"Ja"#####),
      (
        Key(r#"en"#, r##"yes-no"##, r###"cancel"###),
        r#####"Cancel"#####,
      ),
      (Key(r#"en"#, r##"yes-no"##, r###"no"###), r#####"No"#####),
      (Key(r#"en"#, r##"yes-no"##, r###"ok"###), r#####"OK"#####),
      (Key(r#"en"#, r##"yes-no"##, r###"yes"###), r#####"Yes"#####),
      (
        Key(r#"fr"#, r##"yes-no"##, r###"cancel"###),
        r#####"Annuler"#####,
      ),
      (Key(r#"fr"#, r##"yes-no"##, r###"no"###), r#####"Non"#####),
      (Key(r#"fr"#, r##"yes-no"##, r###"ok"###), r#####"OK"#####),
      (Key(r#"fr"#, r##"yes-no"##, r###"yes"###), r#####"Oui"#####),
      (
        Key(r#"pt"#, r##"yes-no"##, r###"cancel"###),
        r#####"Cancelar"#####,
      ),
      (Key(r#"pt"#, r##"yes-no"##, r###"no"###), r#####"Não"#####),
      (Key(r#"pt"#, r##"yes-no"##, r###"ok"###), r#####"OK"#####),
      (Key(r#"pt"#, r##"yes-no"##, r###"yes"###), r#####"Sim"#####),
      (
        Key(r#"zh"#, r##"yes-no"##, r###"cancel"###),
        r#####"取消"#####,
      ),
      (Key(r#"zh"#, r##"yes-no"##, r###"no"###), r#####"否"#####),
      (Key(r#"zh"#, r##"yes-no"##, r###"ok"###), r#####"确定"#####),
      (Key(r#"zh"#, r##"yes-no"##, r###"yes"###), r#####"是"#####),
    ],
  }
}
```

#### bincode

- `output_bincode()`
  - => `{outdir}/{language}{bincode_suffix}`
    - en => tmp/en{bincode_suffix} => tmp/en.bincode
    - en-GB => tmp/en-GB{bincode_suffix} => tmp/en-GB.bincode
- `output_bincode_all_in_one()`
  - 所有语言的 L10n 资源
  - => `{outdir}/all{bincode_suffix}`
    - => tmp/all{bincode_suffix} => tmp/all.bincode

##### **output_bincode()**

**../../locales/en/unread.dsl.toml**:

```toml
num-to-en = """
$num ->
  [0] zero
  [1] one
  [2] two
  [3] three
  *[other] {$num}
"""

unread = "unread message"

unread-count = """
$num ->
  [0] No {unread}s.
  [1] You have { num-to-en } {unread}.
  *[other] You have { num-to-en } {unread}s.
"""

show-unread-messages-count = "{unread-count}"
```

rust:

```rust
    use glossa_codegen::{L10nResources, Generator, generator::MapType};
    use glossa_shared::decode::file::decode_single_file_to_dsl_map;
    use std::path::Path;

    // -------------------
    // Encode

    let resources = crate::L10nResources::new("../../locales/");
    // Output to tmp/{language}_dsl.bincode
    Generator::default()
      .with_resources(resources)
      .with_outdir("tmp")
      .with_bincode_suffix("_dsl.bincode".into())
      .output_bincode(MapType::DSL)?;

    // ------------------
    // Decode

    let file = Path::new("tmp").join("en_dsl.bincode");
    let dsl_maps = decode_single_file_to_dsl_map(file)?;

    let unread_resolver = dsl_maps
      .get("unread")
      .expect("Failed to get AST (map_name: unread)");

    let get_text = |num_str| {
      unread_resolver
        .get_with_context("show-unread-messages-count", &[("num", num_str)])
    };

    let one = get_text("1")?;
    assert_eq!(one, "You have one unread message.");

    let zero = get_text("0")?;
    assert_eq!(zero, "No unread messages.");

    Ok(())
```

## 高级用法

### 语法高亮

[![hlight.crate](https://img.shields.io/crates/v/hlight.svg?logo=rust&logoColor=lightsalmon&label=hlight)](https://crates.io/crates/hlight)

---

glossa-codegen 支持将本地化文本渲染成包含语言高亮的内容，并转换为 rust 代码和 bincode。

Q: 为什么需要预先渲染呢？

A: 为了性能优化。
直接输出常量的 `&'static str` 会比在运行期间使用正则表达式进行语法高亮渲染快很多倍。

Q: 常量的语法高亮字符串有何用武之地？

A: 我们在开发 CLI 应用时，帮助信息可以使用常量的语法高亮字符串。
既保证了性能，又兼顾了可读性。

![highlight_sample](../../../assets/img/zh/highlight_help.png)

#### 数据结构

```rust
pub type HighlightCfgMap<'h> = HashMap<DerivedMapKey, SyntaxHighlightConfig<'h>>;

pub struct DerivedMapKey {
  /// map_name
  base_name: KString,
  /// map_suffix
  suffix: KString,
}

pub struct SyntaxHighlightConfig<'r> {
  resource: HighlightResource<'r>,
  syntax_name: MiniStr,
  true_color: bool,
}

pub struct HighlightResource<'theme> {
  theme_name: MiniStr,
  /// - get or init: [Self::get_or_init_theme]
  theme: OnceLock<&'theme Theme>,
  theme_set: &'theme ThemeSet,
  syntax_set: &'theme SyntaxSet,
  background: bool,
}
```

---

**基本用法:**

```rust
generator.with_highlight(
  HighlightCfgMap::default()
).output_bincode(MapType::Highlight)
```

> 因为我们还没有配置一个有效的 HighlightCfgMap，所以此代码无法正常运行！
>
> 别担心，只要将 `HighlightCfgMap::default()` 改成有效的数据，这段代码就能跑起来。

---

**核心概念:**

HighlightCfgMap 的作用是为多个 maps 应用不同的语法高亮配置。

**目录结构示例:**

```plaintext
en/
 ├── help-markdown.toml    // Base map: help-markdown
 └── a-zsh.toml           // Base map: a-zsh
```

**配置示例（伪代码）：**

```rust
<
  // help-markdown_monokai
  (DerivedMapKey {
    base_name: "help-markdown",
    suffix: "_monokai",
  },
  SyntaxHighlightConfig {
    resource: HighlightResource {
      theme_name: "Monokai Extended",
      background: true,
      ...
    },
    syntax_name: "md",
    true_color: true,
  }),
  // help-markdown_ayu
  (DerivedMapKey {
    base_name: "help-markdown",
    suffix: "_ayu",
  },
  SyntaxHighlightConfig {
    resource: HighlightResource {
      theme_name: "ayu-dark",
      background: false,
      ...
    },
    syntax_name: "md",
    true_color: false,
  }),
  // a-zsh_custom2
  (DerivedMapKey {
    base_name: "a-zsh",
    suffix: "_custom2",
  },
  SyntaxHighlightConfig {
    resource: HighlightResource {
      theme_set: custom_theme_set(),
      theme_name: "OneDark-pro vivid",
      background: false,
      ...
    },
    syntax_name: "sh",
    true_color: true,
  })
>
```

#### 剖析

##### Key

```rust
DerivedMapKey {
  base_name: "help-markdown",
  suffix: "_monokai",
}
```

base-name 会引用一个真实存在的常规 map，所以不能乱改名。
在上面的例子中，codegen 会在 "help-markdown" 的基础上应用语法高亮，然后生成一个新的 Map（map_name: "help-markdown_monokai"）。

我们可以自定义 suffix，但需要避免 `format!("{base_name}{suffix}")` 与 regular map 的名称冲突。

##### Value

```rust
struct SyntaxHighlightConfig<'r> {
  resource: HighlightResource<'r>,
  syntax_name: MiniStr,
  true_color: bool,
}
```

- resource
  - 我们可以自定义主题名称，主题集，语法集，配置是否启用背景
  - 详见 [hlight 的文档](https://docs.rs/hlight)
- syntax_name
  - 语法名称
  - 如果不支持相关语法的话，那么您需要配置 HighlightResource，载入自定义的语法集 (SyntaxSet)。
- true_color
  - 若其值为 true，则启用真彩色，否则使用古早的 256-color。
  - 开启与否主要看您的终端是否支持真彩色。
    - 在支持的终端上，开启 true_color 会让色彩更准确。
    - 在不支持的终端上，比如 macOS 15.3 的 Terminal.app(v2.14)，开启 true_color 会让色彩变得很奇怪。

#### Example

```rust
  fn new_highlight_map<'a>() -> HighlightCfgMap<'a> {
    let mut hmap = HighlightCfgMap::default();
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("md".into())
        .with_suffix("_md".into()),
      SyntaxHighlightConfig::default()
        .with_syntax_name("md".into())
        .with_true_color(false),
    );
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("md".into())
        .with_suffix("_md_ayu_dark".into()),
      SyntaxHighlightConfig::default()
        .with_resource(
          HighlightResource::default()
            .with_theme_name("ayu-light".into())
            .with_background(false),
        )
        .with_syntax_name("md".into()),
    );
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("t".into())
        .with_suffix("_toml".into()),
      SyntaxHighlightConfig::default().with_syntax_name("toml".into()),
    );
    hmap
  }

  let highlight_generator = Generator::default()
    .with_resources(L10nResources::new("locales"))
    .with_outdir("tmp")
    .with_highlight(new_highlight_map())
    .with_bincode_suffix(".highlight.bincode".into());

  highlight_generator.output_bincode_all_in_one(MapType::Highlight)
```
