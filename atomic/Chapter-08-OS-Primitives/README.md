# Chapter 08 — 操作系统原语（Operating System Primitives）

**对应书籍**：第 8 章 — **futex**、`pthread_cond` / `pthread_mutex`、Windows **`WaitOnAddress`** 等。

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | **索引 + 整体脉络**、§8.1–8.4 子笔记表 |
| **[Condvar与条件变量-贯通笔记.md](../Condvar与条件变量-贯通笔记.md)** | Condvar **用法**在第 1 章；**底层**在本章 |
| **[互斥锁与锁体系-贯通笔记.md](../互斥锁与锁体系-贯通笔记.md)** | Mutex 阻塞与三态优化 |

## 阅读顺序

**8.1 futex**（8.1.1→8.1.4）→ **8.2 parks**（8.2.1→8.2.2）→ **8.3 OS cond**（8.3.1→8.3.3）→ **8.4 总结**

## 重要区分

| 主题 | 章节 |
|------|------|
| **OS 原语是什么** | **第 8 章（本章）** |
| **Condvar 怎么写** | **第 1 章 §1.8** + [1.8-parking-condvar-demo.rs](../Chapter-01-Rust-Concurrency-Basics/1.8-parking-condvar/code/1.8-parking-condvar-demo.rs) |
| **虚假唤醒 / 惊群 / 手写 wait** | **第 1 + 9 章** |

## 下一章

[第 9 章 — 构建锁](../Chapter-09-Custom-Locks/本章学习笔记.md) — `atomic_wait` 封装本章 WAIT/WAKE
