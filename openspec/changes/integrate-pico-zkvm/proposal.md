## Why

当前 ZK 证明生成只是一个占位符实现，无法真正验证图像编辑的完整性。用户需要一种机制来证明图像编辑的完整性，而不暴露具体的转换参数（如裁剪坐标、缩放尺寸、亮度调整值等）。Pico ZKVM 是一个轻量级的零知识证明虚拟机，可以集成到 Rust 项目中来实现这一功能。

## What Changes

1. 集成 Pico ZKVM 到项目中作为 ZK 证明生成的后端
2. 实现图像编辑转换的 ZK 电路，证明：
   - 编辑后的图像来源于原始图像
   - 仅应用了声明的转换操作
   - 转换参数被保密（零知识）
3. 添加证明验证功能，无需访问原始图像即可独立验证
4. 更新 CLI 命令以支持新的 ZK 证明流程

## Capabilities

### New Capabilities

- `zk-proof-generation`: 使用 Pico ZKVM 生成图像编辑的零知识证明
- `zk-proof-verification`: 独立验证 ZK 证明，无需原始图像

### Modified Capabilities

- `zk-proof`: 现有占位符实现需要替换为真正的 Pico ZKVM 集成

## Impact

- 新增依赖：Pico ZKVM crate
- 修改模块：`src/zk_proof/mod.rs`
- 修改 CLI：`src/main.rs` 中的 `prove` 和 `verify-proof` 命令
- 新增测试和示例文件
