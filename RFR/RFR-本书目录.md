# 《Rust for Rustaceans》目录

**Rust for Rustaceans: Idiomatic Programming for Experienced Developers**

> 与仓库内 `RFR/Chapter-01-Foundations/` … `Chapter-13-Rust-Ecosystem/` 文件夹一一对应，可按章做笔记或放 demo。

**何时读、如何配合 network/async/locks/LLVM/TLPI**：见 **[学习路径与章节对照.md](./学习路径与章节对照.md)**（含 13 章 ↔ 仓库板块对照表与问题驱动速查）。

---

## 前言与致谢

- Foreword, Preface, Acknowledgments

## 引言

- Introduction

---

## 第 1 章：基础 (Foundations) — `Chapter-01-Foundations/`

- **内存讨论 (Talking About Memory)**：内存术语、变量深入解析、内存区域（栈、堆、寄存器等）。
- **所有权 (Ownership)**。
- **借用与生命周期 (Borrowing and Lifetimes)**：共享引用与可变引用、内部可变性、生命周期深入理解。

本仓库笔记（深度解析）：[`Chapter-01-Foundations/1-基础-Foundations-深度解析.md`](Chapter-01-Foundations/1-基础-Foundations-深度解析.md)

## 第 2 章：类型 (Types) — `Chapter-02-Types/`

- **内存中的类型 (Types in Memory)**：内存对齐 (Alignment)、布局 (Layout)、复杂类型、动态大小类型 (DST) 与宽指针。
- **特质与特质限定 (Traits and Trait Bounds)**：编译与分发（静态分发 vs. 动态分发）、通用特质、一致性与孤儿规则 (Orphan Rule)。
- **特质限定与标记特质 (Marker Traits)**。
- **存在类型 (Existential Types)**。

本仓库笔记（深度解析）：[`Chapter-02-Types/2-类型-Types-深度解析.md`](Chapter-02-Types/2-类型-Types-深度解析.md)

## 第 3 章：接口设计 (Designing Interfaces) — `Chapter-03-Designing-Interfaces/`

- **不意外 (Unsurprising)**：命名习惯、类型的通用特质实现、包装类型。
- **灵活 (Flexible)**：泛型参数、对象安全 (Object Safety)、借用 vs. 拥有。
- **显而易见 (Obvious)**：文档编写、类型系统引导。
- **受限 (Constrained)**：类型修改、特质实现、隐藏契约。

本仓库笔记（深度解析）：[`Chapter-03-Designing-Interfaces/3-接口设计-Designing-Interfaces-深度解析.md`](Chapter-03-Designing-Interfaces/3-接口设计-Designing-Interfaces-深度解析.md)

## 第 4 章：错误处理 (Error Handling) — `Chapter-04-Error-Handling/`

- **表示错误 (Representing Errors)**：枚举式错误、不透明错误、特殊错误场景。
- **传播错误 (Propagating Errors)**。

本仓库笔记（深度解析）：[`Chapter-04-Error-Handling/4-错误处理-Error-Handling-深度解析.md`](Chapter-04-Error-Handling/4-错误处理-Error-Handling-深度解析.md)

## 第 5 章：项目结构 (Project Structure) — `Chapter-05-Project-Structure/`

- **特性 (Features)**：定义、包含与使用特性。
- **工作区 (Workspaces)**。
- **项目配置 (Project Configuration)**：Crate 元数据、构建配置。
- **条件编译 (Conditional Compilation)**。
- **版本控制 (Versioning)**：语义化版本、MSRV（最小支持版本）、变更日志。

本仓库笔记（深度解析）：[`Chapter-05-Project-Structure/5-项目结构-Project-Structure-深度解析.md`](Chapter-05-Project-Structure/5-项目结构-Project-Structure-深度解析.md)

## 第 6 章：测试 (Testing) — `Chapter-06-Testing/`

- **Rust 测试机制 (Rust Testing Mechanisms)**：测试测试架 (Test Harness)、`#[cfg(test)]`、文档测试 (Doctests)。
- **额外测试工具 (Additional Testing Tools)**：Lint 检查、测试生成与增强、性能测试（基准测试）。

本仓库笔记（深度解析）：[`Chapter-06-Testing/6-测试-Testing-深度解析.md`](Chapter-06-Testing/6-测试-Testing-深度解析.md)

## 第 7 章：宏 (Macros) — `Chapter-07-Macros/`

- **声明式宏 (Declarative Macros)**：何时使用、工作原理、编写技巧。
- **过程宏 (Procedural Macros)**：类型（派生、属性、函数式）、成本分析、工作原理。

本仓库笔记（深度解析）：[`Chapter-07-Macros/7-宏-Macros-深度解析.md`](Chapter-07-Macros/7-宏-Macros-深度解析.md)

## 第 8 章：异步编程 (Asynchronous Programming) — `Chapter-08-Asynchronous-Programming/`

- **异步的本质 (What’s the Deal with Asynchrony?)**：同步 vs. 异步接口、多线程、标准化轮询。
- **人体工程学 Future (Ergonomic Futures)**：`async/await`、`Pin` 与 `Unpin`。
- **休眠与唤醒 (Going to Sleep & Waking Up)**：轮询契约、任务与子执行器、`spawn`。

本仓库笔记（深度解析）：[`Chapter-08-Asynchronous-Programming/8-异步编程-Asynchronous-Programming-深度解析.md`](Chapter-08-Asynchronous-Programming/8-异步编程-Asynchronous-Programming-深度解析.md)

## 第 9 章：不安全代码 (Unsafe Code) — `Chapter-09-Unsafe-Code/`

- **unsafe 关键字**。
- **巨大的权力 (Great Power)**：原始指针、调用不安全函数、实现不安全特质。
- **巨大的责任 (Great Responsibility)**：有效性、恐慌 (Panics)、转换 (Casting)、Drop 检查。
- **应对恐惧 (Coping with Fear)**：管理不安全边界、文档编写、检查工作。

本仓库笔记（深度解析）：[`Chapter-09-Unsafe-Code/9-不安全代码-Unsafe-Code-深度解析.md`](Chapter-09-Unsafe-Code/9-不安全代码-Unsafe-Code-深度解析.md)

## 第 10 章：并发与并行 (Concurrency and Parallelism) — `Chapter-10-Concurrency-and-Parallelism/`

- **并发的麻烦 (The Trouble with Concurrency)**：正确性与性能。
- **并发模型 (Concurrency Models)**：共享内存、工作线程池、Actor 模型。
- **异步与并行 (Asynchrony and Parallelism)**。
- **底层并发 (Lower-Level Concurrency)**：原子类型、内存排序 (Memory Ordering)、CAS 操作、Fetch 方法。

本仓库笔记（深度解析）：[`Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md`](Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md)

## 第 11 章：外部函数接口 (Foreign Function Interfaces) — `Chapter-11-Foreign-Function-Interfaces/`

- **跨越边界 (Crossing Boundaries with extern)**：符号、调用约定。
- **跨语言类型 (Types Across Language Boundaries)**：类型匹配、分配、回调、安全性。
- **bindgen 与构建脚本**。

本仓库笔记（深度解析）：[`Chapter-11-Foreign-Function-Interfaces/11-外部函数接口-FFI-深度解析.md`](Chapter-11-Foreign-Function-Interfaces/11-外部函数接口-FFI-深度解析.md)

## 第 12 章：脱离标准库的 Rust (Rust Without the Standard Library) — `Chapter-12-Rust-Without-Standard-Library/`

- **禁用标准库 (Opting Out of std)**。
- **动态内存分配 (Dynamic Memory Allocation)**。
- **Rust 运行时 (The Rust Runtime)**：Panic 处理器、程序初始化、OOM 处理器。
- **底层内存访问与硬件抽象**。
- **交叉编译 (Cross-Compilation)**。

本仓库笔记（深度解析）：[`Chapter-12-Rust-Without-Standard-Library/12-脱离标准库-no_std-深度解析.md`](Chapter-12-Rust-Without-Standard-Library/12-脱离标准库-no_std-深度解析.md)

## 第 13 章：Rust 生态系统 (The Rust Ecosystem) — `Chapter-13-Rust-Ecosystem/`

- **生态资源 (What’s Out There?)**：工具、库、标准库深度解析。
- **实战模式 (Patterns in the Wild)**：索引指针、Drop 卫士、扩展特质、Crate 预导入 (Preludes)。
- **后续学习建议 (What Next?)**。

本仓库笔记（深度解析）：[`Chapter-13-Rust-Ecosystem/13-生态系统-Ecosystem-深度解析.md`](Chapter-13-Rust-Ecosystem/13-生态系统-Ecosystem-深度解析.md)

---

## 索引

- Index
