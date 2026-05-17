# Rust 并发、异步、网络与 LLVM IR — 个人学习仓库

本仓库把 **《Rust Atomics and Locks》路线（原子与锁）**、**Tokio / 异步 Rust 笔记与示例**、**《Network Programming with Rust》分阶段路线**，以及 **LLVM IR 透视（《Learn LLVM 17》取舍 + 归档）** 放在**并列目录**里，职责清晰、互不塞进同一 crate。

仓库地址：<https://github.com/cshonor/rust-async-atomic-crossbeam-tokio-rayon>

---

## 你将找到什么

| 板块 | 路径 | 说明 |
|------|------|------|
| **原子与并发基础** | `atomic/` | Crate **`study_atomic`**：`chapter_01`（线程、`Mutex` / `Condvar`、`Arc`…）、`chapter_02`（`Atomic*`、内存序、栅栏…）。源码按章在 `Chapter-01-*`、`Chapter-02-*` 等目录，`#[path]` 由 `atomic/src/mod.rs` 引入。 |
| **crossbeam / rayon 小示例** | `atomic/crossbeam/`、`atomic/rayon/` | 独立 crate：**`study_crossbeam`**、**`study_rayon`**（与书名/主线的对照副本，可单独 `cargo build`）。 |
| **异步 Rust（袋鼠书向）** | `async_tokio/` | 按章 **`本章学习笔记.md`**、`demo.rs`，以及可配合主工程运行的 **`ch*_*.rs`** 示例源文件（依赖 Tokio / reqwest / mio 等，见各文件顶部说明）。 |
| **网络书路线** | `rust_network_programming/` | **非 crate**：`stage01`～`stage09` 分阶段 `README.md` 模板与优先级说明。 |
| **LLVM IR 与《Learn LLVM 17》** | `llvm_insight/` | **Part 01–04** 章节目录 + **`ir_samples/`** 归档约定；根下 **`llvm_insight_lab`** 小 crate 用于 **`--emit=llvm-ir`**。总览见 `llvm_insight/README.md`，取舍见 `llvm_insight/Learn-LLVM-17-学习取舍.md`。 |

---

## 推荐学习顺序（与目录设计一致）

1. **`atomic/`** — 先把同步世界与 **内存模型 / 原子** 打牢。  
2. **`atomic/crossbeam/`、`atomic/rayon/`** — 通道与数据并行，和第一章线程模型对照。  
3. **`async_tokio/`** — 在已有并发直觉上读 Tokio 与异步控制流。  
4. **`rust_network_programming/`** — 按 `stage` 优先级补 **Socket / 协议 / 工程化网络**。  
5. **`llvm_insight/`** — 用已有 Rust 代码 **反查 IR 与优化**（不必早于网络强行学完）。

---

## 目录结构（与当前仓库一致）

> **说明**：当前快照**根目录没有** `Cargo.toml`；可编译单元分布在 `atomic/`、`atomic/crossbeam/`、`atomic/rayon/`、`llvm_insight/`。若你本地另有「根 workspace + 主程序」的 fork，以该版本根目录文档为准。

```
.
├── README.md
├── atomic/                          # crate：study_atomic
│   ├── Cargo.toml
│   ├── README-学习区.md
│   ├── src/mod.rs                   # #[path] → 各 Chapter/mod.rs
│   ├── Chapter-01-Rust-Concurrency-Basics/
│   ├── Chapter-02-Atomics/
│   ├── Chapter-03-… ~ Chapter-10-…/   # 笔记与占位（英文目录名）
│   ├── crossbeam/                   # crate：study_crossbeam
│   └── rayon/                       # crate：study_rayon
├── async_tokio/                     # 按章笔记 + demo + ch* 示例 .rs（非独立 workspace 成员）
├── rust_network_programming/        # 网络书分阶段 README（非 crate）
└── llvm_insight/                    # llvm_insight_lab + part01–04 + ir_samples
    ├── Cargo.toml
    ├── src/lib.rs
    ├── README.md
    ├── Learn-LLVM-17-学习取舍.md
    ├── part01_basic_compiler/
    ├── part02_src_to_machine/
    ├── part03_llvm_advance/
    ├── part04_custom_backend/
    └── ir_samples/
```

---

## Cargo：编译与 IR 导出（本布局下可直接用）

在**仓库根**执行（路径相对于根）：

```bash
# 原子与并发主库
cargo build --manifest-path atomic/Cargo.toml

# crossbeam / rayon 示例库
cargo build --manifest-path atomic/crossbeam/Cargo.toml
cargo build --manifest-path atomic/rayon/Cargo.toml

# LLVM IR 实验库（包名 llvm_insight_lab）
cargo build --manifest-path llvm_insight/Cargo.toml
cargo rustc --manifest-path llvm_insight/Cargo.toml -p llvm_insight_lab -- --emit=llvm-ir
```

生成的 `.ll` 一般在 **`target/debug/deps/`**（若在各子 crate 目录单独构建，则可能在**该 crate 旁**的 `target/`）；建议将片段复制到 **`llvm_insight/ir_samples/`** 对应子目录并加简短说明（见 `llvm_insight/ir_samples/README.md`）。

### 运行 `study_*` 里的演示逻辑

当前 **`study_atomic` / `study_crossbeam` / `study_rayon` 均为 library**，入口函数例如：

- `study_atomic::demo()`、`run_ex1_default()`、`run_extended()`（见 `atomic/src/mod.rs`）
- `study_crossbeam::demo()`、`study_rayon::demo()`（见各自 `src/lib.rs`）

要**一键跑通**，可在本机任选其一：**新建带 `fn main` 的 example/binary 依赖上述 crate**；或使用你持有的**带根 `cargo run` 的上游版本**。阅读 API 可执行：

```bash
cargo doc --manifest-path atomic/Cargo.toml --no-deps --open
```

### `async_tokio/` 里的 `.rs` 示例

这些文件多为 **`tokio::main`** 或独立 `main`，依赖在**收录它们的 Cargo 工程**里声明。当前树若无根 package，请把需要的文件挂到自建 crate 的 `[[example]]` / `src/bin/`，或恢复上游「根 workspace + `[[example]]` path 指向 `async_tokio/ch*/…`」的布局后再 `cargo run --example ch01_reqwest_join` 等。

---

## `study_atomic` 模块索引（速查）

### `chapter_01`（`Chapter-01-Rust-Concurrency-Basics/`）

| 编号 | 模块（文件） |
|------|----------------|
| 01 | `move_closure.rs` |
| 02 | `thread_advanced.rs` |
| 03 | `thread_example.rs` |
| 04 | `use_cell_refcell.rs` |
| 05 | `use_condvar.rs` |
| 06 | `use_mutex.rs` |
| 07 | `use_mutex_guard_lifetime.rs` |
| 08 | `use_rc_arc.rs` |
| 09 | `use_send_sync.rs` |
| 10 | `useboxleak.rs` |
| 11 | `usecall.rs` |
| 12 | `usejoin.rs` |
| 13 | `userecall.rs` |
| 14 | `usescope.rs` |
| 15 | `usestatic.rs` |

### `chapter_02`（`Chapter-02-Atomics/`）

| 编号 | 模块（文件） |
|------|----------------|
| 01 | `id_allocator.rs` |
| 02 | `lazy_init.rs` |
| 03 | `quick_demo.rs` |
| 04 | `use_atomic.rs` |
| 05 | `use_atomic_operations.rs` |
| 06 | `use_fence.rs` |
| 07 | `use_seqcst.rs` |

---

## 许可证

学习与笔记用途；如需开源许可证可在仓库中自行补充 `LICENSE`。
