# Rust 异步与并发学习

个人练习仓库，覆盖 **原子操作、线程与同步原语、Tokio、crossbeam-channel、Rayon** 等主题。

仓库地址：<https://github.com/cshonor/rust-async-atomic-crossbeam-tokio-rayon>

## 学习内容概览

| 主题 | 代码位置 | 说明 |
|------|----------|------|
| 线程、`Mutex` / `Condvar`、`Arc` 等 | `atomic/Chapter-01-Rust-Concurrency-Basics/`（`study_atomic::chapter_01`） | 原 ex1 系列示例 |
| `std::sync::atomic`、内存序、栅栏 | `atomic/Chapter-02-Atomics/` | 原 ex2 系列示例 |
| async / await、`tokio::spawn`、通道与异步锁 | `src/async_tokio/` | Tokio 运行时示例 |
| 有界通道、`thread::scope` | `crossbeam/src/lib.rs`（`study_crossbeam`） | `crossbeam-channel` + `crossbeam-utils` |
| 并行迭代器 | `rayon/src/lib.rs`（`study_rayon`） | `par_iter` 等 |

`chapter_01` 编号索引：

| 编号 | 模块（文件） |
|------|--------------|
| 01 | `move_closure.rs` |
| 02 | `thread_advanced.rs` |
| 03 | `thread_example.rs` |
| 04 | `use_cell_refcell.rs` |
| 05 | `use_condvar.rs` |
| 06 | `use_mutex.rs` |
| 07 | `use_mutex_guard_lifetime.rs` |
| 08 | `use_rc_arc.rs` |
| 09 | `use_send_sync.rs` |
| 10 | `useboxleak.rs` |
| 11 | `usecall.rs` |
| 12 | `usejoin.rs` |
| 13 | `userecall.rs` |
| 14 | `usescope.rs` |
| 15 | `usestatic.rs` |

`chapter_02` 编号索引：

| 编号 | 模块（文件） |
|------|--------------|
| 01 | `id_allocator.rs` |
| 02 | `lazy_init.rs` |
| 03 | `quick_demo.rs` |
| 04 | `use_atomic.rs` |
| 05 | `use_atomic_operations.rs` |
| 06 | `use_fence.rs` |
| 07 | `use_seqcst.rs` |

## 如何运行

在**仓库根目录**（与 `Cargo.toml` 同级）执行：

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

## 目录结构

```
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── atomic/                 # workspace：`study_atomic`
│   ├── Cargo.toml
│   ├── README-学习区.md
│   ├── src/
│   │   └── mod.rs          # #[path] 引入各 Chapter 下 mod.rs
│   ├── Chapter-01-Rust-Concurrency-Basics/   # 第 1 章源码 + note.md
│   ├── Chapter-02-Atomics/
│   └── Chapter-03-… ~ Chapter-10-…/          # 笔记占位（英文文件夹名）
├── crossbeam/              # workspace：`study_crossbeam`
│   ├── Cargo.toml
│   └── src/lib.rs
├── rayon/                  # workspace：`study_rayon`
│   ├── Cargo.toml
│   └── src/lib.rs
├── async_tokio/            # 按章笔记与 demo（可选，非 crate）
├── src/
│   ├── main.rs             # 命令行入口
│   ├── lib.rs
│   └── async_tokio/
```

## 许可证

学习与笔记用途；如需开源许可证可在仓库中自行补充 `LICENSE`。
