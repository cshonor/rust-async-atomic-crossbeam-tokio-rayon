# 小节 .md 只做索引，完整正文一律在 本章学习笔记.md

> **权威笔记** = 各章 **`本章学习笔记.md`**（完整精读，不可删改缩写）  
> **小节 `X.Y-slug.md`** = 仅索引：指向本章学习笔记对应节 + demo 路径  
> **代码** = 有 demo 时才有 `X.Y-slug/` 目录

---

## 命名规则

| 类型 | 格式 | 示例 |
|------|------|------|
| **本章学习笔记** | `本章学习笔记.md` | 章内完整正文（主文档） |
| **小节索引** | `X.Y-english-slug.md` | `1.3-thread-spawn.md` → 链到本章 §3 |
| **代码目录** | `X.Y-english-slug/` | `1.3-thread-spawn/`（**有 `.rs` 才建**） |
| **Demo 文件** | `X.Y-english-slug-描述-demo.rs` | `1.3-thread-spawn-join-demo.rs` |

**禁止**：用十几行的「占位精读」替代 `本章学习笔记.md` 中的正文。

---

## 目录树

```
Chapter-01-Rust-Concurrency-Basics/
├── 本章学习笔记.md          ← 完整正文（主文档）
├── mod.rs
├── 1.3-thread-spawn.md      ← 仅索引 + demo 链接
├── 1.3-thread-spawn/
│   └── 1.3-thread-spawn-join-demo.rs
└── …
```

---

## mod.rs（只在章根）

```rust
#[path = "1.3-thread-spawn/1.3-thread-spawn-join-demo.rs"]
pub mod usejoin;
```

---

## async_tokio

与 atomic 相同：`本章学习笔记.md` 为主文档；`1.Y-slug.md` 为索引。

路径批量更新：`atomic/scripts/update-chapter-note-paths.ps1` · `async_tokio/scripts/update-chapter-note-paths.ps1`
