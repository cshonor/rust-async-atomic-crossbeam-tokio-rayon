# Stage 04 — 序列化与协议解析

**对应书籍**：第 4 章 — *Data Serialization, Deserialization, and Parsing*（数据序列化、反序列化与解析）

## 核心内容

1. **Serde**：derive、`serde_json`、TCP 上传输结构体（含粘包/分帧）  
2. 自定义 `Serialize` / `Deserialize`、Visitor、`serde_test`  
3. 文本解析：**nom** / **pom**、HTTP 请求行示例  
4. 二进制解析：**nom** `bits!`、IPv6 首部  

## 学习定位

- **工程必备**：RPC、自有协议、行情/日志流都依赖这一层。  
- 建议与 [stage03](../stage03_std_tcp_udp/) 的小 TCP Demo **合并练习**（先能传字节，再传 JSON）。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **高** |
| 是否必写 Demo | **是**：至少 JSON 消息体 + 一种分帧方式 |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch04 序列化与解析 — 学习笔记](notes/Ch04-数据序列化反序列化与解析-学习笔记.md)** | 全书第 4 章精读 |

## 建议 Demo 清单（`demo/` 逐步实现）

- [ ] `Point3D` JSON over TCP（长度前缀）  
- [ ] 固定头 + 负载，或 `bincode` 二进制  
- [ ] （可选）`nom` 解析 HTTP 请求行  

## 学习检查

- [ ] 能解释 Serde 与「TCP 分帧」各自负责哪一层  
- [ ] 知道 `nom` 与 Serde 分别适合什么协议形态  
