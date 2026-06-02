# Stage 08 — 网络安全与生产加固

**对应书籍**：第 8 章 — *Security*（安全）

## 核心内容

1. HTTPS/TLS、CA、X.509、握手与身份验证  
2. **Let's Encrypt**、ACME、**rustls** / webpki 客户端  
3. **rust-openssl**：证书字段、自签名生成  
4. **Tokio + TLS**（书中 `tokio-tls`、PKCS12；现代 `tokio-rustls`）  
5. **`ring`**、X25519、Diffie-Hellman 密钥交换  

## 学习定位

- **生产进阶**：本地实验可晚于 stage03、07。  
- 对外服务前再拉高优先级。

## 优先级与代码

| 项目 | 建议 |
|------|------|
| 优先级 | **中（上线前 → 高）** |
| 是否必写 Demo | **按需**（`reqwest` + rustls GET 或 tokio-rustls echo） |

## 笔记

| 资料 | 说明 |
|------|------|
| **[Ch08 安全 — 学习笔记](notes/Ch08-安全-学习笔记.md)** | 全书第 8 章精读 |

## 学习检查

- [ ] 能说明加密 vs 证书校验各解决什么  
- [ ] 知道开发自签名与生产 CA 签发的区别  
- [ ] 知道 PEM 与 PKCS#12 的常见用途差异  
