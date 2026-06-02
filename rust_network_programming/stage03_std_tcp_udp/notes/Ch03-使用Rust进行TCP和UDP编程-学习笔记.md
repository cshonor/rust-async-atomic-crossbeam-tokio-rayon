# 第 3 章 — 使用 Rust 进行 TCP 和 UDP 编程（TCP and UDP Using Rust）

> 《Network Programming with Rust》· 全书 **5 个小节**（§3.2 含 UDP 多播次级节）  
> 章索引：[本章学习笔记.md](../本章学习笔记.md) · 理论：[Ch01](../../stage01_network_basic/) · 异步：[Ch07](../../stage07_tokio_async_net/)

本章用 **`std::net`** 与相关 crate 编写 TCP/UDP 程序，把 Ch1 理论落到代码。

---

## 章节目录（与原书一致）

| § | 英文 | 中文 |
|---|------|------|
| 3.1 | A Simple TCP server and client | 简单 TCP 服务器与客户端 |
| 3.2 | A Simple UDP server and client | 简单 UDP 服务器与客户端 |
| 3.2.1 | UDP multicasting | UDP 多播 |
| 3.3 | Miscellaneous utilities in std::net | `std::net` 其他实用工具 |
| 3.4 | Some related crates | 相关第三方 crate |
| 3.5 | Summary | 总结 |

---

## 3.1 A Simple TCP server and client（简单 TCP 服务器与客户端）

### 服务器：`TcpListener`

| 步骤 | API |
|------|-----|
| 监听 | `TcpListener::bind("127.0.0.1:8080")?` |
| 接受 | `listener.incoming()` / `accept()` → **`TcpStream`** |
| 并发 | 每连接 **`thread::spawn`** 处理，避免单线程阻塞全体客户端 |

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    stream.write_all(&buf[..n])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || { let _ = handle_client(stream); });
    }
    Ok(())
}
```

- `bind` 后进入 **LISTEN**（Ch1 TCP 状态机）。  
- 线程内 **`move`** 取得 `TcpStream` 所有权。

### 客户端：`TcpStream`

| 步骤 | API |
|------|-----|
| 连接 | `TcpStream::connect("host:port")?` |
| 读写 | `Read` / `Write`、`write_all` |
| 缓冲 | **`BufReader` / `BufWriter`** 减少 syscall，便于按行读 |

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(b"hello")?;
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
```

### 超时

```rust
use std::time::Duration;
stream.set_read_timeout(Some(Duration::from_secs(5)))?;
```

阻塞 `read` 在超时后返回 `Err`（常为 `TimedOut`）。异步版见 [Ch07](../../stage07_tokio_async_net/)。

---

## 3.2 A Simple UDP server and client（简单 UDP 服务器与客户端）

UDP **无连接**、**保留报文边界**（非字节流）。

| 角色 | API |
|------|-----|
| 绑定 | `UdpSocket::bind(addr)?`（服务端/客户端都可能 bind） |
| 收 | **`recv_from(&mut buf)`** → `(len, SocketAddr)` |
| 发 | **`send_to(&buf, target)?`** |

```rust
use std::net::UdpSocket;

let socket = UdpSocket::bind("127.0.0.1:9000")?;
let mut buf = [0u8; 1500];
let (len, src) = socket.recv_from(&mut buf)?;
socket.send_to(&buf[..len], src)?;
```

| | TCP | UDP |
|---|-----|-----|
| 连接 | `accept` / `connect` | 无 |
| 语义 | 流（需分帧） | 数据报 |

### 3.2.1 UDP multicasting（UDP 多播）

- 多播地址：**239.0.0.0/8**（如 `239.0.0.1`）。  
- **`join_multicast_v4(multicast, interface)`** 加入多播组。  
- 场景：服务发现、流媒体、局域网日志汇聚。

---

## 3.3 Miscellaneous utilities in std::net（`std::net` 其他实用工具）

### `IpAddr` / `SocketAddr`

| 类型 | 含义 |
|------|------|
| `IpAddr` | `V4` / `V6` |
| `SocketAddr` | IP + 端口 |

方法：`is_loopback()`、`is_global()`、`is_ipv4()` 等。

```rust
let addr: SocketAddr = "192.168.1.10:8080".parse()?;
```

### DNS

- 书中：**`lookup_host`**（曾需 nightly `feature(lookup_host)`），底层 `getaddrinfo`，多仅 A/AAAA。  
- 日常：**`TcpStream::connect("example.com:80")`**（`ToSocketAddrs` 自动解析）。  
- 全记录类型 → §3.4 **`trust-dns`** / **`hickory-dns`**。

---

## 3.4 Some related crates（相关第三方 crate）

| Crate | 作用 |
|-------|------|
| **`ipnetwork`** | CIDR（`192.168.1.0/24`）、网段判断 |
| **`mio`** | 非阻塞 I/O、`Poll`、Token；**Tokio 基石** |
| **`libpnet`** | 链路层抓包、自解析以太网/IP/TCP 头 |
| **`trust-dns`** | 完整 DNS（同步 + Tokio 异步） |

```text
阻塞 std::net（本章）→ mio 事件循环 → Tokio（Ch7）
```

不推荐裸 **`libc` socket**；优先 **`nix`** 等 Rust 封装。

---

## 3.5 Summary（总结）

| § | 要点 |
|---|------|
| 3.1 | `TcpListener` + `spawn`；`TcpStream` + `BufReader` + 超时 |
| 3.2 | `recv_from` / `send_to`；多播 `join_multicast_v4` |
| 3.3 | `SocketAddr`、`connect` 解析域名 |
| 3.4 | `mio` 通向异步；`ipnetwork` / `libpnet` / DNS crate |

**地基**：不懂阻塞 socket，难理解 Tokio 在抽象什么。

---

## 本章自检

- [ ] §3.1：TCP echo + `set_read_timeout`  
- [ ] §3.2：UDP 与 TCP 差异；多播 API  
- [ ] §3.3：`SocketAddr` 解析  
- [ ] §3.4：`mio` vs 多线程阻塞  

## 建议 Demo

- [ ] TCP 回显 · 长度前缀分帧 · UDP · （可选）多播  

## 下一步

| 方向 | 路径 |
|------|------|
| Ch4 序列化/解析 | [stage04](../../stage04_serde_parse/) |
| Ch7 Tokio | [stage07](../../stage07_tokio_async_net/) |
