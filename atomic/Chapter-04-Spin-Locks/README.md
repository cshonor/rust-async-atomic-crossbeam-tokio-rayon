# Chapter 04 — 构建自旋锁（Spin Lock）

**对应书籍**：第 4 章 — **Acquire / Release** 的经典应用（理论见第 3 章）。

## 学习材料

| 文档 | 说明 |
|------|------|
| **[本章学习笔记.md](./本章学习笔记.md)** | 自旋锁结构、HB、选型、易错点 |
| **[Chapter-03 内存排序](../Chapter-03-Memory-Ordering/本章学习笔记.md)** | 五种 `Ordering` 完整理论 |

## 配套代码

| 文件 | 说明 |
|------|------|
| **`spin_lock.rs`** | `SpinLock<T>` + `Guard` + `demo()`（4 线程累加） |
| **`mod.rs`** | `chapter_04::demo()` |

挂接到 `study_atomic`：在 `atomic/src/mod.rs` 增加：

```rust
#[path = "../Chapter-04-Spin-Locks/mod.rs"]
pub mod chapter_04;
```

## 运行

```bash
cargo build --manifest-path atomic/Cargo.toml
# 挂接后：在临时 bin 或测试中调用 study_atomic::chapter_04::demo()
```
