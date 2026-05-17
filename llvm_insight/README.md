# llvm_insight — LLVM IR 透视（不学 LLVM C++ 源码）

本目录是仓库里的**独立 crate**，与 `atomic/`、`async_tokio/`、`rust_network_programming/` **平级、职责分离**：

- **不放** LLVM 工程源码、不要求写 C++。
- **只放**：从 Rust 侧导出 **LLVM IR**（`.ll`）的样例流程、笔记链接、以及本 crate 里用于「生成对照 IR」的小函数。

## 为什么单独一级目录

| 放在这里的好处 |
|----------------|
| 原子 / 无锁 / `async` 最终都经 rustc → LLVM → 机器码；IR 是「编译器实际看到的形状」。 |
| 与 `atomic`（原理与 API）、`async_tokio`（运行时与书）、网络目录（Socket 与协议）**边界清晰**，互不掺目录。 |

## 与 workspace 的关系

根目录 **`[workspace].members`** 已包含 **`llvm_insight`**，便于：

```bash
cargo build -p llvm_insight
cargo check --workspace
```

**未**把 `async_tokio/`、`rust_network_programming/` 加进 members：它们不是独立 `Cargo.toml` 包，强行加入会导致 workspace 解析失败。异步示例仍在**根 package** 的 `[[example]]` 里。

## 导出 LLVM IR（第一个可复现步骤）

在**仓库根目录**执行（任选其一）：

```bash
# 生成 LLVM IR（debug，便于阅读）
cargo rustc -p llvm_insight -- --emit=llvm-ir

# 或 release + IR（看优化后差异）
cargo rustc -p llvm_insight --release -- --emit=llvm-ir
```

生成物通常在：

- `target/debug/deps/llvm_insight-*.ll`
- 或 `target/release/deps/llvm_insight-*.ll`

Windows 下路径相同，用资源管理器或 `dir target\debug\deps\llvm_insight*.ll` 查找。

### 建议对照实验

1. 改 `src/lib.rs` 里 `Ordering`（如 `Relaxed` ↔ `SeqCst`），重新 `emit=llvm-ir`， diff 两份 `.ll`。  
2. 同一源码下对比 **`--release`** 与 **非 release** 的 IR（观察 LLVM 对「可证明无关」的访问的移动/消除）。  
3. 将 `atomic/` 某段逻辑**抄一小段**到本 lib 的新函数里，只为此函数导出 IR，避免根 crate 噪声。

## 目录里还可以逐步补充（任选）

| 内容 | 说明 |
|------|------|
| `notes/` | 自己的 IR 片段批注、优化 pass 名称、与书中章节的对应 |
| `samples/*.ll` | 从 `cargo rustc` 拷出来的代表性 `.ll`（注意体积，可只保留函数片段） |
| `README` 链接 | 指向 rustc book / LLVM LangRef 中 `atomicrmw`、`fence` 等词条 |

## 学习顺序（与总仓库一致）

1. 仍以 **Rust 并发 / 同步网络 / Tokio** 为主线。  
2. **穿插**：每啃透一个点，在本 crate 写最小复现函数 → 导出 IR → 归档。  
3. LLVM 作**透视工具**，不必先于应用层网络学完。
