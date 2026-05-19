# Chapter 01 - 线程与同步原语（ex1）

本章聚焦 **线程基础** 与 **同步原语**：`JoinHandle`、`thread::scope`、`Rc/Arc`、`Mutex/Condvar`、`Cell/RefCell`、以及 `Send/Sync` 的边界。

**完整梳理（考试/面试向）**：[本章学习笔记.md](./本章学习笔记.md)

## 如何运行

- 运行本章默认入口（原 ex1，较长演示，当前默认是条件变量示例）：

```bash
cargo run -- atomic ex1
```

> 说明：各个示例文件里通常都提供 `pub fn main()`；如果你想在 ex1 里切换默认演示入口，可以调整 **`src/mod.rs`** 里 `run_ex1_default()` 调用的目标。

## 编号索引（文件名即主题）

| 编号 | 模块（文件） | 一句话概览 |
|------|--------------|------------|
| 01 | `move_closure.rs` | 闭包捕获与 `move` 在线程中的使用 |
| 02 | `thread_advanced.rs` | 更进阶的线程用法与注意点（见文件内分段示例） |
| 03 | `thread_example.rs` | 基础线程示例集合 |
| 04 | `use_cell_refcell.rs` | `Cell`/`RefCell`：内部可变性与运行时借用检查 |
| 05 | `use_condvar.rs` | `Condvar`：等待/唤醒、超时等待、`notify_one/all` |
| 06 | `use_mutex.rs` | `Mutex`：`lock()`、`MutexGuard`、多线程累加等 |
| 07 | `use_mutex_guard_lifetime.rs` | `MutexGuard` 生命周期与“锁持有过久”的常见坑 |
| 08 | `use_rc_arc.rs` | `Rc` vs `Arc`：引用计数与线程安全边界 |
| 09 | `use_send_sync.rs` | `Send`/`Sync`：哪些类型能跨线程转移/共享 |
| 10 | `useboxleak.rs` | `Box::leak` 用法示例 |
| 11 | `usecall.rs` | 基础调用/线程示例（见文件内实现） |
| 12 | `usejoin.rs` | `join` 等待线程结束（见文件内实现） |
| 13 | `userecall.rs` | 回调/闭包相关示例（见文件内实现） |
| 14 | `usescope.rs` | `thread::scope`：借用跨线程的安全方式 |
| 15 | `usestatic.rs` | `'static` 与线程相关示例（见文件内实现） |

