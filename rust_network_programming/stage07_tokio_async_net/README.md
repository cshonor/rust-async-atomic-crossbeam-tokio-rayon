# Stage 07 — Tokio 异步网络（TOP1）

**对应书籍**：第 7 章 — *Asynchronous Network Programming Using Tokio*

## 章节目录（原书 3 主节 + 4 次级节）

| 书 § | 英文 | 中文 |
|------|------|------|
| 7.1 | Looking into the Future | 探讨 Future |
| 7.1.1 | Working with streams and sinks | 流与接收器 |
| 7.2 | Heading to tokio | 走向 Tokio |
| 7.2.1 | Socket multiplexing in tokio | 套接字多路复用 |
| 7.2.2 | Writing streaming protocols | 流式协议 |
| 7.2.3 | The larger tokio ecosystem | Tokio 生态 |
| 7.3 | Conclusion | 结论 |

## 学习定位

**全书 TOP1** — 与 [`async_tokio/`](../../async_tokio/) 逐节对照。前置 [stage03](../stage03_std_tcp_udp/)。

## 笔记

| 资料 | 说明 |
|------|------|
| [本章学习笔记.md](./本章学习笔记.md) | §7.1～7.3 索引 |
| [Ch07 精读](notes/Ch07-使用Tokio进行异步网络编程-学习笔记.md) | 正文 + `async_tokio` 对照表 |

## 建议 Demo

异步 echo · `select!` · Framed+JSON · graceful shutdown
