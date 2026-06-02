# async_tokio — Async Rust 学习区

> **主文档** = 各章 **`本章学习笔记.md`**（完整精读，不可删改缩写）  
> **小节 `X.Y-slug.md`** = 仅索引，指向本章学习笔记对应节 + demo 路径  
> 规范：[../atomic/小节笔记与Demo规范.md](../atomic/小节笔记与Demo规范.md)

## 何时建代码目录

**有可运行 `.rs` 才建 `X.Y-slug/`**；纯概念小节只保留 `.md` 索引。

## 章节目录

| 章 | 文件夹 | 主文档 |
|----|--------|--------|
| 1 | [ch01_async_intro](./ch01_async_intro/) | [本章学习笔记.md](./ch01_async_intro/本章学习笔记.md) |
| 2 | [ch02_async_rust_core](./ch02_async_rust_core/) | 同上 |
| 3～11 | `ch03_*` … `ch11_*` | 各章 `本章学习笔记.md` |

## 示例路径（节选）

| 原文件 | 新位置 |
|--------|--------|
| `ch01_reqwest_join.rs` | `ch01_async_intro/1.6-http-performance/1.6-http-performance-reqwest-join-demo.rs` |
| `ch02_counter_future.rs` | `ch02_async_rust_core/2.1-future-trait/2.1-future-trait-counter-demo.rs` |

脚本：`scripts/scaffold-sections.ps1` · `scripts/update-chapter-note-paths.ps1`
