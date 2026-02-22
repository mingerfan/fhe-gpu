# CKKS 加密与解密

## 公钥加密

TODO: 填写加密公式推导

## 对称加密

TODO

## 解密

```
m̃ = [c0 + c1 * s]_{Q_l}
```

## 噪声分析

TODO: 新鲜密文的噪声界

## 代码对应

- `crates/ckks/src/crypto/encrypt.rs`
- `crates/ckks/src/crypto/decrypt.rs`

## TODO

- [ ] 实现 `encrypt`
- [ ] 实现 `encrypt_symmetric`
- [ ] 实现 `decrypt`
