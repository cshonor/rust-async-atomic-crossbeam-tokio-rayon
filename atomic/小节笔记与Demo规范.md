# 小节笔记与 Demo 规范

---

## atomic（以第一章为范本）

| 文件 | 作用 |
|------|------|
| `本章学习笔记.md` | **章索引表**（书 §1.1～1.9 → 笔记 → demo） |
| `1.Y-english-slug/1.Y-english-slug.md` | **该节索引**；子笔记 `1.Y.Z-*.md` 同目录（标题用 `## 1.Y …`，与书编号一致） |
| `1.Y-english-slug/code/*.rs` | Demo；由章根 `mod.rs` 用 `#[path = "..."]` 挂接 |

**唯一编号**：书 § = 文件名前缀 = 代码目录名。例如书 §1.7 → `1.7-mutex-rwlock.md` + `1.7-mutex-rwlock/`。

**禁止**：

- 同一节两套 slug（如同时保留 `1.7-send-sync` 与 `1.6-send-sync`）
- 旧稿内部编号（`§3.2`、`完整正文见本章学习笔记 §4`）出现在小节文件里
- 「仅索引、正文在章笔记」的占位 `.md` 与实体书 9 节正文文件并存

```
Chapter-01-Rust-Concurrency-Basics/
├── 本章学习笔记.md
├── mod.rs
├── 1.1-threads-in-rust/
│   ├── 1.1-threads-in-rust.md   ← §1.1 索引
│   ├── 1.1.1-spawn-join-basics.md
│   └── code/
│       └── 1.1-threads-in-rust-join-demo.rs
├── 1.7-mutex-rwlock/
│   ├── 1.7-mutex-rwlock.md
│   ├── 1.7.1-mutex-motivation.md
│   └── code/
│       └── 1.7-mutex-rwlock-mutex-demo.rs
└── …（§1.2～1.9 同结构：每节一文件夹，索引 + 子笔记 + code/ demo）
```

---

## async_tokio

| 文件 | 角色 |
|------|------|
| `本章学习笔记.md` | **章索引表**（§ → 精读 `.md` → demo 目录） |
| `X.Y-slug.md` | **该节完整精读** |
| `X.Y-slug/*-demo.rs` | **每节至少一个**可运行示例 |

维护：`async_tokio/scripts/scaffold-missing-demos.py` · `update-index-demos.py`
