# Chapter 04 — 构建自旋锁（索引）

## 完整笔记

**[本章学习笔记.md](./本章学习笔记.md)** — 自旋锁 + Acquire/Release 实战（第 3 章理论的应用）

## 本章目标

- 理解 **Acquire 加锁 / Release 解锁** 如何保护 `UnsafeCell` 内数据  
- 能读懂书中 `SpinLock` 结构，并解释为何不用 Relaxed  
- 知道 **swap + spin_loop** 与 CAS 自旋的差异（缓存争用）

## 代码

- **`spin_lock.rs`**、`mod.rs` — 见 [README.md](./README.md)
