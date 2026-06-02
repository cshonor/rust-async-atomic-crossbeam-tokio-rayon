# Stage 09 — 附录与底层拔高

**对应书籍**：**Appendix**（附录；资料中常称第 9 章）· §9.1～9.6

## 章节目录（原书）

| 书 § | 主题 |
|------|------|
| 9.1 | Introduction to coroutines and generators |
| 9.1.1 | How May handles coroutines |
| 9.2 | Awaiting the future |
| 9.3 | Data parallelism |
| 9.4 | Parsing using Pest |
| 9.5 | Miscellaneous utilities（bitflags、url） |
| 9.6 | Summary |

## 学习定位

- **兴趣驱动加深**：与 `async_tokio/`、`atomic/rayon/` 交叉阅读。  
- 不必阻塞主线（stage03 + 07）。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **中高（选读）** |
| 是否必写 Demo | **选做** |

## 笔记

| 资料 | 说明 |
|------|------|
| **[本章学习笔记](本章学习笔记.md)** | §9.1～9.6 索引表 |
| **[Ch09 附录 — 学习笔记](notes/Ch09-附录-学习笔记.md)** | 全书附录精读 |

## 建议活动

- [ ] `async_tokio/ch10_noop_waker_block_on.rs`  
- [ ] `async_tokio/ch05_coroutines_generators/`  
- [ ] `cargo build --manifest-path atomic/rayon/Cargo.toml`  

## 学习检查

- [ ] 能说明 Tokio 与 rayon 各自解决什么问题  
- [ ] 能对比 nom 与 pest 一种即可  
