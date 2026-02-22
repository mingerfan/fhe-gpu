# CKKS 编码：规范嵌入

## 核心思想

CKKS 将 n/2 个复数打包进一个次数为 n 的多项式，通过规范嵌入（Canonical Embedding）实现。

## 数学公式

TODO: 填写 IDFT 编码步骤

## 精度分析

TODO: 填写编码精度与比特长度的关系

## 代码对应

- `crates/ckks/src/encoding/encoder.rs`
- `crates/ckks/src/encoding/slots.rs`

## TODO

- [ ] 实现 `CkksEncoder::encode`
- [ ] 实现 `CkksEncoder::decode`
- [ ] 通过 `test_encode_decode_roundtrip`
