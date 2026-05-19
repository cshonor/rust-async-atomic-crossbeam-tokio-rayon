# Chapter 05 — 构建 Channel（One-Shot）

**对应书籍**：第 5 章 — 用原子 + `MaybeUninit` + park/unpark 实现单次通道。

## 学习材料

- **[本章学习笔记.md](./本章学习笔记.md)**
- **[CAS与Fetch-Modify专题.md](../Chapter-02-Atomics/CAS与Fetch-Modify专题.md)**

## 代码

| 文件 | 说明 |
|------|------|
| **`one_shot_channel.rs`** | `Channel` / `Sender` / `Receiver` + `demo()` |
| **`mod.rs`** | `study_atomic::chapter_05::demo()` |

```bash
cargo build --manifest-path atomic/Cargo.toml
```
