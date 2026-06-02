# Rust 模块系统总结

Rust 的代码组织可以从大到小理解为：

```text
Package
└── Crate
    └── Module
        └── Item
```

对应关系：

```text
Package: Cargo 管理的项目
Crate: Rust 编译器一次编译的单元
Module: crate 内部的代码命名空间
Item: fn、struct、enum、trait、const 等具体定义
```

## Package

Package 是 Cargo 管理的项目单位。一个包含 `Cargo.toml` 的目录就是一个 package。

例如当前目录：

```text
chapter07-modules/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

`Cargo.toml` 中的 `[package]` 定义了 package 的名字、版本、edition 等信息：

```toml
[package]
name = "chapter07-modules"
version = "0.1.0"
edition = "2024"
```

一个 package 可以包含：

- 至多一个 library crate
- 任意多个 binary crate

常见入口文件：

```text
src/lib.rs       # library crate 的 crate root
src/main.rs      # binary crate 的 crate root
src/bin/*.rs     # 额外的 binary crate
```

当前项目同时有 `src/lib.rs` 和 `src/main.rs`，所以它包含：

- 一个 library crate：`src/lib.rs`
- 一个 binary crate：`src/main.rs`

它们属于同一个 package，但不是同一个 crate。

## Crate

Crate 是 Rust 编译器的编译单元。每个 crate 都有一个 crate root，编译器从 crate root 开始分析代码。

默认规则：

```text
src/lib.rs   -> library crate root
src/main.rs  -> binary crate root
```

crate 有两类：

```text
Library crate: 编译成库，供其他 crate 使用
Binary crate: 编译成可执行程序，必须有 main 函数
```

如果同一个 package 中既有 `src/lib.rs` 又有 `src/main.rs`，那么 `main.rs` 所在的 binary crate 可以通过 package 名使用 library crate 中公开的 API。

例如 package 名叫 `chapter07-modules`，Rust 代码中的 crate 名会把 `-` 转为 `_`：

```rust
chapter07_modules::some_public_function();
```

## Module

Module 是 crate 内部组织代码的方式。它主要用于：

- 给代码分组
- 创建命名空间
- 控制私有和公开访问
- 避免命名冲突

当前 `src/lib.rs` 中的代码：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

对应的模块树：

```text
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```

这里的 `crate` 指当前 crate 的根，也就是 `src/lib.rs`。

## Item

Item 是模块里的具体定义，例如：

```rust
fn add_to_waitlist() {}

struct User {
    name: String,
}

enum Status {
    Active,
    Disabled,
}

trait Draw {
    fn draw(&self);
}

const MAX_TABLES: usize = 20;
```

函数、结构体、枚举、trait、常量等都可以是 item。

## `mod`

`mod` 用来声明模块。

它不是导入，也不是简单地引用另一个文件。

内联模块：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

拆分到文件时：

```rust
// src/lib.rs
mod front_of_house;
```

Rust 会去查找：

```text
src/front_of_house.rs
```

或者：

```text
src/front_of_house/mod.rs
```

现代 Rust 更常见的写法是：

```text
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

代码对应为：

```rust
// src/lib.rs
mod front_of_house;
```

```rust
// src/front_of_house.rs
mod hosting;
```

```rust
// src/front_of_house/hosting.rs
fn add_to_waitlist() {}
```

关键点：文件存在不代表模块自动存在，必须在父模块中用 `mod` 声明。

## `use`

`use` 用来把路径引入当前作用域，方便使用。

它不会创建模块，也不会加载文件。

不使用 `use`：

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

使用 `use`：

```rust
use crate::front_of_house::hosting;

hosting::add_to_waitlist();
```

对比：

```text
mod: 声明模块，把模块加入模块树
use: 引入路径，让名字更方便使用
```

## Path

Rust 通过路径访问模块和 item。

绝对路径从 crate root 开始：

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

相对路径从当前模块开始：

```rust
front_of_house::hosting::add_to_waitlist();
```

常见路径前缀：

```text
crate  当前 crate 根
self   当前模块
super  父模块
```

例如：

```rust
mod parent {
    fn hello() {}

    mod child {
        fn call_parent() {
            super::hello();
        }
    }
}
```

`super::hello()` 表示调用父模块 `parent` 中的 `hello`。

## `pub`

Rust 默认所有 item 都是私有的。

例如当前代码里：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

`front_of_house`、`hosting`、`add_to_waitlist` 都是私有的。

如果希望外部访问，需要使用 `pub`：

```rust
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

访问时：

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

需要注意：路径上的每一级都必须可见。

下面这样不够：

```rust
mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

虽然 `add_to_waitlist` 是 `pub`，但 `front_of_house` 和 `hosting` 是私有的，所以外部仍然不能访问。

## 更细的可见性

除了 `pub`，还可以限制公开范围：

```rust
pub(crate) fn foo() {}
```

只在当前 crate 内可见。

```rust
pub(super) fn foo() {}
```

只对父模块可见。

```rust
pub(in crate::front_of_house) fn foo() {}
```

只在指定模块路径内可见。

常用程度：

```text
pub           对外公开 API
pub(crate)    crate 内部共享
pub(super)    给父模块使用
```

## Re-export

`pub use` 可以重新导出路径，改变外部使用 API 的方式。

例如：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
```

外部可以这样使用：

```rust
chapter07_modules::hosting::add_to_waitlist();
```

而不需要写：

```rust
chapter07_modules::front_of_house::hosting::add_to_waitlist();
```

`pub use` 常用于隐藏内部模块结构，给使用者提供更稳定、更简洁的公开 API。

## 外部 Crate

外部依赖写在 `Cargo.toml` 的 `[dependencies]` 中。

例如：

```toml
[dependencies]
rand = "0.8"
```

使用时：

```rust
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=10);
}
```

Rust 2018 以后通常不需要写：

```rust
extern crate rand;
```

## Workspace

Workspace 是多个 package 的集合。

示例：

```text
my_workspace/
├── Cargo.toml
├── app/
│   ├── Cargo.toml
│   └── src/main.rs
└── core/
    ├── Cargo.toml
    └── src/lib.rs
```

根目录 `Cargo.toml`：

```toml
[workspace]
members = ["app", "core"]
```

workspace 可以：

- 统一构建多个 package
- 共享 `Cargo.lock`
- 统一管理依赖版本
- 方便大型项目拆分模块

## 常见误区

### `mod` 不是 `use`

```rust
mod hosting;
```

表示声明模块。

```rust
use crate::front_of_house::hosting;
```

表示把路径引入当前作用域。

### 文件不会自动变成模块

创建了这个文件：

```text
src/front_of_house.rs
```

还需要在父模块里声明：

```rust
mod front_of_house;
```

否则 Rust 不会把它加入模块树。

### `pub fn` 不代表一定能从外部访问

路径上的父模块也要公开。

```rust
mod a {
    pub mod b {
        pub fn foo() {}
    }
}
```

这里 `foo` 不能被 crate 外部访问，因为 `a` 是私有的。

需要写成：

```rust
pub mod a {
    pub mod b {
        pub fn foo() {}
    }
}
```

### `main.rs` 和 `lib.rs` 不是同一个 crate

它们属于同一个 package，但分别是不同 crate。

```text
src/lib.rs   -> library crate
src/main.rs  -> binary crate
```

binary crate 可以调用 library crate 暴露出来的公开 API。

## 记忆表

```text
Package      Cargo 项目，由 Cargo.toml 定义
Crate        编译单元，由 crate root 开始
Crate root   src/lib.rs 或 src/main.rs
Module       crate 内部的命名空间
Item         fn、struct、enum、trait、const 等定义
mod          声明模块
use          引入路径
pub          控制可见性
pub use      重新导出
```

最核心的理解：

```text
Cargo.toml 定义 package
src/lib.rs / src/main.rs 定义 crate
mod 建立模块树
use 简化路径
pub 控制可见性
```
