## 1. 环境准备与依赖

- [x] 1.1 研究 Pico ZKVM 的 Rust crate 名称和 API
- [x] 1.2 在 Cargo.toml 中添加 Pico ZKVM 依赖
- [x] 1.3 创建 ZK 电路模块目录结构

## 2. ZK 电路实现

- [x] 2.1 实现图像哈希计算电路
- [x] 2.2 实现 Crop 转换的 ZK 电路
- [x] 2.3 实现 Resize 转换的 ZK 电路
- [x] 2.4 实现 Brightness 调整的 ZK 电路
- [x] 2.5 实现组合转换的 ZK 电路

## 3. 证明生成实现

- [x] 3.1 更新 ProofInput 和 TransformationProofParams 结构
- [x] 3.2 实现基于 Pico ZKVM 的 generate_proof 函数
- [x] 3.3 实现证明输出序列化为 JSON
- [x] 3.4 添加错误处理和验证

## 4. 证明验证实现

- [x] 4.1 实现基于 Pico ZKVM 的 verify_proof 函数
- [x] 4.2 实现 VerificationResult 结构
- [x] 4.3 添加错误处理和文件验证

## 5. 测试与集成

- [x] 5.1 编写 ZK 电路单元测试
- [x] 5.2 编写证明生成集成测试
- [x] 5.3 编写证明验证集成测试
- [x] 5.4 更新 CLI 命令集成

## 6. 文档与示例

- [ ] 6.1 更新 README.md 中的 ZK 证明说明
- [ ] 6.2 创建使用示例
