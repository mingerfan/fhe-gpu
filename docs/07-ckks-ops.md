# CKKS 同态运算

## 同态加法

```
(c0, c1) + (d0, d1) = (c0 + d0, c1 + d1)
```
代价：一次多项式加法，噪声几乎不增长。

## 同态乘法

TODO: 张量积推导

## Rescale

TODO: 除以最后一个 RNS 素数，将 Δ² 还原为 Δ

## 噪声分析

TODO

## 代码对应

- `crates/ckks/src/eval/add.rs`
- `crates/ckks/src/eval/mul.rs`
- `crates/ckks/src/eval/rescale.rs`

## TODO

- [ ] 实现所有 eval 函数
- [ ] 通过集成测试
