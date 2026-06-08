# Chapter 03 — 内存排序（Memory Ordering）

**对应书籍**：第 3 章 — `std::sync::atomic::Ordering` 与 happens-before。

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | **索引 + 整体脉络**、学习路径、Demo 一览 |
| **[3.3-memory-order-options.md](./3.3-memory-order-options.md)** | 索引；子节 **3.3.1–3.3.5** |
| **[3.3.5-ordering-compare-select.md](./3.3.5-ordering-compare-select.md)** | 五种 Ordering **对照表 + 场景速查** |
| **[Atomics与内存序-贯通笔记.md](../Atomics与内存序-贯通笔记.md)** | 第 2～3 章合一（普通 vs 原子、API、Ordering、规范） |

## 阅读顺序

**3.1 重排** → **3.2 HB / 内存模型** → **3.3 Ordering** → **3.4 fence** → **3.5 误区** → **3.6 总结**

## 第 4 章衔接

五种 `Ordering` 的**工程样板**：**自旋锁**（Acquire 加锁 / Release 解锁）→ [Chapter-04-Spin-Locks/本章学习笔记.md](../Chapter-04-Spin-Locks/本章学习笔记.md)

## 配套代码

| 文件 | 内容 |
|------|------|
| [2.1-atomic-load-store-demo.rs](../Chapter-02-Atomics/2.1-atomic-load-store/2.1-atomic-load-store-demo.rs) | Relaxed、Release-Acquire |
| [3.3-memory-order-options-demo.rs](./3.3-memory-order-options/3.3-memory-order-options-demo.rs) | SeqCst 等 |
| [3.4-fences-demo.rs](./3.4-fences/3.4-fences-demo.rs) | `fence` |
| [use_atomic.rs](../Chapter-02-Atomics/use_atomic.rs) | 贯通 demo 入口 |

## 优先级

| 项目 | 建议 |
|------|------|
| 精读 | **Happens-Before**、Release/Acquire、Relaxed 边界 |
| 必背 | [3.3 对照表](./3.3-memory-order-options.md#对照表必背) |
| 浏览后定 | SeqCst、fence（先掌握 RA 再扩展） |
