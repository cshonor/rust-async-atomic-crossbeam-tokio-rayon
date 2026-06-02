# Stage 03 — Rust 原生同步网络（std）

**对应书籍**：第 3 章 — *TCP and UDP Using Rust*（使用 Rust 进行 TCP 和 UDP 编程）

## 核心内容

1. 多线程 TCP 服务器（`TcpListener` + `thread::spawn`）与客户端（`TcpStream`、`BufReader`、超时）  
2. UDP `UdpSocket`、`recv_from` / `send_to`、多播 `join_multicast_v4`  
3. `IpAddr`、`SocketAddr`、DNS（`lookup_host` / `ToSocketAddrs`）  
4. 生态：`ipnetwork`、`mio`、`libpnet`、`trust-dns`；避免裸 `libc`  

## 学习定位

> **不懂同步 socket，很难真正理解 Tokio 里在抽象什么。**  
> 顺序：**阻塞 `std::net`（本章）→ mio/多路复用概念 → Tokio（stage07）**。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **最高（地基级）** |
| 是否必写 Demo | **是**：建议 `demo/` 下每类各一份最小可运行示例 |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch03 TCP/UDP 编程 — 学习笔记](notes/Ch03-使用Rust进行TCP和UDP编程-学习笔记.md)** | 全书第 3 章精读 |

## 建议 Demo 清单（`demo/` 逐步实现）

- [ ] 回显 TCP 服务端 + 客户端  
- [ ] 换行或长度前缀分帧的一轮请求—响应  
- [ ] UDP 一发一收  
- [ ] （可选）UDP 多播  

## 与仓库其他部分

- 语言/线程基础：[stage02](../stage02_rust_basis/notes/Ch02-Rust及其生态系统简介-学习笔记.md)  
- 异步对照：`async_tokio/ch10_dependency_free_async_server/`  
- Tokio：[stage07](../stage07_tokio_async_net/README.md)  

## 学习检查

- [ ] 能独立写出阻塞 TCP echo 服务端（每连接一线程）  
- [ ] 会用 `set_read_timeout` 处理慢响应  
- [ ] 能说明 `mio` 与多线程阻塞模型的取舍  
