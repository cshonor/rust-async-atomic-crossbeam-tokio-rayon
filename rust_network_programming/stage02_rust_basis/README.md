# Stage 02 — Rust 语言与生态前置

**对应书籍**：第 2 章 — *Introduction to Rust and its Ecosystem*（Rust 及其生态系统简介）

## 核心内容

1. Rust 生态：`rustc` / `rustup` / `Cargo`、crates.io  
2. 所有权、借用、生命周期  
3. 泛型与 trait  
4. `Result` / `Option` 与错误处理  
5. 宏（声明宏 / 过程宏）  
6. 闭包、迭代器  
7. 线程、`mpsc`、`Send`/`Sync`、`unsafe` 初识  
8. 单元测试与 doc-test  

## 学习定位

- **查漏补缺**：已有 [`atomic/`](../../atomic/)、[`async_tokio/`](../../async_tokio/) 基础时，重点看 **I/O 错误处理**、**trait 抽象**、**`Send`/`'static`**。  
- 不必整章精读；网络写码中遇编译器报错再回查对应节。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **低～中**（按基础缺口选读） |
| 是否必写 Demo | **否**（需要时 `cargo new` 写 10～20 行验证即可） |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch02 Rust 及其生态系统 — 学习笔记](notes/Ch02-Rust及其生态系统简介-学习笔记.md)** | 全书第 2 章精读（8 大主题） |

## 学习检查

- [ ] `Result` 在 I/O 边界上的传播习惯（`?`）  
- [ ] `Send` / `Sync` / `'static` 与 `thread::spawn`、Tokio 任务的关系  
- [ ] 能说出 `Read`/`Write` trait 与网络读写的关系  
