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

## 

## Montgomery 约简
蒙哥马利模乘的目的：
正常情况下，我们想计算模乘$a \cdot b~mod~N$需要引入除法，然而，在计算机中，除法的效率较低，因此蒙哥马利模乘通过引入一个新的模数$R$（通常是$2^k$，其中$k$大于$N$的位数）来避免直接的除法操作。

### 表示前提与蒙哥马利表示
蒙哥马利乘要求$gcd(R,N)=1$，$R > N$, 由于R通常为一个偶数，因此N通常为一个奇数

蒙哥马利表示：对于一个整数$x$，其蒙哥马利表示为$\tilde{x} = xR \mod N$

### 蒙哥马利乘法算法
首先不加证明的引入蒙哥马利约简：
$$
REDC(t) = tR^{-1} \mod N
$$

$$
\tilde{a} \cdot \tilde{b} = (aR \mod N) \cdot (bR \mod N) = abR^2 \mod N \\
REDC(abR^2) = abR \mod N \\
ab = REDC(abR)
$$

### 蒙哥马利约简算法及其证明
$$
REDC(t) = tR^{-1} \mod N, 0 < t < NR
$$
然而，我们需要避免直接模N，因此需要使用如下方法：
1. 计算$N'$, 使得$NN' = -1 \mod R$, 也就是说$N' = -N^{-1} \mod R$
2. 计算$m = ((t \mod R) \cdot N') \mod R$
3. 计算$u = (t + mN) / R$
4. 如果$u \geq N$, 则返回$u - N$, 否则返回$u$

证明：
1. $(t + mN)$可以被$R$整除：
$$
\begin{aligned}
&mN+t\mod R\\
&=[(t \mod R) \cdot N^{'} \cdot N +t] \mod R\\
&=[t\cdot (-1) + t] \mod R\\
&=0
\end{aligned}
$$
注意，由于$(a + b) \mod q \equiv ((a\ mod q) + (b \mod q)) \mod q$，我们将内部的$\mod R$省略掉了

2. $u = tR^{-1} \mod N$:  
$$
\begin{aligned}
uR &= (t + mN) \mod N\\
&= t \mod N\\
u &= tR^{-1} \mod N
\end{aligned}
$$

3. 由于$u$与$tR^{-1}$仅仅是在模的意义上相等，而我们计算出的u并没有进行取模，因此要保证结果在$[0, N)$范围内。由于$0 \leq m < R$, $0 < t < NR$, $u = (t + mN) / R < (NR + RN) / R = 2N$，因此，如果$u \geq N$, 则返回$u - N$, 否则返回$u$。

## 如何将普通的整数转换为蒙哥马利表示？
对于一个整数$x$, 我们预计算$R^2 \mod N$, 然后计算REDC($xR^2$)即可得到$x$的蒙哥马利表示。



# Newton/Hensel lifting 牛顿迭代法
## 算法描述
这个算法主要用于快速的计算某个数的模逆，通过迭代的方式，我们无需像扩展欧几里得算法那样，需要可容纳$2^128$的变量来计算$2^{64}$的模逆。

该迭代法的一个特例表述如下：
$$
\begin{aligned}
&\text{Given a odd number } a, \\
&ax_{i} \equiv 1 \mod 2^{2^{i}} \\
&\text{where } x_{i+1}=x_{i}(2-ax_{i}) \mod 2^{2^{i+1}},\quad x_{0}=1
\end{aligned}
$$

通过这个公式，我们就可以从$x_0$开始迭代，快速地计算出$a$的模逆。

## 算法证明
我们通过数学归纳法来证明这个迭代公式的正确性。

1. 当$i=0$时，$ax_0 = a \equiv 1 \mod 2$, 因为$a$是一个奇数，所以这个等式成立。
2. 假设对于某个$i$，$ax_i \equiv 1 \mod 2^{2^i}$成立，我们需要证明对于$i+1$，$ax_{i+1} \equiv 1 \mod 2^{2^{i+1}}$也成立。
根据迭代公式，我们有：
$$
e=1-ax_{i} \equiv 0 \mod 2^{2^{i}}
$$
$$
\begin{aligned}
ax_{i+1} &= ax_{i}(2-ax_{i}) \\
&= (1-e)(2-(1-e)) \\
&= 1-e^2 \mod 2^{2^{i+1}}
\end{aligned}
$$
因此，我们有
$$
e^{2} = 1 - ax_{i+1} \mod 2^{2^{i+1}}
$$
考虑到$e \equiv 0 \mod 2^{2^i}$，我们可以得出$e^2 \equiv 0 \mod 2^{2^{i+1}}$，因此$ax_{i+1} \equiv 1 \mod 2^{2^{i+1}}$。

## 递推式$x_{i+1}=x_{i}(2-ax_{i}) \mod 2^{2^{i+1}}$是如何得到的？
从某种意义上来说，这是一个“猜出来”的递推式。我们以一个简单但是不严谨的方式来说明这一点。

首先构造$e=1-ax$, $x^{'}=x+\delta$, $e^{'}=1-ax^{'}$
由构造，我们可得
$$
\begin{aligned}
e^{'} &= 1 - a(x + \delta) \\
&= 1 - ax - a\delta \\
&= e - a\delta
\end{aligned}
$$

实际上，$\delta$是可以任意选择的，但是为了实现平方递推的效果，我们需要有$e^{'} \equiv e^2 \mod 2^{2^{i+1}}$，因此:
$$
e^{2} = e - a\delta
$$

$$
\begin{aligned}
a\delta &= e(1-e) \\
a\delta &= (1-ax)(ax) \\
\delta &= x(1-ax) \\
\end{aligned}
$$

因此，我们得到了递推式：
$$
x^{'} = x + \delta = x + x(1-ax) = x(2-ax)
$$

上面推导给出恒等式 $e' = e^2$。当我们关心的是模 $2^t$ 的逆元时，若 $2^k \mid e$，则 $2^{2k}\mid e'$，因此可将每一步结果按模 $2^t$ 截断（如 64 位 wrapping），即可得到逐步提升精度的迭代算法。

## 注意
虽然迭代是$2^{2^{i}}$递增的，但由于
- $ax ≡ 1 (mod 2^8)$

那它自动也满足：

- $ax ≡ 1 (mod 2^3)$
- $ax ≡ 1 (mod 2^5)$
- $ax ≡ 1 (mod 2^7)$

因此我们只需要向下取整到$2^i$，就可以得到满足$ax ≡ 1 (mod 2^i)$的结果。

## 多项式环 R_q

TODO: 填写 Z_q[x]/(x^n+1) 的代数结构

## NTT 原理

TODO: 填写 NTT 原理

## 参考资料

- [EN] https://cp-algorithms.com/algebra/
- [CN] https://oi-wiki.org/math/
