# 学习进度日记

## 格式

每条记录包含日期、完成内容、遇到的问题和下一步计划。

---

## 2026-02-21 — 项目初始化

- [x] 创建 workspace 骨架
- [x] 创建所有 crate 目录结构
- [x] 编写骨架代码（含 todo!() 和双语学习链接）
- [ ] 开始实现 fhe-math/modular.rs

**下一步**: 实现 `mod_pow`, `mod_inv`（已完成），然后推进 Barrett reduction 与 Montgomery 上下文初始化。

---

## 2026-03-10 — 完成 `fhe-math::modular` 第一阶段

- [x] 实现基础模运算：`mod_add`、`mod_sub`、`mod_mul`
- [x] 实现快速幂：`mod_pow`
- [x] 实现两种模逆：`mod_inv`、`general_mod_inv`
- [x] 实现 `BarrettReducer::new/reduce/mul_reduce`
- [x] 实现 Montgomery 上下文与 REDC/进出 Montgomery 域/模乘
- [x] 为基础模运算、Barrett、Newton lifting、Montgomery、primitive root 补齐单元测试

**遇到的问题**:
- `mod_add` 和 `mod_sub` 不能只按“小模数”思路写，否则在接近 `u64::MAX` 的模数下会有溢出风险。
- `MontgomeryParams` 这个名字偏泛，放在本项目里容易和 CKKS 参数类型混淆；它更像一个固定模数下的 Montgomery 运算上下文。

**本次调整**:
- 将 `MontgomeryParams` 重命名为 `MontgomeryContext`。
- 将 `mod_add`、`mod_sub` 改成对 64 位边界安全的实现。
- 为 `mod_pow` 增加 `m == 0/1` 的边界处理。

**下一步**:
- 进入 `fhe-math::ntt`，优先实现 `NttPlan::forward` 和 `NttPlan::inverse`
- 然后推进 `poly` 中的 negacyclic NTT 与多项式乘法
- 最后再完成 `rns`，为后续 `ckks/encoding` 打基础
