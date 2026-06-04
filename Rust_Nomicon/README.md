# Rustonomicon 学习仓库

## 目录说明

1. **Nightly/**：对应 Nightly 黑魔法版，笔记 + 带不稳定 `feature` 源码，编译：`rustup run nightly cargo …`（在 `Nightly/` 或各章 `src/` 下）
2. **Stable/**：对应稳定死灵书，笔记 + 全稳定源码，编译：`rustup run stable cargo …`（在 `Stable/` 或各章 `src/` 下）

## 基准 Edition

**2018**（与两版 Nomicon 主体一致；后续可按需升级单章 crate）

## 规则

同一知识点**两份笔记对照学习**：Nightly 看前沿原理与 unstable 写法，Stable 改成可上线代码。

## 全书大纲（11 章 · 两仓 `src/` 子目录同名）

| 目录 | 主题 |
|------|------|
| `01_Safe_Unsafe` | 安全 / 不安全边界 |
| `02_Data_Layout` | 数据布局 |
| `03_Lifetime_Variance` | 生命周期与型变 |
| `04_Type_Cast` | 类型转换 |
| `05_Uninit_Mem` | 未初始化内存 |
| `06_OBRM_RAII` | 所有权与 RAII（OBRM） |
| `07_Panic_Safety` | Panic 安全 |
| `08_Concurrency_Atomic` | 并发与原子 |
| `09_Impl_Vec_Arc` | 实现 Vec / Arc |
| `10_FFI` | 外部函数接口 |
| `11_NoStd` | 无标准库 |

| 资料 | 路径 |
|------|------|
| Nightly 笔记 | [Nightly/notes.md](./Nightly/notes.md) |
| Nightly 源码 | [Nightly/src/](./Nightly/src/) |
| Stable 笔记 | [Stable/notes.md](./Stable/notes.md) |
| Stable 源码 | [Stable/src/](./Stable/src/) |

## 编译示例

```bash
# 进入某一章后（待各章 Cargo.toml 补齐）
cd Rust_Nomicon/Nightly
rustup run nightly cargo build --manifest-path src/09_Impl_Vec_Arc/Cargo.toml

cd Rust_Nomicon/Stable
cargo build --manifest-path src/09_Impl_Vec_Arc/Cargo.toml
```

与本仓库其它板块对照：`atomic/`（Ch08 并发）、`RFR/`（类型与 unsafe 进阶）、`rust_network_programming/stage08`（TLS）。
