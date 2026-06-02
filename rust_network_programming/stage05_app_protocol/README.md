# Stage 05 — 应用层协议（RPC / FTP / SMTP）

**对应书籍**：第 5 章 — *Application Layer Protocols*（应用层协议）

## 核心内容

1. **RPC / gRPC**：`.proto`、protobuf、`protoc-rust-grpc`（现代可对照 `tonic`）  
2. **SMTP**：`lettre` 发信、TLS、告警邮件  
3. **FTP**：`rust-ftp` 客户端、控制/数据连接  
4. **TFTP**：UDP、`tftp_server`、PXE/嵌入式场景  

## 学习定位

- **可选、浅读**：目标不是邮件/FTP 客户端时，记协议共性即可。  
- 时间有限时**让路给 stage03、04、07**。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **低（按需）** |
| 是否必写 Demo | **否**（工作相关再写） |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch05 应用层协议 — 学习笔记](notes/Ch05-应用层协议-学习笔记.md)** | 全书第 5 章精读 |

## 学习检查

- [ ] 能区分传输层字节流与应用层语义（RPC/SMTP/FTP）  
- [ ] 能说明 gRPC 与裸 TCP + JSON 的取舍  
- [ ] 知道 FTP 与 TFTP 在传输层与认证上的差异  
