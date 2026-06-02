# Network Programming with Rust — 学习区

本目录为《Network Programming with Rust》的**结构化学习路线**，与仓库内 `async_tokio/`、`atomic/`、`crossbeam/`、`rayon/` **并列、互不冲突**：侧重 **Socket 实战、IO 模型、协议与工程化网络**，与「并发底层 + 异步理论」形成互补。

## 阶段总览

| 阶段 | 目录 | 定位 |
|------|------|------|
| Stage 1 | `stage01_network_basic` | 书 Ch1 网络理论打底（[Ch01 笔记](stage01_network_basic/notes/Ch01-客户端服务器网络简介-学习笔记.md)） |
| Stage 2 | `stage02_rust_basis` | 书 Ch2 Rust 生态与语言（[Ch02 笔记](stage02_rust_basis/notes/Ch02-Rust及其生态系统简介-学习笔记.md)） |
| Stage 3 | `stage03_std_tcp_udp` | **`std` 同步 TCP/UDP（全书地基之一）** |
| Stage 4 | `stage04_serde_parse` | 序列化与二进制解析（工程必备） |
| Stage 5 | `stage05_app_protocol` | RPC / FTP / SMTP（可选浅读） |
| Stage 6 | `stage06_http_programming` | HTTP / Hyper / reqwest（按需深入） |
| Stage 7 | `stage07_tokio_async_net` | **Tokio 异步网络（与袋鼠书主线对齐，TOP1）** |
| Stage 8 | `stage08_network_security` | TLS / 安全加固（生产进阶） |
| Stage 9 | `stage09_appendix_deep` | 附录：Future / 协程 / 解析器组合子等底层 |

## 推荐学习顺序（减少弯路）

1. `stage01_network_basic` — 概念打底  
2. `stage03_std_tcp_udp` — **先阻塞 Socket，再谈多路复用与异步**  
3. `stage07_tokio_async_net` — 与 `async_tokio/` 笔记、根 crate 示例对照  
4. `stage04_serde_parse` — 协议与数据形态  
5. `stage08_network_security` — 上线前再深挖  
6. 其余阶段按兴趣与时间补全  

## 与现有仓库的关系

- **并发 / 原子 / 内存序**：见 `atomic/`、`crossbeam/`、`rayon/`  
- **异步 Rust / Tokio 概念与示例**：见 `async_tokio/`、`src/async_tokio/`、根目录 `Cargo.toml` 的 `[[example]]`  
- **本目录**：可逐步放入各阶段的 `demo/`、`notes.md` 或小 crate；不必一次写满，按上表优先级推进即可。

## 本目录约定（可选）

- 每个 `stage*/` 内已有 `README.md`：章节对应、优先级、是否必写 Demo。  
- 若增加可运行代码：建议 `stageNN_*/demo/` 下独立小 crate，或后续再挂入根 workspace，避免与现有成员耦合过重。
