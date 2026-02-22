# RNS（剩余数系统）算术

## 为什么需要 RNS

CKKS 需要数百比特的模数 Q，RNS 将 Q 分解为多个小素数之积，
使所有运算保持在 64 位整数范围内。

## CRT（中国剩余定理）

TODO: 填写 CRT 的数学描述

## ModUp / ModDown

TODO: 填写 ModDown 算法（用于 Rescale）

## Garner 算法

TODO: 填写 CRT 重建算法

## 代码对应

- `crates/fhe-math/src/rns.rs`: `RnsPoly`, `mod_down`, `crt_reconstruct`

## TODO

- [ ] 实现 `RnsPoly::from_coeffs`
- [ ] 实现 `RnsPoly::mod_down`
