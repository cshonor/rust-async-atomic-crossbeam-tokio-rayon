# IR 样本归档（`ir_samples/`）

从 **`llvm_insight/src/lib.rs`** 或从 **`atomic/`**、**`async_tokio/`**、**`rust_network_programming/`** 相关代码导出 `.ll` 后，将**脱敏、可复现**的片段统一放在此目录，便于 diff 与版本管理。

## 命名建议

- `atomic_ir/`：`ch04_fence_seqcst_O0.ll`、`ch07_release_acquire_opt3.ll`  
- `async_tokio_ir/`：注明对应 `async_tokio/chNN_*/` 源文件与 rustc 版本  
- `network_ir/`：与 `rust_network_programming` 阶段 demo 对应  
- `optimize_compare/`：**成对**存放 `*_O0.ll` / `*_O3.ll` 或 debug vs release  

## Git 注意

- 单文件过大时只提交**截取的相关函数**或压缩 diff 说明到 `notes/`。  
- 可保留 `.gitkeep` 以占位空目录；有样本后删除对应 `.gitkeep` 亦可。
