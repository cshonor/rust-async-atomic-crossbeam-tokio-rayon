# Chapter 01 — Rust 并发基础（索引）

## 完整笔记

**[本章学习笔记.md](./本章学习笔记.md)** — 《Rust Atomics and Locks》第 1 章：线程模型、数据竞争与借用、`spawn`/`scope`/`Arc`、内部可变性与 `Send`/`Sync`、`Mutex`/`Condvar`/park、术语、示例对照、易错点与背诵卡。

## 本章目标

- 理解进程/线程与共享内存模型  
- 会用 `spawn`、`scope`、`Arc`、`Mutex`、`Condvar` 写出安全的多线程代码  
- 建立 `Send`/`Sync`、锁粒度、虚假唤醒等面试级概念  

## 与本章代码的对应

- **crate 模块**：`study_atomic::chapter_01`，由 `atomic/src/mod.rs` 的 `#[path]` 引入本目录 `mod.rs`。  
- **编号索引**：见 [README.md](./README.md)。  
- **默认长演示**：`study_atomic::run_ex1_default()`（当前指向 `use_condvar`；可在 `atomic/src/mod.rs` 切换）。

```bash
cargo build --manifest-path atomic/Cargo.toml
cargo doc --manifest-path atomic/Cargo.toml --no-deps --open
```
