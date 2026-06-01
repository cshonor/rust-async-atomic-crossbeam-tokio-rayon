# 第 9 章：不安全代码 (Unsafe Code) —— 深度解析

在 Rust 社区中，`unsafe` 常被误解为「关掉借用检查」。在《Rust for Rustaceans》第 9 章的视角里，`unsafe` 是：**当不变量（invariants）无法由编译器自动证明时，由开发者向编译器作出承诺、并在代码中局部承担证明责任**的机制。它不是安全模型的对立面，而是其**受控延伸**。

---

## 1. `unsafe` 的双重角色

- **`unsafe fn`**：**声明契约**。表示「调用方必须满足额外前提，否则可能 UB」；调用处通常需要 **`unsafe` 块**（或位于另一 `unsafe fn` 体中）来表明调用者已审阅契约。
- **`unsafe { ... }`**：**履行契约**。表示「此块内使用了编译器不单独验证的超能力；我保证块内满足所需不变量」。

借用检查等对 `&T` / `&mut T` 的规则在 `unsafe` 块中**仍然适用**；`unsafe` 放开的是若干类操作（裸指针解引用、调用 `unsafe fn`、`static mut`、`unsafe impl`、访问 `union` 字段等，与 The Book 一致）。

---

## 2. 巨大的权力 (Great Power)

### 2.1 裸指针 `*const T` / `*mut T`

- **无**生命周期与别名保证；可为 null；有效性由开发者证明。
- **典型用途**：生命周期难以静态表达的布局（如与第 8 章相关的自引用状态）、与 `Arc` 等运行时管理寿命的结构、FFI 边界等。
- **`NonNull<T>`**：在「永非 null」成立时，优先于裸指针；利于 **`Option<NonNull<T>>` 的 niche 优化**（与布局/ABI 文档一致时使用）。

### 2.2 调用不安全能力

- **FFI**：`extern "C"` 等声明的函数在 Rust 侧视为需 `unsafe` 调用（外部世界不保证 Rust 不变量）。
- **`_unchecked` API**：如 `get_unchecked`——跳过运行时检查；仅在有**可证明**的边界与可量化的热点收益时考虑。
- **`MaybeUninit<T>`**：处理未初始化存储；**`assume_init`** 等为 `unsafe`，因你必须证明内存已构成**合法 `T` 值**（满足 validity）。

### 2.3 `unsafe trait` 与 `unsafe impl`

若 **仅通过安全代码** 误用某 trait 的实现即可导致内存不安全，则该 trait 应标为 **`unsafe trait`**，实现为 **`unsafe impl`**——你在承诺维持该 trait 的**安全契约**。

- **`Send` / `Sync`**：标准库中的 `unsafe trait`；错误地为含裸指针的类型实现它们，可在**安全代码**中制造数据竞争。

**关于 `Unpin`**：`Unpin` 本身是**安全 trait**；危险来自 **`Pin` 与 `!Unpin` 的组合**（如 `Pin::new_unchecked` 等 API），与第 8 章自引用叙事衔接。

---

## 3. 巨大的责任 (Great Responsibility)：UB 从何而来

UB 不一定立即崩溃；可能表现为静默错误或在编译器升级后暴露。

### 3.1 破坏有效性 (Validity)

类型对「哪些位模式算合法值」有约束，例如：

- **引用**：非 null、对齐、指向已分配且按类型合法的对象（无悬垂等）。
- **`bool`**：内存表示只能是 `0` 或 `1`；非法位型可破坏优化假设并导致灾难性后果。

### 3.2 Panic 与不变式破坏（以 `Vec` 为例）

在维护复杂不变式（如手动改 `len` 再逐元素初始化）时，若中间调用可能 **panic** 的用户代码，栈展开可能触发 **`Drop` 于不一致状态**：例如 `len` 已增大但尾部未初始化，析构可能按「全有效元素」处理——**二次释放 / 读垃圾** 等风险极高。unsafe 边界设计必须考虑 **panic-safety**（常用 `Guard` 模式、`MaybeUninit`、defer 等）。

### 3.3 布局与转换 (Casting / transmute)

**`#[repr(Rust)]`** 不保证字段顺序与布局；即使 `Foo<A>` 与 `Foo<B>`「看起来一样」，也不能假设可安全 `transmute`。跨布局假设是常见 UB 来源；需要稳定布局时用 **`repr(C)`** 等并阅读参考手册。

---

## 4. 应对恐惧 (Coping with Fear)

1. **缩小不安全边界**：把 `unsafe` 封进私有模块，暴露安全 API；凡能修改与 unsafe 假设相关状态的代码，都应视为**同一信任边界**的一部分。
2. **文档化契约**：在每个 `unsafe` 块前用 **`// SAFETY:`**（或 crate 内约定）写明**调用者/维护者**必须维持的前提。
3. **用 Miri 验证**：编写或修改 unsafe 后，在测试上运行 **`cargo miri test`**，捕捉未初始化读、悬垂、别名违规等（以 Miri 支持范围为准）。

---

## 小结

`unsafe` 不是漏洞，而是**显式承担证明责任**的语法标记。优先寻找安全替代；确需底层能力时，以**最小 `unsafe` 块**、**清晰契约**与 **Miri** 把风险关在边界内。

与 The Book 笔记/demo 呼应：`19-advanced-features/19.1-unsafe-rust-demo/`（unsafe 超能力示意）。

---

## 与仓库目录的对应

- 原书章名：**Unsafe Code**  
- 本仓库文件夹：`RFR/Chapter-09-Unsafe-Code/`  
- 全书总目录：`RFR/RFR-本书目录.md`
