# Part 02 — 源码到机器码（核心主攻）

| 目录 | 章节 | 策略 |
|------|------|------|
| `chapter03_ast_build` | 第 3 章 AST | **浏览**：懂词法/语法分析原理即可 |
| `chapter04_ir_basic` | 第 4 章 IR / SSA | **精读**：LLVM IR 语法与基本块 |
| `chapter05_ir_advanced_type` | 第 5 章 复合类型与调用 | **精读**：与 Rust 类型 lowering 对照 |
| `chapter06_ir_extend` | 第 6 章 异常与调试元数据 | **浏览**：知道存在即可 |
| `chapter07_ir_optimize` | 第 7 章 优化流水线 | **精读**：与原子/内存屏障约束对照 |

**统一 IR 实验 crate**：本目录上一级的 `llvm_insight/Cargo.toml` + `src/lib.rs`（包名 `llvm_insight_lab`），导出命令见根 `llvm_insight/README.md`。

从 `atomic/`、`async_tokio/`、`rust_network_programming` 相关 Rust 抄最小复现进 `src/lib.rs` 后，将生成的 `.ll` **复制到** `ir_samples/` 下对应子目录。
