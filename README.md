# Rust 并发、异步、网络与系统编程 — 个人学习仓库

本仓库把 **《Rust Atomics and Locks》**、**《Rust for Rustaceans》**、**Tokio / 异步 Rust**、**《Network Programming with Rust》**、**《The Linux Programming Interface》** 与 **LLVM IR（《Learn LLVM 17》取舍）** 放在**并列目录**里，职责清晰、互不塞进同一 crate。

仓库地址：<https://github.com/cshonor/rust-concurrency-async-practice>

---

## 你将找到什么

| 板块 | 路径 | 说明 |
|------|------|------|
| **原子与并发基础** | `atomic/` | Crate **`study_atomic`**：`chapter_01`（线程、`Mutex` / `Condvar`、`Arc`…）、`chapter_02`（`Atomic*`、内存序、栅栏…）。源码按章在 `Chapter-01-*`、`Chapter-02-*` 等目录，`#[path]` 由 `atomic/src/mod.rs` 引入。 |
| **crossbeam / rayon 小示例** | `atomic/crossbeam/`、`atomic/rayon/` | 独立 crate：**`study_crossbeam`**、**`study_rayon`**（与书名/主线的对照副本，可单独 `cargo build`）。 |
| **异步 Rust（Async Rust 书）** | `async_tokio/` | 与 atomic 同规范：`X.Y-slug.md` + `X.Y-slug/` 仅 `.rs`；见 [async_tokio/README.md](./async_tokio/README.md) |
| **网络书路线** | `rust_network_programming/` | **非 crate**：`stage01`～`stage09` 分阶段 `README.md` 模板与优先级说明。 |
| **LLVM IR 与《Learn LLVM 17》** | `llvm_insight/` | **Part 01–04** 章节目录 + **`ir_samples/`** 归档约定；根下 **`llvm_insight_lab`** 小 crate 用于 **`--emit=llvm-ir`**。总览见 `llvm_insight/README.md`，取舍见 `llvm_insight/Learn-LLVM-17-学习取舍.md`。 |
| **Linux 系统编程（TLPI）** | `TLPI/` | 《The Linux Programming Interface》**1～64 章**独立目录，每章 `notes.md` + 进度表；与线程/futex/epoll/socket 对照见 `TLPI/README.md`。 |
| **进阶 Rust（RFR）** | `RFR/` | 《Rust for Rustaceans》**1～13 章**笔记（非 crate）；**建议在 network/async/locks 实战后再系统通读**，阶段内按需查阅见 [`RFR/学习路径与章节对照.md`](RFR/学习路径与章节对照.md)。 |

---

## 推荐学习顺序（与目录设计一致）

> **RFR 定位**：原理向进阶书，不是语法刚过完就读的对象；详见 [`RFR/学习路径与章节对照.md`](RFR/学习路径与章节对照.md)。

1. **基础语法** → **《Effective Rust》**（最佳实践；仓库外）。  
2. **`atomic/`** — 同步与 **内存模型 / 原子**（可与 RFR 第 10 章对照，专书以 `atomic/` 为主）。  
3. **`atomic/crossbeam/`、`atomic/rayon/`** — 通道与数据并行。  
4. **`async_tokio/`** — Tokio / async；遇 **Pin/Unpin** 报错 → 翻 **RFR 第 8 章**。  
5. **`rust_network_programming/`** — Socket / 协议 / 工程化网络。  
6. **`RFR/`** — **问题驱动查阅**（阶段 2）→ **系统通读**（阶段 3，有 TCP/async/锁实战后）。  
7. **`TLPI/`**（按需）— syscall / 线程 / epoll / socket；与 **RFR 第 11 章 FFI** 对照。  
8. **`llvm_insight/`** — IR 与优化；与 **RFR 第 2、9 章** 对照。

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
├── TLPI/                            # TLPI 1–64 章笔记（非 crate）
│   ├── README.md
│   └── chapter-NN-*/notes.md
├── RFR/                             # Rust for Rustaceans 1–13 章笔记（非 crate）
│   ├── RFR-本书目录.md
│   └── Chapter-01-Foundations/ … Chapter-13-Rust-Ecosystem/
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
