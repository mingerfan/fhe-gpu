# CKKS FHE 学习型编译器 — 项目概览

## 目标
从零在 Rust 中实现 RNS-CKKS 全同态加密，兼顾学习、测试与未来生产。

## 学习路径

```
fhe-math (数学基础)
  → ckks/core (参数、密钥、密文)
  → ckks/encoding (编码/解码)
  → ckks/crypto (密钥生成、加密、解密)
  → ckks/eval (同态运算)
  → fhe-ir + fhe-compiler (编译器)
```

## 前置知识

- 线性代数基础（矩阵运算、特征值）
- 抽象代数基础（群、环、域）
- 数论基础（模运算、中国剩余定理、原根）
- 信号处理基础（DFT/FFT）

## Crate 依赖图

```
fhe-math → ckks
fhe-ir (独立，不依赖 ckks)
fhe-compiler → ckks + fhe-ir
fhe-testing (dev-only)
```

## 验证层级

| 层级 | 工具 |
|------|------|
| 单元 | `cargo test` (默认 `#[ignore]`) |
| 集成 | feature flags: phase-math, phase-encoding, phase-crypto, phase-eval |
| 差分 | `RUN_DIFFTESTS=1 cargo test --features difftest` |

## TODO

- [ ] 填写每个模块的学习笔记
- [ ] 完成 fhe-math 实现
- [ ] 完成 ckks 实现
