# 密钥切换：重线性化与旋转

## 重线性化（Relinearization）

TODO: 数字分解方案推导

## Galois 自同构

TODO: φ_k(f(x)) = f(x^k) mod (x^n + 1) 的推导

## 旋转实现步骤

1. 计算 Galois 元素 k = 5^step mod 2n
2. 对密文施加自同构
3. 用 Galois 密钥切换回原秘密钥

## 代码对应

- `crates/ckks/src/eval/relin.rs`
- `crates/ckks/src/eval/rotate.rs`

## TODO

- [ ] 实现 `relinearize`
- [ ] 实现 `apply_automorphism`
- [ ] 实现 `rotate_slots`
