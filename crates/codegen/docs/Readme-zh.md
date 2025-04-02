# glossa-codegen

glossa-codegen 能够用来生成 (包含本地化文本的) rust 代码，以及 bincode。

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

> 注：a.toml => a 与 a.tmpl.toml => a.tmpl 不是同 stem 文件。
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

#### 模版 DSL

[![tmpl-resolver.crate](https://img.shields.io/crates/v/tmpl-resolver.svg?logo=rust&logoColor=lightsalmon&label=tmpl-resolver)](https://github.com/2moe/tmpl-resolver)

模版 DSL 使用 tmpl-resolver 进行处理。

我们可以在 5 分钟内，掌握其 5 种语法。

##### 1. 最基本的 **key = "value"**

- toml: **name = "Tom"**
- json: `{"name": "Tom"}`

###### 2. 引用

toml:

```toml
name = "Tom"
hello = "Hello {name}"
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

###### 区别 `{ 🐱 }` 与 `{ $🐱 }`

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

```toml
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
  Template,
}
```

- Regular：K-V pairs
- Highlight：带有语法高亮的 K-V pairs
- RegularAndHighlight： 融合了 Regular 和 Highlight。
- Template：模版 DSL 的AST

从本质上来上说，Regular 与 Highlight 使用相同的数据结构。
之所以将它们分开，是为了更“细粒度”的控制。

## L10nResources (本地化资源)

```rust
pub struct L10nResources<'i> {
  dir: PathBuf,
  tmpl_suffix: MiniStr,
  include_languages: &'i [&'i str],
  include_map_names: &'i [&'i str],
  exclude_languages: &'i [&'i str],
  exclude_map_names: &'i [&'i str],
  /// get data: [Self::get_or_init_data]
  lazy_data: OnceLock<L10nResMap>,
}
```

- dir： 本地化资源所在的目录，例如 "./locales"
- tmpl_suffix
  - 模版DSL 文件的后缀，默认为 ".tmpl"
    - 当后缀为 ".tmpl" 时
      - "a.tmpl.toml" 会被识别为 **模版DSL** 文件
      - "a.toml" 则为常规文件
- include_languages
  - 白名单模式，当其不为空时，只有位于列表中的语言 id 才会被初始化
    - 假设所有语言 id 为: "de", "en", "es", "pt", "ru", "zh"
    - `.with_include_language(&["en", "zh"])` => 只有 "en" 和 "zh" 的本地化资源才会被初始化
- include_map_names
  - 当其不为空时，只有位于列表中的 map_names 才会被初始化。
    - 假设存在: "en/a.toml", "en/b.json", "zh/a.json", "zh/b.ron"
    - 不难看出，所有 map_names 为 `["a", "b"]`
    - `.with_include_map_names(&["a"])` => 只有 "en/a.toml" 和 "zh/a.json" 会被初始化
- exclude_languages
  - 黑名单模式。位于黑名单中的语言 id 不会被初始化
    - 假设存在: "de", "en", "es", "pt", "ru", "zh"
      - `.with_exclude_languages(&["en", "es", "ru"])` => `["de", "pt", "zh"]`
      - `.with_include_languages(&["en", "es"]).with_exclude_languages(&["en"])` => `["es"]`
- exclude_map_names
  - 位于列表中的 map_names 不会被初始化
  - 假设存在:
    - "en/a.toml"
    - "en/b.json"
    - "zh/a.json"
    - "zh/b.ron"
    - "zh/c.toml"
  - `.with_exclude_map_names(&["a"])` => "en/b.json", "zh/b.ron", "zh/c.toml"
  - `.with_include_map_names(&["b", "c"]).with_exclude_map_names(&["b"])` => "zh/c.toml"
  - `.with_include_language(&["en"]).with_exclude_map_names(&["a"])` => "en/b.json"
- lazy_data
  - 在运行期间**延迟**初始化的数据
  - 通过 `.get_or_init_data()` 来获取数据，相当于缓存

| 方法                               | 描述                                             |
| ---------------------------------- | ------------------------------------------------ |
| `.get_dir()`                       | 获取 dir                                         |
| `.with_dir("/path/to/new_dir")`    | 设置 dir                                         |
| `.get_tmpl_suffix()`               | 获取 tmpl_suffix                                 |
| `.with_tmpl_suffix(".new_suffix")` | 设置 tmpl_suffix                                 |
| `.with_include_languages(&[])`     | 设置 include_languages                           |
| `.with_include_map_names(&[])`     | 设置 include_map_names                           |
| `.with_exclude_languages(&[])`     | 设置 exclude_languages                           |
| `.with_exclude_map_names(&[])`     | 设置 exclude_map_names                           |
| `.get_or_init_data()`              | 获取 `&HashMap<KString, Vec<L10nMapEntry>>`      |
| `.with_lazy_data(OnceLock::new())` | 设置 lazy_data，可以将OnceLock重置为未初始化状态 |

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
pub struct Generator<'i, 'h> {
  resources: Box<L10nResources<'i>>,

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
    - `.get_or_init_template_maps()` // Template

### 构造 Generator

```rust
use glossa_codegen::{Generator, L10nResources};
let resources = L10nResources::new("locales");
let generator = Generator::default()
  .with_resources(resources)
  .with_outdir("tmp");
```

### 输出

- match 函数
- phf 函数
- bincode
