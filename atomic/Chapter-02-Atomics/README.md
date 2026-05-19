# Chapter 02 - 原子与内存序（ex2）

本章聚焦标准库原子类型 `std::sync::atomic`：**内存序（Ordering）**、**原子操作方法**、**延迟初始化**、**ID 分配器**、**SeqCst 与 fence** 等。

**完整梳理（理论 + API + 案例）**：[本章学习笔记.md](./本章学习笔记.md)（含跨 ch1/ch3 的内存模型与数据竞争定义）  
**第 2～3 章贯通**：[Atomics与内存序-贯通笔记.md](../Atomics与内存序-贯通笔记.md)  
**CAS / Fetch-Modify 专题**：[CAS与Fetch-Modify专题.md](./CAS与Fetch-Modify专题.md)

## 如何运行

- 运行原子模块短概览（quick demo）：

```bash
cargo run -- atomic
```

- 运行本章全套演示（原 ex2，较长）：

```bash
cargo run -- atomic ex2
```

> 说明：ex2 的执行顺序由 **`src/mod.rs`** 的 `run_extended()` 决定；每个文件里也通常有 `pub fn main()` 作为单文件入口。

## 编号索引（文件名即主题）

| 编号 | 模块（文件） | 一句话概览 |
|------|--------------|------------|
| 01 | `id_allocator.rs` | 三种 ID 分配策略：`fetch_add`、溢出回滚、`compare_exchange` |
| 02 | `lazy_init.rs` | 延迟初始化：不安全版本 vs `compare_exchange` 安全版本 |
| 03 | `quick_demo.rs` | 原子类型快速概览：`load/store/fetch_add/compare_exchange` |
| 04 | `use_atomic.rs` | `AtomicBool` 停止标志与 `Relaxed` / `Release-Acquire` 示例 |
| 05 | `use_atomic_operations.rs` | `fetch_*`、`swap`、按位操作、max/min 等原子方法 |
| 06 | `use_fence.rs` | 栅栏（fence）：单线程/多线程语义与与 RA/SeqCst 对比 |
| 07 | `use_seqcst.rs` | `SeqCst`：全局顺序一致性与典型适用场景 |

