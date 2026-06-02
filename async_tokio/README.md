# async_tokio — Async Rust 学习区

> **小节 `X.Y-slug.md`** = 该节**完整精读**（概念、表格、代码片段、与书的对应）  
> **`本章学习笔记.md`** = 章内索引表（链到各节 + demo 目录）  
> **`X.Y-slug/`** = 每节**至少一个** `*-demo.rs`（与书 § 对齐）  
> 规范：[../atomic/小节笔记与Demo规范.md](../atomic/小节笔记与Demo规范.md) · 对照：[章节与小节对照表.md](./章节与小节对照表.md)

## 目录约定

```
ch11_async_testing_debugging/
├── 本章学习笔记.md              ← 索引（8 节）
├── 11.3-testing-for-deadlocks.md  ← 精读正文
└── 11.3-testing-for-deadlocks/
    └── 11.3-testing-for-deadlocks-timeout-demo.rs
```

## 如何运行 Demo

**仅需 std**（第 10 章部分、占位小节）：

```bash
cd async_tokio/ch10_dependency_free_async_server/10.2-building-std-async-runtime
rustc 10.2-building-std-async-runtime-noop-waker-demo.rs
```

**需要 Tokio**（多数章节）：在仓库根 `atomic/` 工程已含 Tokio 时，可将 demo 拷入 `examples/`，或在本机：

```bash
# 单文件（需本机已 cargo init 且 Cargo.toml 含 tokio）
rustc --edition 2021 --crate-type bin your-demo.rs  # 通常仍用 cargo 更省事
```

推荐：为常用 demo 在 `atomic/Cargo.toml` 增加 `[[example]]` 指向 `../async_tokio/...`（可按需自行添加）。

## 维护脚本

```bash
python async_tokio/scripts/scaffold-missing-demos.py   # 为缺 demo 的小节建目录
python async_tokio/scripts/update-index-demos.py       # 刷新各章索引表的 Demo 列
python async_tokio/scripts/split_chapter_notes.py      # 旧笔记拆分（已对齐的章会自动 skip）
```

## 章节目录

| 章 | 文件夹 | 索引 |
|----|--------|------|
| 1–11 | `ch01_async_intro` … `ch11_async_testing_debugging` | 各章 `本章学习笔记.md` |

完整 § 对照见 [章节与小节对照表.md](./章节与小节对照表.md)。
