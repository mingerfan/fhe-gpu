# CKKS 密钥生成

## 密钥层次

- 秘密钥 sk: 三元多项式 s
- 公钥 pk = (-as + e, a)
- 重线性化密钥 rlk
- Galois 密钥 gk_k

## RLWE 假设

TODO: 填写 RLWE 困难问题描述

## 误差采样

TODO: 填写离散高斯分布采样方法

## 代码对应

- `crates/ckks/src/crypto/keygen.rs`

## TODO

- [ ] 实现 `sample_error`
- [ ] 实现 `gen_sk`
- [ ] 实现 `gen_pk`
- [ ] 实现 `gen_rlk`
- [ ] 实现 `gen_gk`
