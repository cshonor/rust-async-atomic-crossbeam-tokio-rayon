# Chapter 03 — 内存排序（Memory Ordering）

**对应书籍**：第 3 章 — `std::sync::atomic::Ordering` 与 happens-before。

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | Relaxed / Release / Acquire / SeqCst、选型、易错点 |
| **[Atomics与内存序-贯通笔记.md](../Atomics与内存序-贯通笔记.md)** | 第 2～3 章合一（普通 vs 原子、API、Ordering、规范） |

## 第 4 章衔接

五种 `Ordering` 的**工程样板**：**自旋锁**（Acquire 加锁 / Release 解锁）→ [Chapter-04-Spin-Locks/本章学习笔记.md](../Chapter-04-Spin-Locks/本章学习笔记.md)

## 配套代码

本章概念在 **`Chapter-02-Atomics/`** 中演示（第 3 章常与第 2 章同卷练习）：

| 文件 | 内容 |
|------|------|
| `use_atomic.rs` | Relaxed、Release-Acquire |
| `use_fence.rs` | `fence` |
| `use_seqcst.rs` | SeqCst |

## 优先级

| 项目 | 建议 |
|------|------|
| 精读 | Release/Acquire、Relaxed 边界、何时不能 Relaxed |
| 浏览后定 | SeqCst、fence（先掌握 RA 再扩展） |
