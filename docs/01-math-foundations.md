# 数学基础

## 巴雷特约简

目的是不用除法，进行模约减。
可以参考：https://zhuanlan.zhihu.com/p/621388087

Given modules $m$, input $x$, for $0 < x < m^2$, m is not power of 2, we precompute: $k = \lceil \log_2 m \rceil$, $magic=\lfloor2^{2k}/m\rfloor$.

```text
barrett_reduce(x):
   q  = (x * magic) >> (2k)      -- approximation of floor(x / m)
   r  = x - q * m                -- remainder (may be slightly off)
   if r >= m: r -= m             -- correction step
   return r
```
数学原理如下：
$$
\begin{aligned}
&x - qm = x - \lfloor x/m \rfloor m = r \\
&let~1/m=magic/2^{k1},~then~calculate~magic=2^{k1}/m, \\
&\lfloor magic \rfloor = 2^{k1}/m - \epsilon, ~where~0 \leq \epsilon < 1\\
&so, magic=\lfloor magic\rfloor + \epsilon\\
&\lfloor x\cdot 1/m\rfloor = \lfloor x\cdot magic/2^{k1} \rfloor\\
&= \lfloor x\cdot (\lfloor magic\rfloor + \epsilon)/2^{k1} \rfloor \\
&= \lfloor x\lfloor magic\rfloor/2^{k1} + x\epsilon/2^{k1} \rfloor \\
\end{aligned}
$$
k1 can be chosen abitrarily, however, to ensure simplicity, $x\epsilon/2^{k1}$ must less than 1, so we can choose $k1 = \lceil \log_2 (max\_input) \rceil$. In mul_reduce, $0 < x < m^2$, so we can choose $k1 = \lceil 2\log_2 m \rceil$.

Because $x\epsilon/2^{k1} < 1$，$\lfloor x\lfloor magic\rfloor/2^{k1} + x\epsilon/2^{k1} \rfloor = \lfloor x\lfloor magic\rfloor/2^{k1}\rfloor ~or~ \lfloor x\lfloor magic\rfloor/2^{k1}\rfloor +1$

so, we can calculate $q = \lfloor x\cdot magic/2^{k1} \rfloor$ and $r = x - qm$, if $r \geq m$, we can do one more subtraction to get the correct result.

数学上本质是使用一个大致的值替代原先的q，然后再计算这个值带来的误差，最后进行修正。

## 多项式环 R_q

TODO: 填写 Z_q[x]/(x^n+1) 的代数结构

## NTT 原理

TODO: 填写 NTT 原理

## 参考资料

- [EN] https://cp-algorithms.com/algebra/
- [CN] https://oi-wiki.org/math/
