# Rust Atomics and Locks — 章节学习区

本目录 **`atomic/`** 作为**总区**：与 **`Cargo.toml`**、**`src/`** 平级，下面直接是 **10 个英文命名的 `Chapter-…` 文件夹**。

## 两块内容怎么分

| 位置 | 用途 |
|------|------|
| **`src/mod.rs`** | `study_atomic` 库入口：用 **`#[path = "../Chapter-01-…/mod.rs"]`** 等方式，把第 1、2 章文件夹挂成 **`chapter_01` / `chapter_02`** 模块。 |
| **`Chapter-01-Rust-Concurrency-Basics/`** | 第 1 章：笔记 **`note.md`** + 全部 `*.rs`（含 `mod.rs`、`README.md`）。 |
| **`Chapter-02-Atomics/`** | 第 2 章：笔记 **`note.md`** + 全部 `*.rs`。 |
| **`Chapter-03`～`Chapter-10`** | 目前仅 **`note.md`** 占位；后续可按书补代码或再挂 `#[path]`。 |

构建：`cargo build -p study_atomic` 或根目录 `cargo run -- atomic`。

## 章节目录（文件夹名 = 英文）

| 文件夹名 | 主题（中文） | 源码 |
|----------|----------------|------|
| `Chapter-01-Rust-Concurrency-Basics` | Rust 并发基础 | 本文件夹内 `*.rs` |
| `Chapter-02-Atomics` | 原子操作 | 本文件夹内 `*.rs` |
| `Chapter-03-Memory-Ordering` | 内存排序 | 待补 |
| `Chapter-04-Spin-Locks` | 构建自旋锁 | 待补 |
| `Chapter-05-Channels` | 构建通道 | 待补 |
| `Chapter-06-Custom-Arc` | 构建自定义 Arc | 待补 |
| `Chapter-07-Processors` | 理解处理器 | 待补 |
| `Chapter-08-OS-Primitives` | 操作系统原语 | 待补 |
| `Chapter-09-Custom-Locks` | 构建自定义锁 | 待补 |
| `Chapter-10-Advanced-Concurrent-Data-Structures` | 高级并发数据结构 | 待补 |
