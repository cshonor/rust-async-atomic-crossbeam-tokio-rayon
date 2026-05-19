# Chapter 08 — 操作系统原语（Operating System Primitives）

**对应书籍**：第 8 章 — **futex**、`pthread_cond` / `pthread_mutex`、Windows **`WaitOnAddress`** 等。

## 学习材料

- **[本章学习笔记.md](./本章学习笔记.md)** — futex / POSIX / Windows 与 Rust 同步原语对应  
- **[Condvar与条件变量-贯通笔记.md](../Condvar与条件变量-贯通笔记.md)** — Condvar **用法**在第 1 章；**底层**在本章  
- **[互斥锁与锁体系-贯通笔记.md](../互斥锁与锁体系-贯通笔记.md)** — Mutex 阻塞与三态优化

## 重要区分

| 主题 | 章节 |
|------|------|
| **OS 原语是什么** | **第 8 章（本章）** |
| **Condvar 怎么写** | **第 1 章** + `use_condvar.rs` |
| **虚假唤醒 / 惊群** | **第 1 + 9 章**（见 Condvar 贯通笔记） |
