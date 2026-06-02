# Stage 07 — Tokio 异步高并发网络（核心主线）

**对应书籍**：第 7 章 — *Asynchronous Network Programming Using Tokio*（使用 Tokio 进行异步网络编程）

## 核心内容

1. **Future**：`poll`、`Ready`/`Pending`、执行器、组合子 / `async-await`  
2. **Stream / Sink**  
3. 异步 **oneshot / mpsc / BiLock**（对照 `tokio::sync`）  
4. **Codec → Protocol → Service**（`tokio-proto` 思想 → `Framed`）  
5. 多路复用、流式 Header/Body（Collatz 服务器示例）  
6. 生态：`tokio-io`、TLS、HTTP 客户端的现代对照  

## 学习定位

- **全书 TOP1**：与 [`async_tokio/`](../../async_tokio/) 笔记、根 `Cargo.toml` **`[[example]]`** **逐节对照**。  
- 前置：先完成 [stage03](../stage03_std_tcp_udp/) 阻塞 Socket。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **TOP1** |
| 是否必写 Demo | **是** |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch07 Tokio 异步网络 — 学习笔记](notes/Ch07-使用Tokio进行异步网络编程-学习笔记.md)** | 全书第 7 章精读 + `async_tokio/` 对照表 |

## 建议 Demo 清单

- [ ] 异步 echo server  
- [ ] `select!` / `timeout`  
- [ ] Framed + JSON（接 stage04）  
- [ ] graceful shutdown  

## 与仓库其他部分

- **主线**：`async_tokio/`、`src/async_tokio/mod.rs`  
- 阻塞对照：stage03 · HTTP：stage06  

## 学习检查

- [ ] 能独立写一个 `#[tokio::main]` echo 服务  
- [ ] 能说明为何 async 通道不能用阻塞 `std::sync::mpsc`  
