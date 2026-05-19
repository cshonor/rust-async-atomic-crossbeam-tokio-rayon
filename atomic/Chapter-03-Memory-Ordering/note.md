# Chapter 03 — 内存排序（索引）

## 完整笔记

- **[本章学习笔记.md](./本章学习笔记.md)** — Ordering 语义、选型、易错点  
- **[Atomics与内存序-贯通笔记.md](../Atomics与内存序-贯通笔记.md)** — 与第 2 章 API 合一总览  

## 本章目标

- 能解释 **Relaxed / Release / Acquire / SeqCst** 差异  
- 能判断何时原子变量是「信使」、必须用 RA  
- 知道 **SeqCst** 不是默认答案  

## 代码位置

实现与演示在 **`../Chapter-02-Atomics/`** 的 `use_atomic.rs`、`use_fence.rs`、`use_seqcst.rs`。
