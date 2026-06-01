# 第 5 章：项目结构 (Project Structure) —— 深度解析

《Rust for Rustaceans》第 5 章把视角从类型与错误处理**抬高到工程化**：如何在 crate 变大、需要对外发布、依赖关系复杂时，仍能用 Cargo 把项目**组织清楚、构建可控、版本可演进**。默认 `cargo new` 够用之后，本章是走向「生态公民」的路线图。

---

## 1. 特性 (Features)：条件编译与可选能力

**Feature** 本质是传给依赖解析与编译器的**构建开关**，用于裁剪/打开能力。

### 1.1 附加性 (Additive) 原则

特性应主要用于**增加**能力，而避免用特性去：

- 删除模块、替换公开类型、或改变函数签名等「互斥语义」。

**原因**：同一依赖在不同上游里启用的 feature 会被 Cargo **取并集**后编译一份。若两路 feature 语义互斥，合并后可能直接无法编译或产生意外组合。

### 1.2 可选依赖与默认特性

- **`optional = true`**：依赖默认不启用，只有打开对应 feature 才拉进构建，可显著缩短默认编译路径。
- **默认特性 (default)**：把「大多数用户都要」的能力放进 `default`。使用者可用 **`default-features = false`** 只开子集，是优化编译与依赖面的常用手段。

---

## 2. 工作区 (Workspaces)：多 crate 协作

巨石单 crate 会导致「改一行注释也触发大范围重算」的体验变差。

### 2.1 工作区是什么

根 **`Cargo.toml`** 中声明 **`[workspace]`**，将多个 member crate 编进同一工作区。

### 2.2 共享什么

- 通常共享 **`Cargo.lock`**（根解析结果一致）。
- 共享 **`target/`** 目录与增量编译产物，减少重复工作。

### 2.3 工作流收益

在根目录 **`cargo test` / `cargo check`** 可一键覆盖成员；增量编译时往往只需重建**变更成员及其反向依赖**，迭代更快。

---

## 3. 项目配置 (Project Configuration)

### 3.1 `[patch]`：临时替换依赖

上游 crate 有 bug 时，可 fork 或本地修复，用 **`[patch]`** 让 Cargo 在解析时优先使用指定 **path / git revision**，便于验证修复再 upstream。

### 3.2 `[profile.*]`：优化与编译权衡

| 选项 | 直觉 |
|------|------|
| **`opt-level`** | `0` 调试友好；`3` 追求速度；`"s"`/`"z"` 偏体积。 |
| **`codegen-units`** | 单元越多并行越好、单单元优化空间越大；常在 release 下调小以换峰值性能（编译更慢）。 |
| **`lto`** | 链接期跨单元/跨 crate 优化，运行时收益大，**编译时间**显著增加。 |

### 3.3 `panic = "abort"`

- **`panic = "abort"`**：panic 时不展开栈，可减小体积、简化部分嵌入式模型。
- 注意：这是**全局策略**，会影响依赖与可恢复性预期，需与团队/平台约定一致。

---

## 4. 条件编译 (Conditional Compilation)

**`#[cfg(...)]`** 让代码或依赖只在特定条件下进入编译：

- **平台**：`#[cfg(windows)]`、`#[cfg(target_os = "macos")]` 等。
- **测试**：`#[cfg(test)]`（仅测试构建）。
- **特性**：`#[cfg(feature = "foo")]`。
- **依赖侧**：在 `Cargo.toml` 里按 target 引入平台专属依赖（与 `cfg` 协同）。

---

## 5. 版本控制 (Versioning)

### 5.1 SemVer 与 MSRV

Rust 生态在 SemVer 之上还有**不成文约定**：提升 **MSRV（最低支持的编译器版本）** 往往按 **minor** 处理（例如 `2.6.0 → 2.7.0`），让旧工具链用户可把依赖**上限锁在 `< 2.7`**，避免无意拉新导致编译失败。

### 5.2 依赖下界（Minimum versions）

把依赖写得过紧（例如实际只需 `1.5.6` 却写 `1.7.3`）会**不必要地收紧**解析空间，与他 crate 的上限冲突。

**实践**：在 `Cargo.toml` 中写**能通过你测试的最低兼容版本**；可用 **`cargo update -Zminimal-versions`**（nightly 不稳定标志）等工具做底线探测（以当前 Cargo 文档为准）。

### 5.3 Changelog

不要只靠 Git log：建议按 **Keep a Changelog** 等规范维护人类可读的 **CHANGELOG**，突出破坏性变更、MSRV、feature 默认值变化等。

---

## 小结

第 5 章讲的是「如何让项目在他人机器上也好用」：**附加性 feature**、**workspace 拆分**、**patch/profile/panic 策略**、**cfg 裁剪**、以及 **SemVer + MSRV + 依赖下界 + 变更日志**。这些决定库的**可组合性、编译成本与升级体验**。

---

## 与仓库目录的对应

- 原书章名：**Project Structure**  
- 本仓库文件夹：`RFR/Chapter-05-Project-Structure/`  
- 全书总目录：`RFR/RFR-本书目录.md`
