# async_tokio — Async Rust 学习区

> **主文档** = 各章 **`本章学习笔记.md`**（章内**索引**；正文在各 `X.Y-slug.md`）  
> **小节 `X.Y-slug.md`** = 从主文档迁出的完整精读（非占位）  
> 规范：[../atomic/小节笔记与Demo规范.md](../atomic/小节笔记与Demo规范.md) · 对照：[章节与小节对照表.md](./章节与小节对照表.md)

## 何时建代码目录

**有可运行 `.rs` 才建 `X.Y-slug/`**；纯概念小节只保留 `.md`。

## 章节目录

| 章 | 文件夹 | 索引 |
|----|--------|------|
| 1 | [ch01_async_intro](./ch01_async_intro/) | [本章学习笔记.md](./ch01_async_intro/本章学习笔记.md) |
| 2 | [ch02_async_rust_core](./ch02_async_rust_core/) | 同上 |
| 3～11 | `ch03_*` … `ch11_*` | 各章 `本章学习笔记.md` |

## 维护脚本

```bash
python async_tokio/scripts/migrate_demo_dirs.py
python async_tokio/scripts/split_chapter_notes.py
```

## 示例路径（与书 § 对齐）

| 原根目录文件（已移除） | 新位置 |
|------------------------|--------|
| `ch01_reqwest_join.rs` | `ch01_async_intro/1.6-http-performance/1.6-http-performance-reqwest-join-demo.rs` |
| `ch02_counter_future.rs` | `ch02_async_rust_core/2.1-future-trait/2.1-future-trait-counter-demo.rs` |
| `ch03_join_macro_flume.rs` | `ch03_custom_task_queue/3.6-custom-join-macro/3.6-custom-join-macro-flume-demo.rs` |

完整对照见 [章节与小节对照表.md](./章节与小节对照表.md)。
