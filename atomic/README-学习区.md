# Rust Atomics and Locks — 章节学习区

本目录 **`atomic/`** 作为**总区**：与 **`Cargo.toml`**、**`src/`** 平级，下面直接是 **10 个英文命名的 `Chapter-…` 文件夹**。

## 两块内容怎么分

| 位置 | 用途 |
|------|------|
| **`src/mod.rs`** | `study_atomic` 库入口：用 **`#[path = "../Chapter-01-…/mod.rs"]`** 等方式，把第 1、2 章文件夹挂成 **`chapter_01` / `chapter_02`** 模块。 |
| **`Chapter-01`～`Chapter-10`** | **主文档** = 各章 **`本章学习笔记.md`**（完整精读）；**书目录** = [全书目录-与实体书一致.md](./全书目录-与实体书一致.md)；**索引** = `X.Y-slug.md`；**有 demo 时**才有 **`X.Y-slug/`** |
| **`Chapter-02-Atomics/`** | 第 2 章：**`本章学习笔记.md`**（原子 API + 内存模型/DR 理论）+ **`note.md`** + 全部 `*.rs`。 |
| **`Chapter-03`～`Chapter-08`、`Chapter-10`** | 多数已有 **`本章学习笔记.md`**；无锁见第 10 章与 [无锁编程-贯通笔记.md](./无锁编程-贯通笔记.md)。 |

构建：`cargo build -p study_atomic` 或根目录 `cargo run -- atomic`。

**跨章专题笔记**：[互斥锁与锁体系-贯通笔记.md](./互斥锁与锁体系-贯通笔记.md) · [RwLock与读写锁体系-贯通笔记.md](./RwLock与读写锁体系-贯通笔记.md) · [Condvar与条件变量-贯通笔记.md](./Condvar与条件变量-贯通笔记.md) · [无锁编程-贯通笔记.md](./无锁编程-贯通笔记.md) · [Atomics与内存序-贯通笔记.md](./Atomics与内存序-贯通笔记.md)

## 章节目录（文件夹名 = 英文）

| 文件夹名 | 主题（中文） | 源码 |
|----------|----------------|------|
| `Chapter-01-Rust-Concurrency-Basics` | Rust 并发基础 | 本文件夹内 `*.rs` |
| `Chapter-02-Atomics` | 原子操作 | 本文件夹内 `*.rs` |
| `Chapter-03-Memory-Ordering` | 内存排序 | **`本章学习笔记.md`** + [贯通笔记](./Atomics与内存序-贯通笔记.md)；代码见 **Chapter-02** 的 `use_atomic` / `use_fence` / `use_seqcst` |
| `Chapter-04-Spin-Locks` | 构建自旋锁 | **`spin_lock.rs`** + **`本章学习笔记.md`**（Acquire/Release 实战）；`study_atomic::chapter_04` |
| `Chapter-05-Channels` | 构建通道 | **`one_shot_channel.rs`** + **`本章学习笔记.md`**；`study_atomic::chapter_05` |
| `Chapter-06-Custom-Arc` | **构建自定义 Arc**（非 Mutex） | `README.md` 澄清；实现待补 |
| `Chapter-07-Processors` | **理解处理器**（缓存/屏障，非 RwLock） | `README.md`；与锁性能见贯通笔记 |
| `Chapter-08-OS-Primitives` | **OS 同步原语**（futex/pthread/Windows） | [本章学习笔记.md](./Chapter-08-OS-Primitives/本章学习笔记.md) · [Condvar与条件变量-贯通笔记.md](./Condvar与条件变量-贯通笔记.md) |
| `Chapter-09-Custom-Locks` | **手写 Mutex/Condvar/RwLock**（有锁） | [本章学习笔记.md](./Chapter-09-Custom-Locks/本章学习笔记.md) · 各锁贯通笔记 |
| `Chapter-10-Advanced-Concurrent-Data-Structures` | **全书收尾**：进阶模式、无锁、设计/优化/排错/面试 | [本章学习笔记.md](./Chapter-10-Advanced-Concurrent-Data-Structures/本章学习笔记.md) · [无锁编程-贯通笔记.md](./无锁编程-贯通笔记.md) |
