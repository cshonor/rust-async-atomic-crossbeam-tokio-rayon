# Stage 06 — HTTP 编程（Hyper / Rocket / reqwest）

**对应书籍**：第 6 章 — *Talking HTTP in the Internet*（在互联网上进行 HTTP 通信）

## 核心内容

1. **Hyper**：异步 HTTP 服务、单线程 vs 多线程 / `SO_REUSEPORT`  
2. **Rocket**：路由宏、博客 CRUD、`diesel` + `r2d2`、Tera 模板、Managed State  
3. **reqwest**：HTTP 客户端、Serde JSON、同步与异步  

## 学习定位

- **按项目触发**：不做 Web/REST 时可后移；需要调 API 时优先 **reqwest**。  
- 与 [stage07](../stage07_tokio_async_net/)、`async_tokio/` 中 reqwest 示例对照。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **中** |
| 是否必写 Demo | **按需**（如一个 `reqwest` GET + 超时） |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch06 HTTP 通信 — 学习笔记](notes/Ch06-在互联网上进行HTTP通信-学习笔记.md)** | 全书第 6 章精读 |

## 建议 Demo（按需）

- [ ] `reqwest` 同步或 `#[tokio::main]` 异步 GET  
- [ ] （可选）最小 Hyper 或 axum 回显路由  

## 与仓库其他部分

- `async_tokio/ch01_async_intro/` — `ch01_reqwest_join` 等  

## 学习检查

- [ ] 能区分 Hyper / Rocket / reqwest 的职责  
- [ ] 知道异步 HTTP 客户端依赖 Tokio 运行时  
