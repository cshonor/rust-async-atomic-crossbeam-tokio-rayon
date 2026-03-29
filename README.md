# Rust 异步与并发学习

个人练习仓库，覆盖 **原子操作、线程与同步原语、Tokio、crossbeam-channel、Rayon** 等主题。

仓库地址：<https://github.com/cshonor/rust-async-atomic-crossbeam-tokio-rayon>

## 学习内容概览

| 主题 | 代码位置 | 说明 |
|------|----------|------|
| 线程、`Mutex` / `Condvar`、`Arc` 等 | `src/atomic/chapter_01/` | 原 ex1 系列示例 |
| `std::sync::atomic`、内存序、栅栏 | `src/atomic/chapter_02/` | 原 ex2 系列示例 |
| async / await、`tokio::spawn`、通道与异步锁 | `src/async_tokio/` | Tokio 运行时示例 |
| 有界通道、`thread::scope` | `src/crossbeam/` | `crossbeam-channel` + `crossbeam-utils` |
| 并行迭代器 | `src/rayon/` | `par_iter` 等 |

## 如何运行

Rust 工具链需已安装（`rustc`、`cargo`）。在项目子目录中执行：

```bash
cd rust-async-atomic-crossbeam-tokio-rayon
```

```bash
cargo build
cargo run -- <命令> [子命令]
```

### 命令一览

| 命令 | 作用 |
|------|------|
| `cargo run -- all` | 依次运行：Tokio → 原子短演示 → crossbeam → rayon |
| `cargo run -- async_tokio` | Tokio 示例 |
| `cargo run -- atomic` | 原子模块短概览（`quick_demo`） |
| `cargo run -- atomic ex1` | 第一章长演示（条件变量等，对应 `chapter_01`） |
| `cargo run -- atomic ex2` | 第二章全套（Atomic、lazy_init、fence 等，对应 `chapter_02`） |
| `cargo run -- crossbeam` | crossbeam 示例 |
| `cargo run -- rayon` | Rayon 示例 |

无参数时运行 `cargo run --` 会打印用法说明。

## 目录结构（crate 内）

```
rust-async-atomic-crossbeam-tokio-rayon/
├── Cargo.toml
├── src/
│   ├── main.rs           # 命令行入口
│   ├── lib.rs
│   ├── atomic/
│   │   ├── chapter_01/   # 线程与同步
│   │   ├── chapter_02/   # 原子与内存序
│   │   └── mod.rs
│   ├── async_tokio/
│   ├── crossbeam/
│   └── rayon/
```

## 许可证

学习与笔记用途；如需开源许可证可在仓库中自行补充 `LICENSE`。
