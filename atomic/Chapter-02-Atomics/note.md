# Chapter 02 — 原子操作（索引）

## 完整笔记

**[本章学习笔记.md](./本章学习笔记.md)** — 内存模型与数据竞争（跨 ch1/ch3 理论）+ 本章 **Load/Store、Fetch-Modify、CAS** 案例、与 `*.rs` 对照、日常规避法则、背诵卡。

## 本章目标

- 会用 `Atomic*` 的 `load`/`store`/`fetch_*`/`compare_exchange`  
- 区分 **数据竞争** vs **竞争初始化**、`Relaxed` 的适用范围  
- 知道 `fetch_add` 溢出与多原子变量中间态陷阱  

## 与本章代码的对应

- **模块**：`study_atomic::chapter_02` ← `atomic/src/mod.rs` `#[path]`。  
- **编号索引**：[README.md](./README.md)。  
- **短演示**：`study_atomic::demo()` → `quick_demo`；**全套**：`run_extended()`。

```bash
cargo build --manifest-path atomic/Cargo.toml
```
