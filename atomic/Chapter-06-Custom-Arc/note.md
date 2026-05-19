# Chapter 06 — 构建自定义 Arc（索引）

## 重要：本章不是 Mutex

书中 **第 6 章** 主题是 **`Arc` 底层**（原子引用计数、控制块、弱引用等），与 **互斥锁** 无关。

互斥锁体系见：**[互斥锁与锁体系-贯通笔记.md](../互斥锁与锁体系-贯通笔记.md)**

## 本章目标（待展开）

- 用 `AtomicUsize` 等实现引用计数增减  
- 理解 `Arc::clone` / `Drop` 与内存序  
- 与 `Chapter-02` 的 `fetch_add`、CAS 对照

## 代码

待在本目录补充 `custom_arc.rs` 并挂接 `study_atomic::chapter_06`。
