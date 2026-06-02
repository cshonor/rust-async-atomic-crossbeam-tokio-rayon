# Stage 09 — 附录与底层拔高

**对应书籍**：**附录（Appendix）** — 扩展与替代方案（资料中或称作「第 9 章」）

## 核心内容

1. 协程 / 生成器（无栈 vs 有栈、nightly `yield`）  
2. **`may`** / `may_minihttp`（有栈协程 HTTP）  
3. **`futures-await`** → 现代 **`async/await`**  
4. **`rayon`** 数据并行  
5. **`pest`** vs Ch4 **`nom`**  
6. **`bitflags`**、**`url`**  

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
| **[附录 — 学习笔记](notes/附录-扩展与替代方案-学习笔记.md)** | 全书附录精读 |

## 建议活动

- [ ] `async_tokio/ch10_noop_waker_block_on.rs`  
- [ ] `async_tokio/ch05_coroutines_generators/`  
- [ ] `cargo build --manifest-path atomic/rayon/Cargo.toml`  

## 学习检查

- [ ] 能说明 Tokio 与 rayon 各自解决什么问题  
- [ ] 能对比 nom 与 pest 一种即可  
