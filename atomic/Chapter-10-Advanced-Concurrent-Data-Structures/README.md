# Chapter 10 — Ideas and Inspiration（全书收尾）

**对应书籍**：第 10 章 — **进阶模式**、**无锁**、**设计 / 优化 / 排错 / 面试**。

## 重要：本章 ≠ 第 9 章

| 章 | 主题 |
|----|------|
| **9** | 手写 **Mutex / Condvar / RwLock**（有锁） |
| **10** | **信号量、RCU、无锁栈、Parking lot、SeqLock** + 全书收尾 |

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | **索引 + 整体脉络**、§10.1–10.7 子笔记表 |
| **[无锁编程-贯通笔记.md](../无锁编程-贯通笔记.md)** | 速览导航 |

## 阅读顺序

**10.1 信号量** → **10.2 RCU** → **10.3 无锁栈** → **10.4 Parking lot** → **10.5 阻塞锁** → **10.6 SeqLock** → **10.7 设计/优化/面试**

## Demo

| 节 | 文件 |
|----|------|
| 10.1 | [10.1-semaphores-demo.rs](./10.1-semaphores/10.1-semaphores-demo.rs) |
| 10.2 | [10.2-rcu-pointer-swap-demo.rs](./10.2-rcu/10.2-rcu-pointer-swap-demo.rs) |
| 10.3 | [10.3-lock-free-stack-push-demo.rs](./10.3-lock-free-linked-list/10.3-lock-free-stack-push-demo.rs) |

## 前置

- [第 2～3 章](../Chapter-02-Atomics/本章学习笔记.md) CAS / Ordering  
- [第 9 章](../Chapter-09-Custom-Locks/本章学习笔记.md) 有锁手写  

## 生态

生产无锁结构优先 **`crossbeam-queue`**、`crossbeam-epoch`；锁可用 **`parking_lot`**。
