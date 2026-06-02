# 第 2 章 — Rust 及其生态系统简介（Introduction to Rust and its Ecosystem）

> 《Network Programming with Rust》· 全书 **10 个小节**（§2.6、§2.7 各含 2 个次级小节）  
> 章索引：[本章学习笔记.md](../本章学习笔记.md) · 阶段：[stage02](../README.md)

本章为后续网络编程提供语言基础：生态与入门 → 借用/泛型/错误 → 宏与函数式 → 并发与测试。

---

## 章节目录（与原书一致）

| § | 英文 | 中文 |
|---|------|------|
| 2.1 | The Rust ecosystem | Rust 生态系统 |
| 2.2 | Getting started with Rust | Rust 入门 |
| 2.3 | Introduction to the borrow checker | 借用检查器简介 |
| 2.4 | Generics and the trait system | 泛型与特征系统 |
| 2.5 | Error handling | 错误处理 |
| 2.6 | The macro system | 宏系统（含语法宏、过程宏） |
| 2.7 | Functional features in Rust | 函数式特性（含高阶函数、迭代器） |
| 2.8 | Concurrency primitives | 并发原语 |
| 2.9 | Testing | 测试 |
| 2.10 | Summary | 总结 |

---

## 2.1 The Rust ecosystem（Rust 生态系统）

Rust 社区与工具链是写网络程序的前提。

| 组件 | 作用 |
|------|------|
| **`rustc`** | 编译器；渠道：**stable**（学习与生产）、**beta**、**nightly**（实验特性） |
| **`rustup`** | 安装/切换工具链，管理 `clippy`、`rustfmt` 等组件 |
| **`Cargo`** | 包管理 + 构建 + 测试 + 运行 |
| **crates.io** | 包注册中心；`Cargo.toml` 声明依赖即可拉取 |

网络项目常见依赖：`tokio`、`serde`、`bytes`、`reqwest` 等，均通过 Cargo 引入。

---

## 2.2 Getting started with Rust（Rust 入门）

书中演示用 Cargo 创建项目并从 crates.io 引入外部 crate（如 **`term`** 做终端输出）。

```bash
cargo new my_net_app
cd my_net_app
# Cargo.toml [dependencies] 中添加 crate
cargo build
cargo run
```

| 命令 | 作用 |
|------|------|
| `cargo new` | 新建二进制或库工程 |
| `cargo build` | 编译（debug / `--release`） |
| `cargo run` | 编译并运行 |
| `cargo test` | 运行测试（见 §2.9） |

**与网络的关系**：后续 Ch3 起的 Socket 示例几乎都是 Cargo 工程；先熟悉工程布局（`src/main.rs`、`Cargo.toml`）再写 `std::net`。

---

## 2.3 Introduction to the borrow checker（借用检查器简介）

Rust **无 GC** 仍保证内存安全，靠**所有权 + 借用 + 生命周期**，在编译期拦截悬垂指针与数据竞争。

### 所有权（Ownership）

- 每个值在任意时刻有**唯一**所有者；所有者离开作用域 → **drop**。  
- **Move**：`String`、`Vec` 等堆数据赋值时转移所有权，旧绑定失效。  
- **Copy**：`i32`、`bool` 等栈上简单类型按位拷贝。

```rust
let s1 = String::from("hello");
let s2 = s1;   // move，s1 不再可用
```

### 借用（Borrowing）

| 类型 | 写法 | 同一作用域内 |
|------|------|----------------|
| 不可变借用 | `&T` | 可多个 |
| 可变借用 | `&mut T` | 至多一个，且不能与 `&T` 共存 |

### 生命周期（Lifetimes）

标注引用有效范围（`'a`、`'static`），避免悬垂引用。

- **`'static`**：整个程序有效；`thread::spawn`、部分异步任务要求 `'static` 闭包/Future。  

系统精读：[`atomic/Chapter-01`](../../../atomic/Chapter-01-Rust-Concurrency-Basics/本章学习笔记.md)。

---

## 2.4 Generics and the trait system（泛型与特征系统）

### 泛型

```rust
fn print_twice<T: std::fmt::Display>(x: T) {
    println!("{x} {x}");
}
```

一份实现，多种类型复用。

### 特征（Traits）

定义类型**共有行为**（类似 interface）：

- 实现 `Display`、`Debug`、`Add` 等标准 trait。  
- 自定义 trait + `impl Trait for MyType`。  
- 泛型约束：`T: Read + Write`。

**网络**：`Read` / `Write`、`AsyncRead` / `AsyncWrite` 是 I/O 抽象基石（Ch3/Ch7）。

---

## 2.5 Error handling（错误处理）

| 类型 | 含义 | 变体 |
|------|------|------|
| **`Result<T, E>`** | 可恢复错误 | `Ok` / `Err` |
| **`Option<T>`** | 可能无值 | `Some` / `None` |

```rust
fn read_config(path: &str) -> std::io::Result<String> {
    let mut f = std::fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

| 方式 | 适用 |
|------|------|
| **`?`** | 向上传播 `Err` |
| **`match` / `if let`** | 分支处理 |
| **`unwrap` / `expect`** | 原型（生产慎用） |
| **`panic!`** | 不可恢复逻辑错误 |

**网络 I/O**：`connect`、`read`、`write` 多返回 `Result`；顶层可用 `fn main() -> Result<(), Box<dyn Error>>`。

---

## 2.6 The macro system（宏系统）

宏在**编译期**展开；调用常带 **`!`**。具备**卫生性（Hygienic）**：宏内变量默认不污染调用方作用域。

### 2.6.1 Syntactic macros（语法宏）

用 **`macro_rules!`** 做模式匹配与模板替换：

```rust
macro_rules! say_hello {
    () => { println!("Hello!"); };
}
```

`vec!`、`println!`、`panic!` 均属此类。

### 2.6.2 Procedural macros（过程宏）

直接操作编译器 **AST**，更灵活：

| 形式 | 示例 |
|------|------|
| **derive 宏** | `#[derive(Serialize, Deserialize)]` |
| **属性宏** | `#[tokio::main]`、`#[test]` |
| **函数式宏** | `async_trait::async_trait` 等 |

书中指出过程宏有望逐步承担更多代码生成职责。网络开发中：**Serde derive**、**Tokio 入口宏** 最常见。

---

## 2.7 Functional features in Rust（Rust 中的函数式特性）

受 Haskell、OCaml 等影响；利于表达数据变换与组合逻辑。

### 2.7.1 Higher-order functions（高阶函数）

**高阶函数**：接收函数作为参数，或返回函数的函数。

**闭包（Closures）** 是 Rust 中最常用的高阶手段：

```rust
let add = |a, b| a + b;
let v = vec![1, 2, 3];
let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
```

- **`move`**：闭包取得捕获变量的所有权 → `thread::spawn`、异步 `spawn` 必备。  
- 函数指针 `fn(i32) -> i32` 也可作为参数传递。

### 2.7.2 Iterators（迭代器）

**迭代器**是惰性序列；`Iterator::next() -> Option<Item>`，`None` 表示结束。

```rust
for n in 1..5 {
    println!("{n}");
}
// 适配器：map、filter、fold — 在 collect/for 消费时才执行
```

自定义类型实现 `Iterator` 后可用 `for` 与适配器（书中 Collatz 等示例思路）。

**网络**：按行读（`lines()`）、批量处理连接、协议字段遍历。

---

## 2.8 Concurrency primitives（并发原语）

**无畏并发（fearless concurrency）**：1:1 线程模型（一个 Rust 线程 ≈ 一个 OS 线程）。

| 机制 | API / 概念 |
|------|------------|
| 创建线程 | `std::thread::spawn` |
| 消息传递 | `mpsc::channel` |
| 共享状态 | `Arc<Mutex<T>>` 等 |
| 线程安全标记 | **`Send`**（可跨线程移所有权）、**`Sync`**（可安全共享 `&T`） |

非法跨线程共享在**编译期**报错。  
**`unsafe`**：绕过部分检查，用于 FFI、底层结构；应用层尽量少用。

| 深挖 | 路径 |
|------|------|
| 线程、锁 | [`atomic/`](../../../atomic/Chapter-01-Rust-Concurrency-Basics/) |
| 异步并发 | [`async_tokio/`](../../../async_tokio/) · [Ch07](../../stage07_tokio_async_net/) |

---

## 2.9 Testing（测试）

测试在 Rust 中是一等公民。

| 类型 | 写法 |
|------|------|
| **单元测试** | 同文件 `#[cfg(test)] mod tests { #[test] fn ... }` |
| **集成测试** | 项目根 `tests/*.rs` |
| **文档测试（Doc-tests）** | `///` 代码块中的示例，`cargo test` 自动运行 |

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

网络项目：`127.0.0.1` 起临时服务测协议；异步用 `#[tokio::test]`。

---

## 2.10 Summary（总结）

| 块 | 要点 |
|----|------|
| 生态 | `rustc` + `rustup` + `Cargo` + crates.io |
| 核心 | 所有权/借用/生命周期；泛型 + trait；`Result`/`Option` |
| 进阶 | 语法宏 / 过程宏；闭包 + 迭代器 |
| 工程 | 线程、`mpsc`、`Send`/`Sync`；单元测试与 doc-test |

写网络代码时，编译器报错大多落在 **§2.3（借用）**、**§2.5（错误）**、**§2.8（Send/'static）** — 可优先回查这三节。

---

## 本章自检

- [ ] §2.1～2.2：能说明 Cargo 基本工作流  
- [ ] §2.3：move/copy、`&` / `&mut`、`'static` 与 spawn 的关系  
- [ ] §2.4：`Read`/`Write` 与 trait bound  
- [ ] §2.5：`Result` + `?` 在 I/O 中的用法  
- [ ] §2.6：能区分 `macro_rules!` 与 `#[derive]`  
- [ ] §2.7：`move` 闭包与 `par_iter` 以外的 `iter` 惰性链  
- [ ] §2.8：`mpsc` vs `Arc<Mutex<_>>`，`Send`/`Sync`  
- [ ] §2.9：`cargo test` 与 doc-test  

---

## 下一步

| 方向 | 路径 |
|------|------|
| Ch1 网络基础 | [stage01](../../stage01_network_basic/) |
| Ch3 `std::net` | [stage03](../../stage03_std_tcp_udp/) |
| Ch7 Tokio | [stage07](../../stage07_tokio_async_net/) |
