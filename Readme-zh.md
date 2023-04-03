# Glossa

Glossa 是一个用于语言本地化（localisation）的库。

## Functionality

划分为两类

- 编译时：将配置文件转换为常量（const fn）rust 代码，从而实现高效的本地化。
  - 优点：高效
  - 缺点：
    - 需要 `codegen`, 代码膨胀后会有一些冗余的东西
    - 目前仅支持简单的键值（K-V）对
- 运行时：管理 fluent 资源
  - 优点：fluent 的语法可能更适合本地化
  - 缺点：占用更多的资源

注：fluent 同样支持在编译时加载本地化资源，但需要在运行时解析数据。  
前者只是简单的 K-V 对，使用了 phf 的 const map 来存储数据。  
因为简单，所以高效。

两类功能相互独立，对于后者，请参阅 [Fluent.md](Fluent.md)

## codegen

使用代码生成器来生成代码。

glossa-codegen 有以下 features：

- yaml
- ron
- toml
- json

默认启用的是 `yaml`。

这对应不同类型的配置文件。  
您可以启用全部的功能，也可以按需添加。

默认根据文件名后缀来判断文件类型，根据文件名称来设置 Map Name(表的名称)，根据启用的功能来判断是否需要在编译时解析（反序列化）。

假设 `assets/l10n/zh` 目录下存在两个文件，分别是 `test.yaml` 和 `test.yml`，那么我们可以认为它们有着相同的名称。

这时候会有两个表：

- test
- test.yml

为了避免这种情况，建议为不同的文件使用不同的名称。

> 当调用 `MapLoader` 的 `.get()` 时，你需要传入 map name。

### 准备工作

在编写 `build.rs` 之前，我们需要先准备本地化资源文件。

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

> 注：pt 指的是“葡萄牙语（巴西）”,而不是“葡萄牙语（葡萄牙）” （português, Portugal）

```yaml
text-not-found: Nenhum texto localizado encontrado
```

### build.rs

先添加编译时依赖

```sh
cargo add --build glossa-codegen
```

然后开始创建 `build.rs`。

> 该文件与 Cargo.toml 同级。

简单的单项目结构

```js
.
├── assets
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── src
└── target
```

稍微复杂一点的多项目结构

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

当然，您可以手动指定 `build.rs` 的路径，而不是使用默认的。

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
    // 指定版本号为当前软件包的版本, 避免相同版本反复编译
    let version = Some(get_pkg_version!());
    // 在开发时，我们可以将其设置为 None
    // let version = None;

    // 这是一个常量数组： ["src", "assets", "localisation.rs"]，它会转化为路径，用于存储自动生成的（与本地化相关的）rust 代码。
    // 在 Windows 上，路径为 'src\assets\localisation.rs'
    // 在 Unix, 路径则是 "src/assets/localisation.rs"
    // 注意：这是相对路径！
    let mut path = PathBuf::from_iter(default_l10n_rs_file_arr());

    // 如果已经是相同版本，那就退出。
    if is_same_version(&path, version)? {
        return Ok(());
    }

    // 如果路径为 "src/assets/localisation.rs"，那么它会追加 `mod localisation;` 以及相关的 `use` 语句到 "src/assets/mod.rs"
    append_to_l10n_mod(&path)?;

    // 这里会创建一个新的文件： "src/assets/localisation.rs"
    // 与 append 不同，如果只是单纯的 create 的话，那么会清空文件。
    let mut file = BufWriter::new(File::create(&path)?);

    // default_l10n_dir_arr() 也是一个常量数组： ["assets", "l10n"]
    // 如果当前本地化资源的路径位于上一级的话，那么您可以使用 `path = PathBuf::from_iter([".."].into_iter().chain(default_l10n_dir_arr()));`
    path = PathBuf::from_iter(default_l10n_dir_arr());

    // 此处将 l10n 文件反序列化为 map， 并写入 rs 文件
    // file: "src/assets/localisation.rs"
    // path: "assets/l10n"
    // visibility（可见性）：用来设定自动生成的 `fn` 的可见性。 若为 None, 则使用 Some("pub(crate)")。您可以传入 `Some("pub(in path)")` 或 `Some("pub")` 等值。
    deser_cfg_to_map(&mut file, &mut path, Some("pub(crate)"), version)
}
```

## Get Text

代码生成完毕后，让我们编写一个函数来测试一下吧！

不过在那之前，我们得要先添加依赖。

```sh
cargo add phf glossa
```

测试函数如下：

```rust
    #[test]
    fn new_loader() {
        use crate::assets::localisation::locale_hashmap;
        use glossa::{fallback::FallbackChain, GetText, MapLoader};

        let loader = MapLoader::new(locale_hashmap());
        loader.show_chain();
        // 这里为了简易性，使用了 `get_or_default()`
        // 实际上 `.get()` 的用法是一样的，不过它返回的是 Result<&str>, 而不是 Cow<str>
        let msg = loader.get_or_default("error", "text-not-found");
        assert_eq!(msg, "No localized text found");
    }
```

如果您的系统语言是 en, 那么测试应该会成功。

请注意: `locale_hashmap()` 不是 `const fn`, 而是普通的函数。  
但这并不意味着开销特别大。

HashMap 查询操作的时间复杂度是 **O(1)**。

它的值指向了子表，子表以及子表的子表全都是 `consts`。

此外，如果启用了 ahash feature，那么默认会使用 ahash 的 RandomState, 而不是标准库的。

您还可以用 OnceCell 来创建全局静态数据, 只创建一次数据。

```rust
pub(crate) fn locales() -> &'static MapLoader {
    static RES: OnceCell<MapLoader> = OnceCell::new();
    RES.get_or_init(|| MapLoader::new(locale_hashmap()))
}
```

> 等等，别扯这些没用的，我们刚刚的测试失败了。

好吧，让我们重新回顾之前做的事情。  
我们之前创建了德语、西班牙语和葡萄牙语的本地化资源文件。

首先，它会自动检测系统语言，如果本地化资源不存在，那么它会自动使用 fallback chain。  
如果本地化资源存在，并且您的系统语言不是英语，那么上面的测试会失败。

让我们继续测试：

```rust
let loader = locales();
let msg = loader.get("error", "text-not-found")?;
```

假设您的语言是德语 (de-Latn-DE)

```rust
assert_eq!(msg, "Kein lokalisierter Text gefunden");
```

西班牙语 (es-Latn-ES)

```rust
assert_eq!(msg, "No se encontró texto localizado");
```

葡萄牙语 (pt-Latn-BR)

```rust
assert_eq!(msg, "Nenhum texto localizado encontrado");
```
