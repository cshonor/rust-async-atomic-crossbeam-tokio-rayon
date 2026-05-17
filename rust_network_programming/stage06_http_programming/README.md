# Stage 06 — HTTP 编程（Hyper / Rocket / reqwest）

**对应书籍**：第 6 章

## 核心内容

- HTTP/1.1 基础、服务端与客户端 API  
- 生态：`hyper`、`reqwest`、（若书中）`rocket` 等

## 学习定位

- **选学 / 次要**：若近期不做网关、REST/gRPC 网关，可后移。  
- 需要抓 HTTP 时，与 `async_tokio` 中 `reqwest` 示例、`Cargo.toml` 依赖对照即可。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **中（按项目触发）** |
| 是否必写 Demo | **按需**（例如一个 `reqwest` GET + 超时） |

## 建议 Demo 清单（按需）

- [ ] 最小 HTTP 客户端（`reqwest` 或书中推荐栈）  
- [ ] （可选）最小 HTTP 服务端（与 Stage 07 的 async 模型一起看）  

## 与仓库其他部分

- 根 crate 示例：`ch01_reqwest_join`（路径在 `async_tokio/ch01_async_intro/`）。  

## 笔记区

（在此补充链接、摘录或 `notes.md`）
