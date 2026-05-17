# llvm_insight — LLVM IR 透视（不学 LLVM C++ 源码）

本目录是仓库里的**独立 crate**，与 `atomic/`、`async_tokio/`、`rust_network_programming/` **平级、职责分离**：

- **不放** LLVM 工程源码、不要求写 C++。
- **只放**：从 Rust 侧导出 **LLVM IR**（`.ll`）的样例流程、笔记链接、以及本 crate 里用于「生成对照 IR」的小函数。

## 《Learn LLVM 17》在仓库里的读法（章节目录 + 取舍）

不必啃全书、不必写 C++ 编译器。仓库内有一份**与本书 13 章对齐的精读/跳过清单**，并串到 `atomic` → crossbeam/rayon → `async_tokio` → 网络 → 再 LLVM 的顺序：

- **[Learn-LLVM-17-学习取舍.md](./Learn-LLVM-17-学习取舍.md)**

## 为什么单独一级目录

| 放在这里的好处 |
|----------------|
| 原子 / 无锁 / `async` 最终都经 rustc → LLVM → 机器码；IR 是「编译器实际看到的形状」。 |
| 与 `atomic`（原理与 API）、`async_tokio`（运行时与书）、网络目录（Socket 与协议）**边界清晰**，互不掺目录。 |

## 与 Cargo workspace 的关系

- **若仓库根存在** `Cargo.toml` 且 **`[workspace].members`** 包含 **`llvm_insight`**：在仓库根执行 `cargo build -p llvm_insight`、`cargo rustc -p llvm_insight -- --emit=llvm-ir` 即可。  
- **若本目录为独立 crate**（根无 workspace 或未列入 members）：在**仓库根**用 manifest 调用：

```bash
cargo build --manifest-path llvm_insight/Cargo.toml
cargo rustc --manifest-path llvm_insight/Cargo.toml -- --emit=llvm-ir
```

`async_tokio/`、`rust_network_programming/` 通常不是独立包；Tokio 示例若在根 package 的 `[[example]]` 里，仍在根目录 `cargo run --example …`。

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

1. 仍以 **Rust 并发 / 同步网络 / Tokio** 为主线（详见 [Learn-LLVM-17-学习取舍.md](./Learn-LLVM-17-学习取舍.md) 末尾「与当前仓库学习路线」）。  
2. **穿插**：每啃透一个点，在本 crate 写最小复现函数 → 导出 IR → 归档。  
3. LLVM 作**透视工具**；《Learn LLVM 17》按取舍文档**靠后精读**第 2、4、5、7、10 章即可。
