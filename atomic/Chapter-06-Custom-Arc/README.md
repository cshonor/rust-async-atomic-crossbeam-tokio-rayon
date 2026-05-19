# Chapter 06 — 构建我们自己的 Arc

**对应书籍**：第 6 章 — **原子引用计数**（`AtomicUsize` / `fetch_add` / `compare_exchange` 等），**不是**互斥锁专章。

## 与「锁」笔记区分

- **Mutex / RwLock / 自旋锁 / 工业锁优化** → [互斥锁与锁体系-贯通笔记.md](../互斥锁与锁体系-贯通笔记.md)（跨第 1、4、8、9 章）  
- **本章** → 如何用原子操作实现 **`Arc` 式** 共享所有权

## 前置

- [Chapter-02-Atomics/CAS与Fetch-Modify专题.md](../Chapter-02-Atomics/CAS与Fetch-Modify专题.md)  
- [Chapter-03-Memory-Ordering/本章学习笔记.md](../Chapter-03-Memory-Ordering/本章学习笔记.md)

## 状态

| 项目 | 说明 |
|------|------|
| 源码 | 待按书实现 `CustomArc` 等 |
| 笔记 | `note.md` 占位 |
