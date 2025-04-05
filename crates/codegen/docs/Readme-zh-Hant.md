# glossa-codegen

[![glossa-codegen.crate](https://img.shields.io/crates/v/glossa-codegen.svg?logo=rust&logoColor=lightsalmon&label=glossa-codegen)](https://crates.io/crates/glossa-codegen)

[![Documentation](https://docs.rs/glossa-codegen/badge.svg)](https://docs.rs/glossa-codegen)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-codegen.svg?logo=apache)](./License)

<!-- https://img.shields.io/badge/Language/%E8%AA%9E%E8%A8%80-7D4698?logo=googletranslate&logoColor=white -->

<details open>
<summary>
<img alt="Language/語言" src="./svg/language.svg" />
</summary>

- [繁體中文](Readme-zh-Hant.md)
- [English](Readme.md)
- [簡體中文](Readme-zh.md)

</details>

<!-- https://img.shields.io/badge/目錄-2CA5E0.svg?logo=readme&logoColor=white -->
<details open>
<summary>
<img alt="目錄" src="./svg/toc/目錄.svg"/>
</summary>

- [基本概念](#基本概念)
  - [語言 id 與 map\_name](#語言-id-與-map_name)
  - [L10n 資料](#l10n-資料)
  - [Raw L10n 文字語法](#raw-l10n-文字語法)
    - [常規 K-V pairs](#常規-k-v-pairs)
    - [glossa-DSL](#glossa-dsl)
      - [1. 最基本的 **key = "value"**](#1-最基本的-key--value)
      - [2. 引用](#2-引用)
      - [3. 外部傳入的引數](#3-外部傳入的引數)
        - [`{ 🐱 }` 與 `{ $🐱 }` 的區別](#---與----的區別)
      - [4. 選擇器（條件控制語法）](#4-選擇器條件控制語法)
      - [5. 轉義語法](#5-轉義語法)
  - [MapType](#maptype)
- [L10nResources (本地化資源)](#l10nresources-本地化資源)
- [Generator (生成器)](#generator-生成器)
  - [構造 Generator](#構造-generator)
  - [輸出](#輸出)
    - [生成程式碼: 包含 match-expr 的 const 函式](#生成程式碼-包含-match-expr-的-const-函式)
      - [**output\_match\_fn()**](#output_match_fn)
      - [**output\_match\_fn\_all\_in\_one()**](#output_match_fn_all_in_one)
      - [**output\_match\_fn\_all\_in\_one\_by\_language\_and\_key()**](#output_match_fn_all_in_one_by_language_and_key)
    - [生成程式碼: 包含 phf map 的 const 函式](#生成程式碼-包含-phf-map-的-const-函式)
      - [**output\_phf()**](#output_phf)
      - [**output\_phf\_all\_in\_one()**](#output_phf_all_in_one)
    - [bincode](#bincode)
      - [**output\_bincode()**](#output_bincode)
- [高階用法](#高階用法)
  - [語法高亮](#語法高亮)
    - [資料結構](#資料結構)
    - [剖析](#剖析)
      - [Key](#key)
      - [Value](#value)
    - [Example](#example)

</details>

---

glossa-codegen 能夠用來生成 (包含本地化文字的) rust 程式碼，以及 bincode。

> 注：儘管 glossa-codegen 需要 std，但是 glossa 和 glossa-shared 都支援 no-std 環境。
>
> - glossa-codegen 用於生成**正式**程式碼。
> - glossa 用於生成 fallback chain。
> - glossa-shared 提供**正式**程式碼所需的各種資料型別。
>
> 您只需要在 `#[test]` 測試程式碼或 `build.rs` 中引入 glossa-codegen，並不需要在正式程式碼中引入。

## 基本概念

### 語言 id 與 map_name

假設存在 locales 目錄，其結構如下所示。

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

其中， `"ar", "en", "es", "fr", "ru", "zh"` 為 **語言 ID**。

"error" 和 "yes-no" 為 map 名稱。

> 不同 file_stem (e.g., a.toml, b.json) 對應不同的 map 名稱。
>
> 那麼相同的呢？(e.g., a.toml, a.json)

Q：假設同時存在 "error.yaml"、 "error.yml"、 "error.toml"、 "error.json5"、 "error.json"、 "error.ron"，那麼哪一個才是真正的 "error" map 呢？

A:
  如果所有檔案內容都是有效且不為空的 K-V String Pairs，那麼靠運氣！
  否則的話，第一個**有效**的“同 stem” 檔案將成為真正的 map。

> 注：a.toml => a 與 a.dsl.toml => a.dsl 不是同 stem 檔案。
>
> en/a.toml => a 與 en/subdir/a.json => a 是同 stem 檔案

🍀🍀🍀

Q：為什麼要靠運氣呢？

A:
  因為在初始化本地化資源的時候，會呼叫 rayon 來進行多執行緒反序列化（多個檔案多個執行緒同時讀取並解析）。
  其中的順序並不是有序的。

### L10n 資料

| L10n 型別          | 描述                                   |
| ------------------ | -------------------------------------- |
| Raw 文字檔案       | 未經處理的原始檔案，比如 en/hello.toml |
| 轉換出來的rust程式碼 | 支援 const fn，直接硬編碼到程式中      |
| bincode            | 支援高效反序列化的二進位制檔案           |

我們可以簡單將 Raw 檔案理解為原始碼，其他東西都是用 Raw 檔案編譯出來的。

### Raw L10n 文字語法

#### 常規 K-V pairs

這是最基本的型別。

以 toml 為例：

`key = "value"`，其中 key 和 value 都為字串。

```toml
hello = "你好"
"🐱" = "喵 ฅ(°ω°ฅ)"
```

以 json5 為例：

```json
{
  // json5 可以用註釋
  "hello": "你好",
  "🐱": "喵 ฅ(°ω°ฅ)", /* 可以尾隨逗號 "," */
}
```

#### glossa-DSL

[![glossa-dsl.crate](https://img.shields.io/crates/v/glossa-dsl.svg?logo=rust&logoColor=lightsalmon&label=glossa-dsl)](https://github.com/2moe/glossa-dsl)

> DSL: 領域特定語言

我們可以在 5 分鐘內，掌握 glossa-dsl 的 5 種語法。

##### 1. 最基本的 **key = "value"**

- toml: **name = "Tom"**
- json: `{"name": "Tom"}`

##### 2. 引用

toml:

```toml
name = "Tom"
hello = "Hello { name }"
```

①. hello 引用了 `{ name }` （注：`{ name }` 與 `{name}` 本質上是一樣的）
②. 展開 hello
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
②. 展開 `hello`
③. 得到了 `"Hello ฅ(°ω°ฅ)"`.

rust:

```rust
let text = res.try_get("hello")?;
assert_eq!(text, "Hello ฅ(°ω°ฅ)");
```

##### 3. 外部傳入的引數

toml:

```toml
"打招呼" = "早安喵 { $🐱 }"
greeting = "{ 打招呼 }，{ $name }！"
```

> `{ $🐱 }` 和 `{ $name }` 依賴外部傳入的引數

rust:

```rust
let ctx = [("name", "Moe"), ("🐱", "ฅ(°ω°ฅ)")];

let text = res.get_with_context("greeting", &ctx)?;
assert_eq!(text, "早安喵 ฅ(°ω°ฅ)，Moe！");
```

---

###### `{ 🐱 }` 與 `{ $🐱 }` 的區別

重點是有沒有加 `$`，加了 `$` 就依賴於外部引數，沒加就是內部引用。

內部引用：

```toml
"🐱" = "ฅ(°ω°ฅ)"
meow = "{ 🐱 }"
```

依賴外部傳入引數:

```toml
meow = "{ $🐱 }"
```

##### 4. 選擇器（條件控制語法）

zh/unread.toml:

```toml
"阿拉伯數字轉漢字" = """
  $num ->
    [0] 〇
    [1] 一
    [2] 二
    [3] 三
    [10] 十
    *[其他] {$num}
"""

"未讀msg" = "未讀訊息"

"顯示未讀訊息數量" = """
  $num ->
      [0] 沒有{ 未讀msg }
      [2] 您有兩條{ 未讀msg }
     *[其他] 您有{ 阿拉伯數字轉漢字 }條{ 未讀msg }
"""

show-unread-messages-count = "{顯示未讀訊息數量}。"
```

rust:

```rust
let get_text = |num_str| res.get_with_context("show-unread-messages-count", &[("num", num_str)]);

assert_eq!(get_text("0")?, "沒有未讀訊息。");
assert_eq!(get_text("1")?, "您有一條未讀訊息。");
assert_eq!(get_text("2")?, "您有兩條未讀訊息。");
assert_eq!(get_text("10")?, "您有十條未讀訊息。");
assert_eq!(get_text("100")?, "您有100條未讀訊息。");
```

我們可以將 "顯示未讀訊息數量" 理解為一個函式， `$num` 理解為函式的引數。

若將上文的 toml 文字理解為 rust 程式碼，則其會是如此：

```rust
let 未讀msg = "未讀訊息";
let 顯示未讀訊息數量 = |num| match num {
  "0" => fmt!("沒有{未讀msg}"),
  "2" => fmt!("您有兩條{未讀msg}"),
  _ => fmt!("您有{n}條{未讀msg}", n = 阿拉伯數字轉漢字(num)),
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

##### 5. 轉義語法

在上文中，我們瞭解到 `{ a }` 就是內部引用，而 `{ $a }` 依賴於外部傳入的 `a` 引數。

Q：如果需要得到使用原始的 `{a  }`，避免其自動解析，那該怎麼辦呢？

A：使用轉義語法。

- 如果需要得到原始的 `{a  }`，那麼外部至少需要包裹兩層 `{}`, 也就是 `{{  {a  }   }}`。
- 如果需要得到原始的 `{{a  }`，那麼外部至少需要包裹三層 `{}`, 也就是 `{{{  {{a  }     }}}`。

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
- Highlight：帶有語法高亮的 K-V pairs
- RegularAndHighlight： 融合了 Regular 和 Highlight。
- DSL：glossa-DSL。由於 MapType 一般配合 `.output_*` 使用，因此當 MapType 為 DSL 時，輸出的Map為 glossa-DSL 的 AST，而不是 Raw glossa-DSL。

> AST：抽象語法樹

從本質上來上說，Regular 與 Highlight 使用相同的資料結構。
之所以將它們分開，是為了更“細粒度”的控制。

## L10nResources (本地化資源)

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

- dir： 本地化資源所在的目錄，例如 "./locales"
- dsl_suffix
  - glossa-DSL 檔案的字尾，預設為 ".dsl"
    - 當其值為 ".dsl" 時
      - "a.dsl.toml" 會被識別為 **glossa-DSL** 檔案
      - "b.dsl.json" 也會被識別為 **glossa-DSL** 檔案
      - "a.toml" 為常規檔案
- include_languages
  - 白名單模式，當其不為空時，只有位於列表中的語言 id 才會被初始化
    - 假設所有語言 id 為: "de", "en", "es", "pt", "ru", "zh"
    - `.with_include_language(["en", "zh"])` => 只有 "en" 和 "zh" 的本地化資源才會被初始化
- include_map_names
  - 當其不為空時，只有位於列表中的 map_names 才會被初始化。
    - 假設存在: "en/a.toml", "en/b.json", "zh/a.json", "zh/b.ron"
    - 不難看出，所有 map_names 為 `["a", "b"]`
    - `.with_include_map_names(["a"])` => 只有 "en/a.toml" 和 "zh/a.json" 會被初始化
- exclude_languages
  - 黑名單模式。位於黑名單中的語言 id 不會被初始化
    - 假設存在: "de", "en", "es", "pt", "ru", "zh"
      - `.with_exclude_languages(["en", "es", "ru"])` => `["de", "pt", "zh"]`
      - `.with_include_languages(["en", "es"]).with_exclude_languages(["en"])` => `["es"]`
- exclude_map_names
  - 位於列表中的 map_names 不會被初始化
  - 假設存在:
    - "en/a.toml"
    - "en/b.json"
    - "zh/a.json"
    - "zh/b.ron"
    - "zh/c.toml"
  - `.with_exclude_map_names(["a"])` => "en/b.json", "zh/b.ron", "zh/c.toml"
  - `.with_include_map_names(["b", "c"]).with_exclude_map_names(["b"])` => "zh/c.toml"
  - `.with_include_language(["en"]).with_exclude_map_names(["a"])` => "en/b.json"
- lazy_data
  - 在執行期間**延遲**初始化的資料
  - 透過 `.get_or_init_data()` 來獲取資料，相當於快取

| 方法                                     | 描述                                             |
| ---------------------------------------- | ------------------------------------------------ |
| `.get_dir()`                             | 獲取 dir                                         |
| `.with_dir("/path/to/new_dir".into())`   | 設定 dir                                         |
| `.get_dsl_suffix()`                      | 獲取 dsl_suffix                                  |
| `.with_dsl_suffix(".new_suffix".into())` | 設定 dsl_suffix                                  |
| `.with_include_languages([])`            | 設定 include_languages                           |
| `.with_include_map_names([])`            | 設定 include_map_names                           |
| `.with_exclude_languages([])`            | 設定 exclude_languages                           |
| `.with_exclude_map_names([])`            | 設定 exclude_map_names                           |
| `.get_or_init_data()`                    | 獲取 `&HashMap<KString, Vec<L10nMapEntry>>`      |
| `.with_lazy_data(OnceLock::new())`       | 設定 lazy_data，可以將OnceLock重置為未初始化狀態 |

Q: 如何構造一個新的 L10nResources 結構體呢？

A：

```rust
use glossa_codegen::L10nResources;
let _res = L10nResources::new("locales");
// 相當於 L10nResources::default().with_dir("locales".into())
```

"locales" 可以改成其他目錄，比如 "../../l10n/"

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

- resources: 本地化資源
- visibility
  - 生成的 rust 程式碼的可見性, 預設為 PubCrate
    - > `glossa_codegen::Visibility { Private, PubCrate, Pub, PubSuper }`
  - `.with_visibility(Visibility::Pub)` => `pub const fn xxx`
  - `.with_visibility(Visibility::PubCrate)` => `pub(crate) const fn xxx`
- outdir
  - 輸出 rust 程式碼以及 bincode 的目錄
- bincode_suffix: bincode檔案字尾，預設為 ".bincode"
- mod_prefix
  - 生成的 rust 程式碼的模組字首，預設為 "l10n_"
- highlight: 語法高亮的配置，這個稍微有點複雜，我們將會在高階用法中提到。
- lazy_maps
  - 延遲初始化的maps
  - 相關方法：
    - `.get_or_init_maps()`  // Regular
    - `.get_or_init_highlight_maps()` // Highlight
    - `.get_or_init_merged_maps()` // RegularAndHighlight
    - `.get_or_init_dsl_maps()` // Template

### 構造 Generator

```rust
use glossa_codegen::{Generator, L10nResources};

let resources = L10nResources::new("locales");

let generator = Generator::default()
  .with_resources(resources)
  .with_outdir("tmp");
```

### 輸出

- 內部是 match 表示式的 const 函式
  - 呼叫 Generator 的 `.output_match_fn(MapType::Regular)` 會生成 rust 程式碼
    - `const fn map(map_name: &[u8], key: &[u8]) -> &'static str { match (map_name, key) {...} }`
- phf map 函式
  - 呼叫 Generator 的 `.output_phf(MapType::Regular)` 會生成 rust 程式碼
    - `const fn map() -> super::PhfL10nOrderedMap { ... }`
- bincode
  - 呼叫 Generator 的 `.output_bincode(MapType::Regular)` 會生成 bincode 二進位制檔案

MapType::DSL 只能輸出為 bincode，而其他 MapType 支援所有的輸出型別。

> 您可以將 DSL 指定為 Regular Map（可能需要修改 L10nResources 的 dsl_suffix），不過這樣做並不會帶來效能優勢。因為解析 DSL 的 AST 要比解析 Raw DSL 更快。
>
> 當將 DSL 指定為 Regular 時，生成的程式碼是 Raw K-V pairs。在執行期間需要先將其解析為 AST，再進行處理。
>
> 而若將 MapType::DSL 直接輸出為 bincode，那輸出的結果就是 DSL 的 AST 的 bincode，而不是 Raw K-V pairs。

#### 生成程式碼: 包含 match-expr 的 const 函式

相關方法有：

- `.output_match_fn()`
  - 為不同的語言生成獨立的 rust 程式碼檔案
  - => `{outdir}/{mod_prefix}{snake_case_language}.rs`
    - 比如
      - en => tmp/l10n_en.rs
      - en-GB => tmp/l10n_en_gb.rs
- `.output_match_fn_all_in_one()`
  - 將所有語言的本地化資源都收集為一個字串
    - 其內容為 `const fn map(lang: &[u8], map_name:&[u8], key:&[u8]) -> &'static str {...}`
- `.output_match_fn_all_in_one_by_language()`
  - 將所有語言的本地化資源都收集為一個字串
    - 其內容為 `const fn map(language: &[u8]) -> &'static str {...}`
    - 只有當 map_name 和 key 都只有唯一一個時，您才能使用此函式，否則 map_name 和 key 會出現衝突。
- `.output_match_fn_all_in_one_by_language_and_key()`
  - 將所有語言的本地化資源都收集為一個字串
    - 其內容為 `const fn map(language: &[u8], key: &[u8]) -> &'static str {...}`
    - 只有當 map_name 只有唯一一個時，您才能使用此函式，否則 key 會出現衝突。

##### **output_match_fn()**

假設存在如下兩個檔案：

l10n/en-GB/error.toml

```toml
text-not-found = "No localised text found"
```

l10n/de/error.yml

```yaml
text-not-found: Kein lokalisierter Text gefunden
```

我們可以呼叫 `.output_match_fn(Regular)` 來生成常規型別的 Map 的程式碼。

```rust
use glossa_codegen::{generator::MapType, Generator, L10nResources};

let resources = L10nResources::new("l10n");

Generator::default()
  .with_resources(resources)
  .with_outdir("tmp")
  .output_match_fn(MapType::Regular)?;
```

輸出結果:

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

Q: 我們如果使用 `output_match_fn_all_in_one()` ，那麼會得到什麼呢？
A: 會得到一個包含函式資料的 String。

> 所有語言的本地化資源都在同一個函式中

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

當 map_name 只有唯一一個時，我們可以省略它，以此來達到效能最佳化的目的。

```rust
match (lang, key) { ... }
```

```rust
match (lang, map_name, key) { ... }
```

將兩段 match 表示式進行對比：由於前者少匹配了一個項，所以從理論上來說，前者會更快。

`output_match_fn_all_in_one_by_language_and_key()` 會生成類似於前者的程式碼。

您如果不關心納秒級別的效能最佳化，那麼完全不用在意這一小節的內容。

---

舉個例子：

- `en/yes-no { yes: "Yes", no: "No"}`
- `de/yes-no { yes: "Ja", no: "Nein" }`

在本例中，唯一的 map_name 是 yes-no，因此我們可以省略它。

呼叫 `.output_match_fn_all_in_one_by_language_and_key(Regular)?` 會生成如下程式碼：

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

當 map_name 不是唯一時，比如: 新增一個 `en/yes-no2 { yes: "YES", no: "NO", ok: "OK"}`。

此時不同的 map_names 有相同的 keys ("yes", "no")，這會產生衝突，我們就不能省略 map_name 了。
在這種情況下，我們應該用 `output_match_fn_all_in_one()`。

#### 生成程式碼: 包含 phf map 的 const 函式

- `.output_phf()`
  - 為不同的語言生成獨立的 rust 程式碼檔案
- `.output_phf_all_in_one()`
  - 將所有語言的本地化資源都收集為一個包含 phf map 的函式資料的字串

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

Q：等等，PhfL10nOrderedMap 和 PhfTupleKey 都是哪來的？

A: [![glossa-shared.crate](https://img.shields.io/crates/v/glossa-shared.svg?logo=rust&logoColor=lightsalmon&label=glossa-shared)](https://crates.io/crates/glossa-shared) 裡包含了相關的資料型別。

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
      (Key(r#"zh"#, r##"yes-no"##, r###"ok"###), r#####"確定"#####),
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
  - 所有語言的 L10n 資源
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

## 高階用法

### 語法高亮

[![hlight.crate](https://img.shields.io/crates/v/hlight.svg?logo=rust&logoColor=lightsalmon&label=hlight)](https://crates.io/crates/hlight)

---

glossa-codegen 支援將本地化文字渲染成包含語言高亮的內容，並轉換為 rust 程式碼和 bincode。

Q: 為什麼需要預先渲染呢？

A: 為了效能最佳化。
直接輸出常量的 `&'static str` 會比在執行期間使用正則表示式進行語法高亮渲染快很多倍。

Q: 常量的語法高亮字串有何用武之地？

A: 我們在開發 CLI 應用時，幫助資訊可以使用常量的語法高亮字串。
既保證了效能，又兼顧了可讀性。

![highlight_sample](../../../assets/img/zh/highlight_help.png)

#### 資料結構

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

> 因為我們還沒有配置一個有效的 HighlightCfgMap，所以此程式碼無法正常執行！
>
> 別擔心，只要將 `HighlightCfgMap::default()` 改成有效的資料，這段程式碼就能跑起來。

---

**核心概念:**

HighlightCfgMap 的作用是為多個 maps 應用不同的語法高亮配置。

**目錄結構示例:**

```plaintext
en/
 ├── help-markdown.toml    // Base map: help-markdown
 └── a-zsh.toml           // Base map: a-zsh
```

**配置示例（虛擬碼）：**

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

base-name 會引用一個真實存在的常規 map，所以不能亂改名。
在上面的例子中，codegen 會在 "help-markdown" 的基礎上應用語法高亮，然後生成一個新的 Map（map_name: "help-markdown_monokai"）。

我們可以自定義 suffix，但需要避免 `format!("{base_name}{suffix}")` 與 regular map 的名稱衝突。

##### Value

```rust
struct SyntaxHighlightConfig<'r> {
  resource: HighlightResource<'r>,
  syntax_name: MiniStr,
  true_color: bool,
}
```

- resource
  - 我們可以自定義主題名稱，主題集，語法集，配置是否啟用背景
  - 詳見 [hlight 的文件](https://docs.rs/hlight)
- syntax_name
  - 語法名稱
  - 如果不支援相關語法的話，那麼您需要配置 HighlightResource，載入自定義的語法集 (SyntaxSet)。
- true_color
  - 若其值為 true，則啟用真彩色，否則使用古早的 256-color。
  - 開啟與否主要看您的終端是否支援真彩色。
    - 在支援的終端上，開啟 true_color 會讓色彩更準確。
    - 在不支援的終端上，比如 macOS 15.3 的 Terminal.app(v2.14)，開啟 true_color 會讓色彩變得很奇怪。

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
