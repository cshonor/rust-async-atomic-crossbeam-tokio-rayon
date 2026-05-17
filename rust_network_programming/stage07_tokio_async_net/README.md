# Stage 07 — Tokio 异步高并发网络（核心主线）

**对应书籍**：第 7 章

## 核心内容

- 异步 `TcpListener` / `TcpStream`、`tokio::net`  
- 任务、`spawn`、背压、半关闭  
- `Stream` / `Sink`、分帧、超时与取消（与袋鼠书一致）  
- 运行时与多路复用在工程上的对应关系

## 学习定位

- **全书对你价值最高的一章**：与 `async_tokio/` 笔记、根目录 `[[example]]` **逐节对照**。  
- 建议：**逐节手写 Demo + 笔记归档**。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **TOP1** |
| 是否必写 Demo | **是**（与书本进度同步） |

## 建议 Demo 清单

- [ ] 异步 echo server（多连接）  
- [ ] 带 `tokio::select!` 或 timeout 的连接生命周期管理  
- [ ] 与 Stage 04 结合的编解码管线（可先同步再改成 async read 循环）  
- [ ] （可选）graceful shutdown（对照 `async_tokio/ch07_tokio_graceful_shutdown/`）  

## 与仓库其他部分

- 笔记与已有示例：`async_tokio/` 各章目录、`src/async_tokio/mod.rs`、根 `Cargo.toml` 中 `ch07_*` 等示例名。  

## 笔记区

（在此补充链接、摘录或 `notes.md`）
