# Chapter 09 — 构建我们自己的锁（Building Our Own Locks）

**对应书籍**：第 9 章 — **`atomic_wait` + 状态机** 手写 **Mutex、Condvar、RwLock**（**有锁**，非无锁）。

> **无锁** → [第 10 章](../Chapter-10-Advanced-Concurrent-Data-Structures/本章学习笔记.md) · [无锁编程-贯通笔记.md](../无锁编程-贯通笔记.md)

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | **索引 + 整体脉络**、§9.1–9.4 子笔记表 |
| **[9.3.2 写优先 RwLock](./9.3.2-writer-starvation-state-machine.md)** | 完整状态机与代码骨架 |
| **[互斥锁与锁体系-贯通笔记.md](../互斥锁与锁体系-贯通笔记.md)** | Mutex 三态 §5 |

## 阅读顺序

**9.1 Mutex**（9.1.1→9.1.4）→ **9.2 Condvar**（9.2.1→9.2.2）→ **9.3 RwLock**（9.3.1→9.3.3）→ **9.4 总结**

## 前置

- [第 4 章 自旋锁](../Chapter-04-Spin-Locks/本章学习笔记.md)  
- [第 8 章 OS 原语](../Chapter-08-OS-Primitives/本章学习笔记.md)  
- [第 1 章 Mutex/Condvar/RwLock 用法](../Chapter-01-Rust-Concurrency-Basics/1.7-mutex-rwlock/1.7-mutex-rwlock.md)

## 说明

书中依赖 **`atomic_wait`** crate；仓库笔记以**状态机阅读**为主，demo 待补。日常 **优先标准库**。
