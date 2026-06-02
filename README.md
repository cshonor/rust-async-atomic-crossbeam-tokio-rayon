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

## 推荐学习顺序

### 全仓库主线（各板块先后）

> **RFR 定位**：原理向进阶书，不是语法刚过完就读的对象；详见 [`RFR/学习路径与章节对照.md`](RFR/学习路径与章节对照.md)。

1. **基础语法** → **《Effective Rust》**（最佳实践；仓库外）。  
2. **`async_tokio/` + `atomic/`** — 见下节 **贯通计划**（并发两条线，建议螺旋式对照，而非各读各的）。  
3. **`atomic/crossbeam/`、`atomic/rayon/`** — 通道与数据并行（可在 async Ch08 / atomic Ch05 之后）。  
4. **`rust_network_programming/`** — Socket / 协议 / 工程化网络（与 async Ch04、Ch01 §1.6 衔接）。  
5. **`RFR/`** — **问题驱动查阅** → **系统通读**（有 TCP / async / 锁实战后）。  
6. **`TLPI/`**（按需）— syscall / 线程 / futex / epoll；与 async Ch04、atomic Ch08 对照。  
7. **`llvm_insight/`** — IR 与优化；与 **RFR 第 2、9 章** 对照。

### `async_tokio/` 与 `atomic/` 如何相辅相成

| 维度 | [`async_tokio/`](async_tokio/README.md)（*Async Rust*） | [`atomic/`](atomic/README-学习区.md)（*Rust Atomics and Locks*） |
|------|--------------------------------------------------------|------------------------------------------------------------------|
| 核心问题 | I/O 等待时如何**不阻塞线程** | 多线程/多任务如何**安全共享数据** |
| 主要工具 | `Future`、`async/await`、Tokio、Waker | `Mutex`、`Atomic*`、内存序、通道、futex |
| 在本书仓库里 | `ch01_*` … `ch11_*`，每节 `X.Y-slug.md` + `X.Y-slug/*-demo.rs` | `Chapter-01-*` … `Chapter-10-*`，`cargo build -p study_atomic` |

一句话：**async 解决「等 I/O 时干什么」；atomic/锁 解决「多个执行流碰同一块数据怎么办」。** Tokio 程序里两者天天见面（`tokio::sync::Mutex`、`Arc`、响应式里的 `Atomic*`、Actor 与通道等）。

#### 先学谁？

| 顺序 | 适合 | 说明 |
|------|------|------|
| **先 async → 再 atomic** | 尽快写 Tokio / Web | 正反馈快；读到 Ch06–08 时建议按下面表回跳 atomic |
| **先 atomic → 再 async** | 想搞懂再写、偏面试/底层 | 本仓库早期 README 默认此序 |
| **螺旋贯通**（推荐） | 你现在这种双文件夹仓库 | 大方向先 async，到「锁/原子/通道」节点插入 atomic，再回 async |

下面 **阶段 0～7** 按螺旋贯通编写（约 8～12 周，可伸缩）。每步：读 async 该章索引 → 各节 `.md` → 进 `X.Y-slug/` 跑 demo；对照表跳到 atomic 对应章。

---

#### 阶段 0：共同地基（约 3～5 天）

| async | atomic | 目标 |
|-------|--------|------|
| [Ch01 §1.1](async_tokio/ch01_async_intro/1.1-what-is-async.md) + [join demo](async_tokio/ch01_async_intro/1.1-what-is-async/) | 可选：[Chapter-01](atomic/Chapter-01-Rust-Concurrency-Basics/本章学习笔记.md) 线程概念 | 分清阻塞 / 非阻塞 / 并发 / 并行 |

---

#### 阶段 1：异步心智 + Future（约 2 周）

| 周 | async_tokio | 同步补 atomic | 贯通问题 |
|----|-------------|---------------|----------|
| 1 | **Ch01** 全章（§1.6 HTTP `join!`） | Ch01：`Mutex` / `Arc` 扫读 | 为何「一连接一线程」难扩展 |
| 2 | **Ch02**（Task、Future、Pin、Waker、§2.6–2.8） | — | `.await` ≈ `poll` + 可能 `Pending` |

**Demo 优先**：`ch02/2.2-futures/`、`2.5-remote-waker/`（若有）。

---

#### 阶段 2：执行器与队列（约 1.5 周）— 第一次深入 atomic

| async_tokio | atomic | 贯通 |
|-------------|--------|------|
| **Ch03** 任务队列、窃取、优先级 | [Ch04 自旋锁](atomic/Chapter-04-Spin-Locks/本章学习笔记.md)、[Ch05 通道](atomic/Chapter-05-Channels/本章学习笔记.md) | 队列里的任务 vs 线程；`mpsc` 与 Ch05 |
| Ch03.7 配置运行时 | [Ch03 内存序](atomic/Chapter-03-Memory-Ordering/本章学习笔记.md)（Relaxed / Acquire / Release 概念） | 为 Ch06 加热器打基础 |

---

#### 阶段 3：网络与运行时边界（约 1.5 周）

| async_tokio | atomic | 贯通 |
|-------------|--------|------|
| **Ch04**（Executor、mio、socket） | [Ch07 处理器/缓存](atomic/Chapter-07-Processors/本章学习笔记.md)（浏览） | 非阻塞 `WouldBlock` 为何不能阻塞执行线程 |
| demo：`4.8` / `4.9` | — | 不必全书实现 hyper |

---

#### 阶段 4：协程 + 响应式（约 2 周）— 第二次深入 atomic

| async_tokio | atomic | 贯通 |
|-------------|--------|------|
| **Ch05** 协程/生成器 | — | `async` 是编译器状态机 |
| **Ch06** 观察者、`Atomic*`、`compare_exchange`、事件总线 | **Ch02–03** 原子 + 内存序 | **最重要交叉** |
| Ch06.5–6.7 事件总线 | Ch05 通道、Ch01 `Mutex` | 总线 vs `Mutex<HashMap>` |

**必读**：async [§6.2](async_tokio/ch06_reactive_async_streams/6.2-building-display-observer.md) · atomic [Atomics与内存序-贯通笔记.md](atomic/Atomics与内存序-贯通笔记.md)（若有）。

---

#### 阶段 5：定制 Tokio + Actor（约 2 周）

| async_tokio | atomic | 贯通 |
|-------------|--------|------|
| **Ch07** Builder、本地池、`UnsafeCell` | Ch01 `Mutex`、Ch04 自旋、Ch07 缓存行 | pinned 线程上 `UnsafeCell` 且不在持锁期间 `await` |
| **Ch08** Actor、`mpsc` / `oneshot`、路由、监督 | **Ch05** + [互斥锁与锁体系-贯通笔记.md](atomic/互斥锁与锁体系-贯通笔记.md) | Actor vs `Mutex`（async §8.2） |

**Demo 优先**：`8.1-building-basic-actor/`、`8.2-actors-versus-mutexes/` + atomic `use_mutex*.rs`。

---

#### 阶段 6：设计模式 + 手写运行时 + 测试（约 2 周）

| async_tokio | atomic | 贯通 |
|-------------|--------|------|
| **Ch09** 重试、熔断 | Ch02 `AtomicBool` / `AtomicUsize` | 熔断计数 |
| **Ch10** 纯 `std` 运行时 | [Ch08 OS 原语](atomic/Chapter-08-OS-Primitives/本章学习笔记.md)（futex 概念） | Waker 在 OS 层大致对应什么 |
| **Ch11** 死锁 / 竞态 / 通道测试 | Ch01–03、Ch10 | `timeout`、多线程测试 |

**Demo 优先**：Ch10 `10.2-building-std-async-runtime/` · Ch11 `11.3`～`11.5`。

---

#### 阶段 7：收束与进阶（持续）

| 方向 | 资源 |
|------|------|
| 锁 / Mutex / RwLock | [互斥锁与锁体系-贯通笔记.md](atomic/互斥锁与锁体系-贯通笔记.md)、[RwLock与读写锁体系-贯通笔记.md](atomic/RwLock与读写锁体系-贯通笔记.md) |
| 条件变量 | [Condvar与条件变量-贯通笔记.md](atomic/Condvar与条件变量-贯通笔记.md) |
| 无锁 | [无锁编程-贯通笔记.md](atomic/无锁编程-贯通笔记.md) |
| 网络实战 | [`rust_network_programming/`](rust_network_programming/README.md) |
| Pin / 进阶类型 | [`RFR/`](RFR/RFR-本书目录.md) 第 8 章（遇错再查） |

---

#### 章节对照跳转表（学到 async 某章时查 atomic）

| async 章 | 建议同步或回看 atomic |
|----------|------------------------|
| Ch01 进程/线程 | [Chapter-01](atomic/Chapter-01-Rust-Concurrency-Basics/本章学习笔记.md) |
| Ch02 §2.6–2.7 共享数据 | Chapter-01 `Mutex`/`Arc`；Chapter-02 原子初识 |
| Ch03 任务队列 | Chapter-04、05 |
| Ch06 响应式 | **Chapter-02、03**（重点） |
| Ch07 本地池 / `UnsafeCell` | Chapter-01、04、07 |
| Ch08 Actor | **Chapter-05** + 互斥锁贯通笔记 |
| Ch09 熔断 | Chapter-02 |
| Ch11 测试 | Chapter-01–03、10 |

**async 全书索引**：[async_tokio/本书详细目录.md](async_tokio/本书详细目录.md) · [章节与小节对照表.md](async_tokio/章节与小节对照表.md)  
**atomic 全书索引**：[atomic/全书目录-与实体书一致.md](atomic/全书目录-与实体书一致.md) · [atomic/README-学习区.md](atomic/README-学习区.md)

#### 每周动手习惯

1. 打开该章 **`本章学习笔记.md`** → 逐节读 **`X.Y-slug.md`** → 进入 **`X.Y-slug/`** 跑 demo。  
2. 用上表跳到 atomic 对应章：`cargo build --manifest-path atomic/Cargo.toml`。  
3. 可选：在节末写一句「本节 async 用到了 atomic 的什么」，方便复习。

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
