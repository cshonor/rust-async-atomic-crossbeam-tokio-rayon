# Stage 09 — 附录与底层拔高

**对应书籍**：附录（协程、`Future`、解析器组合子等）

## 核心内容

- `Future` 状态机、`Waker`、执行器视角（与第 10 章无依赖栈对照）  
- 协程 / generator（若使用 nightly 实验）  
- 解析器组合子、与网络流式读写的结合

## 学习定位

- **与 LLVM / 编译优化、异步本质** 联动时的加深区。  
- 可与 `atomic/`、`async_tokio/ch02_async_rust_core/`、`ch10_*` 示例交叉阅读。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **中高（兴趣驱动）** |
| 是否必写 Demo | **选做**（手写 `poll`、最小 `block_on` 等） |

## 建议活动

- [ ] 对照 `async_tokio/ch10_dependency_free_async_server/ch10_noop_waker_block_on.rs` 理解驱动循环  
- [ ] 对照 `async_tokio/ch05_coroutines_generators/` 与书中协程叙述  
- [ ] （可选）独立 crate 实验 nightly generator  

## 笔记区

（在此补充链接、摘录或 `notes.md`）
