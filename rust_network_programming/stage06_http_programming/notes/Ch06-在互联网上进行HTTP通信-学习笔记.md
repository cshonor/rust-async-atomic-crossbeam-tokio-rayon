# 第 6 章 — 在互联网上进行 HTTP 通信（Talking HTTP in the Internet）

> 全书 **4 个小节**（§6.1～6.4）  
> 章索引：[本章学习笔记.md](../本章学习笔记.md) · Tokio：[Ch07](../../stage07_tokio_async_net/) · Serde：[Ch04](../../stage04_serde_parse/)

三个核心 crate：**Hyper**（服务底座）· **Rocket**（全栈 Web）· **reqwest**（客户端）。

---

## 章节目录（与原书一致）

| § | 英文 | 中文 |
|---|------|------|
| 6.1 | Introducing Hyper | Hyper 简介 |
| 6.2 | Introducing Rocket | Rocket 简介 |
| 6.3 | Introducing reqwest | reqwest 简介 |
| 6.4 | Summary | 总结 |

---

## 6.1 Introducing Hyper（Hyper 简介）

**Hyper**：成熟底层 **HTTP** 库（解析/生成 HTTP + 驱动连接），常与 **Tower** 组合；基于 **Tokio + async/await**。

| 主题 | 要点 |
|------|------|
| 单线程服务 | 一个 reactor 处理连接，模型简单，单核吞吐受限 |
| 多线程扩展 | **`tokio`** 多线程运行时 + **`SO_REUSEPORT`**（书中 `net2`）多监听同一端口，连接分摊到多 reactor，吞吐可近翻倍 |

**现代**：`tokio` 1.x + `hyper` 1.x（书中 `tokio-core` 即 Reactor 前身）。

---

## 6.2 Introducing Rocket（Rocket 简介）

**Rocket**：高层 Web 框架（体验接近 Flask）。

| 特性 | 说明 |
|------|------|
| 路由 | `#[get]`, `#[post]` 等属性宏 |
| 编译器 | 书中时代常需 **nightly**（以当前文档为准） |
| 博客 CRUD 示例 | **`diesel`** ORM + **`r2d2`** 连接池 + SQLite |
| 入参 | **Data Guards** + **Serde** 自动校验 JSON |
| 模板 | **`rocket_contrib`** + **Tera**（类 Jinja2） |
| 状态 | **Managed State**，如 **`AtomicUsize`** 线程安全访问计数 |

生产还可选 **axum**、**actix-web**；Rocket 重在理解路由 + 提取器模式。

---

## 6.3 Introducing reqwest（reqwest 简介）

**reqwest**：HTTP 客户端，API 接近 Python **`requests`**。

| 能力 | 说明 |
|------|------|
| 同步 | `reqwest::blocking::Client` |
| 异步 | `#[tokio::main]` + `.await`（书中 `tokio_core` + Future 链 → 现为 async/await） |
| JSON | 内置 **Serde**：`.json(&body)` / `.json()` 反序列化 |

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://httpbin.org/get").await?.text().await?;
    Ok(())
}
```

仓库：`async_tokio/ch01_reqwest_join` 等。

---

## 6.4 Summary（总结）

| Crate | 角色 |
|-------|------|
| **Hyper** | 底层 HTTP 服务 / 网关 |
| **Rocket** | 快速 REST + 模板站 |
| **reqwest** | 调第三方 API |

依赖链：**Ch3 Socket** → **Ch7 Tokio** → **Ch6 HTTP** → **Ch8 TLS（HTTPS）**。

---

## 本章自检

- [ ] §6.1：Hyper vs 多线程 `REUSEPORT`  
- [ ] §6.2：Rocket + diesel + Serde 入参  
- [ ] §6.3：reqwest 同步/异步 + JSON  

## 下一步

[Ch07 Tokio](../../stage07_tokio_async_net/) · [Ch08 安全](../../stage08_network_security/)
