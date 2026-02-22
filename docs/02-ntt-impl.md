# NTT 实现细节

## Cooley-Tukey 蝴蝶操作

TODO

## 旋转因子预计算

TODO

## 负循环 NTT（Negacyclic NTT）

TODO: 解释扭转因子（twist factor）的作用

## 代码对应

- `crates/fhe-math/src/ntt.rs`: `NttPlan::forward`, `NttPlan::inverse`
- `crates/fhe-math/src/poly.rs`: `Poly::ntt_forward`, `Poly::ntt_inverse`

## TODO

- [ ] 实现 `NttPlan::forward`
- [ ] 实现 `NttPlan::inverse`
- [ ] 通过 `test_ntt_roundtrip` 测试
