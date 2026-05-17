# llvm_insight — LLVM IR 透视（不学 LLVM C++ 源码）

与 `atomic/`、`async_tokio/`、`rust_network_programming/` **平级**：**笔记 + 可运行小 crate + IR 样本归档** 三件事分目录完成；不要求啃 LLVM 工程源码、不要求写 C++ 编译器。

## 总览：与《Learn LLVM 17》13 章对齐的目录树

```text
llvm_insight/
├── Cargo.toml                 # 包名 llvm_insight_lab：统一 IR 导出入口
├── src/lib.rs                 # 最小复现（原子/栅栏等），配合 ch04/ch07 精读改这里
├── Learn-LLVM-17-学习取舍.md  # 全书精读/浏览/跳过清单 + 与仓库学习顺序
├── README.md                  # 本文件：命令与约定
├── part01_basic_compiler/
│   ├── README.md
│   ├── chapter01_env_setup/           # 第1章 【浏览】
│   └── chapter02_compiler_arch/       # 第2章 【精读】notes/ + code/
├── part02_src_to_machine/             # 核心主攻
│   ├── README.md
│   ├── chapter03_ast_build/           # 第3章 【浏览】
│   ├── chapter04_ir_basic/            # 第4章 【精读】IR / SSA
│   ├── chapter05_ir_advanced_type/  # 第5章 【精读】复合类型 IR
│   ├── chapter06_ir_extend/         # 第6章 【浏览】
│   └── chapter07_ir_optimize/         # 第7章 【精读】优化流水线
├── part03_llvm_advance/
│   ├── README.md
│   ├── chapter08_tablegen/            # 第8章 【跳过】仅占位
│   ├── chapter09_jit_compile/       # 第9章 【浏览】
│   └── chapter10_debug_tool/         # 第10章 【精读】工具链
├── part04_custom_backend/             # 第11–13章 【整部分跳过】
│   ├── README.md
│   ├── chapter11_target_desc/
│   ├── chapter12_instr_select/
│   └── chapter13_backend_extend/
└── ir_samples/                        # 业务向 .ll 归档（见 ir_samples/README.md）
    ├── atomic_ir/
    ├── async_tokio_ir/
    ├── network_ir/
    └── optimize_compare/
```

### 图例

| 标记 | 含义 | 目录里一般有什么 |
|------|------|------------------|
| **精读** | ch02、ch04、ch05、ch07、ch10 | `notes/` + `code/`（可选）；IR 进 `ir_samples/` |
| **浏览** | ch01、ch03、ch06、ch09 | 以 `notes/` 为主，概念即可 |
| **跳过** | ch08、ch11–ch13、整 part04 | 仅 `README.md` 占位，不投入精力 |

## 统一 IR 实验 crate（根目录 `Cargo.toml`）

包名：**`llvm_insight_lab`**（避免与目录名 `llvm_insight` 在部分工具里混淆）。

在**仓库根**执行：

```bash
cargo build --manifest-path llvm_insight/Cargo.toml
cargo rustc --manifest-path llvm_insight/Cargo.toml -- --emit=llvm-ir
cargo rustc --manifest-path llvm_insight/Cargo.toml --release -- --emit=llvm-ir
```

若仓库根 **`Cargo.toml`** 的 **`[workspace].members`** 已包含 **`llvm_insight`**，也可：

```bash
cargo build -p llvm_insight_lab
cargo rustc -p llvm_insight_lab -- --emit=llvm-ir
```

生成物通常在 `target/debug/deps/llvm_insight_lab-*.ll`（或 `release`）。将需要的片段**复制**到 `ir_samples/` 下对应子目录（见 `ir_samples/README.md`）。

### 建议对照实验

1. 改 `src/lib.rs` 中 `Ordering`，重新 `emit=llvm-ir`，diff 两份 `.ll` → 归档到 `ir_samples/optimize_compare/`。  
2. 从 `atomic/Chapter-02-Atomics/` 抄 20 行以内逻辑进 `src/lib.rs`，只导出该路径的 IR → `ir_samples/atomic_ir/`。  
3. 对异步示例：在带 Tokio 依赖的 crate 中导出后，截取相关 `define` → `ir_samples/async_tokio_ir/`。

## 《Learn LLVM 17》取舍与仓库路线

详见 **[Learn-LLVM-17-学习取舍.md](./Learn-LLVM-17-学习取舍.md)**（与上表 **精读/浏览/跳过** 一致）。

## 为什么单独一级目录

| 好处 |
|------|
| 原子 / 无锁 / `async` 经 rustc → LLVM → 机器码；IR 是「编译器里长什么样」。 |
| 与 `atomic`（原理）、`async_tokio`（书与 demo）、网络阶段 README **职责分离**。 |

Tokio 示例若在根 package 的 `[[example]]` 里，仍在仓库根 `cargo run --example …`；本目录只负责 **IR 与 LLVM 书** 的归档与笔记。
