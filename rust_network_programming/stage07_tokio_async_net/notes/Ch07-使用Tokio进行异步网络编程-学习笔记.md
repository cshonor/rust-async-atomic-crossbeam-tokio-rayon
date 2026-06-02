# 第 7 章 — 使用 Tokio 进行异步网络编程（Asynchronous Network Programming Using Tokio）

> 全书 **3 个主节**（§7.1 含 1 个次级节，§7.2 含 3 个次级节）· **TOP1**  
> 章索引：[本章学习笔记.md](../本章学习笔记.md) · 阻塞对照：[Ch03](../../stage03_std_tcp_udp/) · **`async_tokio/`**：[../../../async_tokio/)

异步 I/O 不阻塞线程；支柱是 **Future**、**Stream/Sink** 与 **Tokio** 运行时。

---

## 章节目录（与原书一致）

| § | 英文 | 中文 |
|---|------|------|
| 7.1 | Looking into the Future | 探讨 Future |
| 7.1.1 | Working with streams and sinks | 流与接收器 |
| 7.2 | Heading to tokio | 走向 Tokio |
| 7.2.1 | Socket multiplexing in tokio | 套接字多路复用 |
| 7.2.2 | Writing streaming protocols | 流式协议 |
| 7.2.3 | The larger tokio ecosystem | Tokio 生态 |
| 7.3 | Conclusion | 结论 |

---

## 7.1 Looking into the Future（探讨 Future）

顺序代码在等 I/O 时会**阻塞线程**；异步要求等待时不占住线程。

```text
Ch3 阻塞 read  →  Ch7 read().await → Pending 时调度其他任务
底层：mio (epoll) + Executor (Tokio)
```

### Future 与 `poll`

**Future** = 尚未完成、将来产出结果的计算（期约）。

| 书中 | 现代 `Poll<T>` | 含义 |
|------|----------------|------|
| `Async::Ready(v)` | `Poll::Ready(v)` | 完成 |
| `Async::NotReady` | `Poll::Pending` | 未完成，稍后 wake 再 poll |

**惰性**：须由**执行器**驱动；`async fn` 编译为状态机。

执行方式：当前线程阻塞跑完 / 线程池（CPU 任务）/ **事件循环（网络主流）**。

### 组合子

`and_then`、`map`、`select_ok` → 现代 **`async/await`** + `select!` / `join!`：

```rust
async fn work() -> Result<(), Error> {
    let a = step1().await?;
    let b = step2(a).await?;
    Ok(())
}
```

### 异步协调（配合 Future 的 task 间通信）

标准库 `std::sync::mpsc` / 阻塞 `Mutex` 不适合 `.await` 边界。

| 原语 | 说明 |
|------|------|
| **`oneshot`** | 单次完成通知 |
| **`mpsc`（async）** | 多生产者单消费者 |
| **`BiLock`** | 两 future 间仲裁共享状态 |
| **`tokio::sync::Mutex`** | `.lock().await` 让出而非阻塞 OS 线程 |

对照：[`async_tokio/ch06`](../../../async_tokio/) · [`atomic/`](../../../atomic/) 阻塞版。

---

## 7.1.1 Working with streams and sinks（流与接收器）

### Stream

**异步版 Iterator**：`next().await` → `Option<Item>`，`None` 结束。

用途：TCP 连续帧、WebSocket 消息、定时 tick。

### Sink

**异步版写入端**：接收一系列项并写出（socket、channel 发送端）。

```text
Stream ← 入站    Sink → 出站
```

常配合 **`AsyncRead` / `AsyncWrite`** 或 **`Framed`**（Codec 解码为 `Stream<Item=Frame>`）。

---

## 7.2 Heading to tokio（走向 Tokio）

**Tokio** = **`mio`**（非阻塞 poll）+ **executor**（task + wake）+ **`tokio::net`**。

### 分层架构（书中 `tokio-proto` 思想）

| 层 | 职责 | 现代 |
|----|------|------|
| **Codec** | 字节 ↔ 帧 | `tokio_util::codec::Framed` |
| **Protocol** | 绑事件循环；请求-响应 / 多路复用 / 流式 | 状态机、hyper |
| **Service** | 请求 → 异步响应 Future | `async fn handler` |

```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            if n == 0 { return; }
            let _ = socket.write_all(&buf[..n]).await;
        });
    }
}
```

书中 **Collatz 服务器**多版演进演示下列两节模式。

### 7.2.1 Socket multiplexing in tokio（套接字多路复用）

多个请求同时在飞时，为每个分配 **`RequestId`**：

- 处理完即响应，**不必**与接收顺序一致  
- 短任务可先返回 → 提高并发响应性与吞吐  

类似 HTTP/2 Stream ID、gRPC 并发 call。

### 7.2.2 Writing streaming protocols（流式协议）

- 先收齐 **Header** → 立即路由/鉴权（不必等整个 Body）  
- **Body** 边收边处理 → Stream 持续 `read`  

与 Ch4 **nom/Codec** 长度前缀或定长头配合。

### 7.2.3 The larger tokio ecosystem（更大的 Tokio 生态）

| 书中 | 现代 |
|------|------|
| `tokio-io` | `tokio::io`、`AsyncReadExt` |
| `tokio-curl` | **`reqwest`** |
| `tokio-tls` | **`tokio-rustls`** / `tokio-native-tls` |
| — | **hyper**、**tonic**、**tower** |

TLS：[Ch08](../../stage08_network_security/)。

---

## 7.3 Conclusion（结论）

| 阶段 | 收获 |
|------|------|
| §7.1 | Future、`poll`、组合子、async 通道；Stream/Sink |
| §7.2 | Tokio 分层、多路复用、流式协议、生态 |
| 实践 | 先 Ch3 阻塞 socket，再本章 + **`async_tokio/`** |

### 本书 Ch7 ↔ `async_tokio/`

| 主题 | 目录 |
|------|------|
| async 入门 | `ch01_async_intro/` |
| Future | `ch02`～`ch03` |
| Channel | `ch06_async_channels/` |
| Tokio / 优雅关闭 | `ch07_tokio_*` |
| 测试 | `ch11_async_testing_debugging/` |

根 `Cargo.toml`：`ch07_*`、`ch01_reqwest_join` 等 **[[example]]**。

---

## 本章自检

- [ ] §7.1：`Ready` / `Pending`；async `mpsc` vs `std::sync::mpsc`  
- [ ] §7.1.1：Stream / Sink 类比  
- [ ] §7.2：Codec → Protocol → Service  
- [ ] §7.2.1：RequestId 多路复用  
- [ ] §7.2.2：Header 先处理、Body 流式  

## 建议 Demo

异步 echo · `select!`/timeout · Framed+JSON · graceful shutdown

## 下一步

[Ch08 TLS](../../stage08_network_security/) · [附录 stage09](../../stage09_appendix_deep/)
