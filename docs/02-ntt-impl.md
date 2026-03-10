# FFT 基本原理
DFT是傅里叶变换在时域和频域上都呈离散的形式

对于某一序列$\{x_{n}\}_{n=0}^{N-1}$，其满足有限性条件，它的DFT如下：
$$
X_{k}=\sum\limits_{n=0}^{N-1}x_{n}e^{-i\frac{2\pi}{N}kn}
$$

我们通常用符号$\mathcal{F}$来表示这个变换，即$\hat{x}=\mathcal{F}x$

其逆离散傅里叶变换（IDFT）如下：
$$
x_{n}=\frac{1}{N}\sum\limits_{k=0}^{N-1}X_{k}e^{i\frac{2\pi}{N}kn}
$$

可以计为$x=\mathcal{F}^{-1}\hat{x}$


由于$e^{-i\frac{2\pi}{N}kn}$可以看做是单位根$e^{-i\frac{2\pi}{N}k}$的$n$次方，这是一个十分特殊的形式，因此我们可以将$x_{n}$看作是多项式$a_{0}+a_{1}y+\dots+a_{n}y^{n}$中的系数$a_{n}$，那么$X_{k}$可以看作是多项式在单位根上的求值（值评估）

同时注意到，这种求和本质上是一个线性运算，因此可以变为矩阵的表示
$$
\begin{bmatrix}
X_0 \\
X_1 \\
X_2 \\
\vdots \\
X_{N-1}
\end{bmatrix}
=
\begin{bmatrix}
1 & 1 & 1 & \cdots & 1 \\
1 & \alpha & \alpha^2 & \cdots & \alpha^{N-1} \\
1 & \alpha^2 & \alpha^4 & \cdots & \alpha^{2(N-1)} \\
\vdots & \vdots & \vdots & \ddots & \vdots \\
1 & \alpha^{N-1} & \alpha^{2(N-1)} & \cdots & \alpha^{(N-1)(N-1)}
\end{bmatrix}
\begin{bmatrix}
x_0 \\
x_1 \\
x_2 \\
\vdots \\
x_{N-1}
\end{bmatrix}
$$
其中$\alpha=e^{-i\frac{2\pi}{N}}$

## 分治FFT
举例来说，对于一个7次（N=8）多项式
$$
f(x)=a_{0}+a_{1}x+a_{2}x^{2}+a_{3}x^{3}+a_{4}x^{4}+a_{5}x^{5}+a_{6}x^{6}+a_{7}x^{7}
$$

我们可以按照奇偶来划分，然后对奇数系数序号的四项提出$x$这个公因子
$$
\begin{aligned}
f(x) &= (a_0 + a_2x^2 + a_4x^4 + a_6x^6) + (a_1x + a_3x^3 + a_5x^5 + a_7x^7) \\
&= (a_0 + a_2x^2 + a_4x^4 + a_6x^6) + x(a_1 + a_3x^2 + a_5x^4 + a_7x^6)
\end{aligned}
$$

> [!question] N是什么？
> N在这里表示的是多项式的系数数量，对于7次多项式，其系数数量就是8

我们可以用奇偶次项建立新的函数：
$$
\begin{aligned}
G(x) &= a_0 + a_2x + a_4x^2 + a_6x^3 \\
H(x) &= a_1 + a_3x + a_5x^2 + a_7x^3
\end{aligned}
$$

那么原先的$f(x)$可以用新的函数表示：
$$
f(x) = G(x^2) + x \times H(x^2)
$$

由于我们的单位根有如下性质：
$$
\begin{align}
\omega_{N}^{k}=e^{-i\frac{2\pi}{N}k}
\end{align}
$$
$$
\begin{align}
\omega_{N}^{k+N/2} &= e^{-i\frac{2\pi}{N}k}e^{-i\frac{2\pi}{N}\frac{N}{2}} \\ 
&=-e^{-i\frac{2\pi}{N}k} \\
&=-\omega_{N}^{k}
\end{align}
$$

且$G(x^{2})$与$H(x^{2})$是偶函数，因此我们可以知道$w_{N}^{k}$和$\omega_{N}^{k+N/2}$在$G(x^{2})$上的评估值是相同的，在$H(x^{2})$中也遵循同样的规律

因此我们有
$$
\begin{aligned}
f(\omega_N^k) &= G((\omega_N^k)^2) + \omega_N^k \times H((\omega_N^k)^2) \\
&= G(\omega_N^{2k}) + \omega_N^k \times H(\omega_N^{2k}) \\
&= G(\omega_{N/2}^k) + \omega_N^k \times H(\omega_{N/2}^k)
\end{aligned}
$$

以及
$$
\begin{aligned}
f(\omega_N^{k+N/2}) &= G(\omega_N^{2k+N}) + \omega_N^{k+N/2} \times H(\omega_N^{2k+N}) \\
&= G(\omega_N^{2k}) - \omega_N^k \times H(\omega_N^{2k}) \\
&= G(\omega_{N/2}^k) - \omega_N^k \times H(\omega_{N/2}^k)
\end{aligned}
$$

这样子，我们在求出$G(\omega_{N/2}^{k})$以及$H(\omega_{N/2}^{k})$后，就可以同时求出$f(\omega_N^k)$以及$f(\omega_N^{k+N/2})$，然后对$G$和$H$分别递归进行DFT即可

[快速傅里叶变换(蝶形变换)-FFT](https://zhuanlan.zhihu.com/p/374489378)
由于$\omega_N^k$与$\omega_N^{k+N/2}$正好隔着一个半周期，因此如果我们将这个递归过程画出来，我们会发现，这个过程很像蝴蝶，这正是蝶形变换的来源。
以下是N=4的例子
![NTT蝶形变换](attachments/ntt_butterfly.png)

下图则被称为一个蝶形运算单元
![NTT蝶形单元](attachments/a_butterfly_unit.png)

以下是N=8的例子
![NTT蝶形变换N=8](attachments/8_ntt_butterfly.png)

## 蝶形变换FFT
[快速傅里叶变换(蝶形变换)-FFT](https://zhuanlan.zhihu.com/p/374489378)

除了使用递归的方法，我们也可以使用递推的方法。

### 多项式的层级划分 ($N=8$) 
我们将分解过程分为 4 个层级（Layer）： 
1. **初始层**：待划分的完整多项式 $$f\{a_0, a_1, a_2, a_3, a_4, a_5, a_6, a_7\}$$
2. **第一轮奇偶划分**： $$G\{a_0, a_2, a_4, a_6\}, \quad H\{a_1, a_3, a_5, a_7\}$$
3. **第二轮奇偶划分**： $$GG\{a_0, a_4\}, \quad GH\{a_2, a_6\}, \quad HG\{a_1, a_5\}, \quad HH\{a_3, a_7\}$$
4. **最细粒度（叶子节点）**： $$\{a_0\},\{a_4\},\{a_2\},\{a_6\},\{a_1\},\{a_5\},\{a_3\},\{a_7\}$$

**层级解析：** 
* **第 2 层**：根据经典的奇偶划分，$G$ 包含偶数索引系数，$H$ 包含奇数索引系数。 
* **第 3 层**：对 $G$ 和 $H$ 继续进行奇偶划分。例如 $G(x)$ 被划分为 $GG(x^2)$ 和 $x \cdot GH(x^2)$。 
	* *示例*：$GG$ 对应的多项式为 $a_0 + a_4 x$，此时它还是关于 $x$ 的函数。
* **第 4 层**：划分至不再包含 $x$ 的常数项。 
	* *示例*：$GG(x)$ 向下分解为常数 $a_0$ 和 $a_4$。此时 $a_i$ 已被视为独立的频域分量（点值）。

### 自底向上的合并（递推） 
迭代法的核心是**逆向还原**上述过程。
我们需要将第 4 层的常数项两两合并，最终还原为第 1 层的结果。

> [!warning] 关键点：单位根的变化 
> 在合并过程中，随着多项式阶数 $N$ 的每一层变化，对应的单位根 $\omega_{N}^{k}$ 也在变化。
> * **底层合并**：处理 $N=2$ 的规模，使用 $\omega_{2}^{0}$。 
> * **顶层合并**：处理 $N=8$ 的规模，使用 $\omega_{8}^{k}$。

### 数据重排：位逆序置换 (Bit-Reversal Permutation) 
观察第 4 层的系数排列： $$\{a_0, a_4, a_2, a_6, a_1, a_5, a_3, a_7\}$$**核心发现**： 由于每一层递归都是基于“奇偶”而非简单的“左右”切分，导致最终叶子节点的顺序并不是线性的 $0 \to 7$。为了使用迭代法（自底向上合并），我们需要先将原始数组重排成上述顺序。 

> [!question] 如何高效地重排系数？ 
> 我们可以使用**位逆序置换（Bit-Reversal Permutation）**算法。 
> **原理**：对于 $N=2^n$，将下标的二进制表示进行翻转，即可得到其在叶子节点中的最终位置。 
> * **例 1**：原下标 $2$ (二进制 `010`) $\rightarrow$ 翻转后 `010` $\rightarrow$ 新位置 $2$。 
> * **例 2**：原下标 $4$ (二进制 `100`) $\rightarrow$ 翻转后 `001` $\rightarrow$ 新位置 $1$。 
> 
> *注意：观察上面的序列，原下标 4 的 $a_4$ 确实出现在了数组索引 1 的位置（即第 2 个位置）。*

> [!note] 规律与证明线索 
> 观察系数的分布规律： 
> * $a_0$ 与 $a_1$ 在第 1 次划分时分开，最终它们的物理距离差为 4 ($N/2$)。 
> * $a_0$ 与 $a_2$ 在第 2 次划分时分开，最终它们的物理距离差为 2 ($N/4$)。 
> * 以此类推，第 $k$ 次划分分开的元素，最终距离为 $N/2^k$。 
> 
> 利用这个序号差与 $a_0$ 的绝对位置，我们可以确定所有参数的最终位置

### 蝶形变换的优势
经过变换后，我们可以将第一层的运算结果存储在一个数组中，然后逐渐两两合并，最终得到计算结果，达到节约空间复杂度的目的（递归方法空间比这个方法多得多）

# NTT 基本原理
在数学中，NTT 是关于任意 [环](https://oi-wiki.org/math/algebra/basic/#%E7%8E%AF) 上的离散傅立叶变换（DFT）。在有限域的情况下，通常称为数论变换（NTT）。

**数论变换**（number-theoretic transform, NTT）是离散傅里叶变换（DFT）在数论基础上的实现；**快速数论变换**（fast number-theoretic transform, FNTT）是 [快速傅里叶变换](https://oi-wiki.org/math/poly/fft/)（FFT）在数论基础上的实现。

在FFT中，有两个单位根的两个性质是非常重要的，它们分别是：
- 周期性：$\omega_{N}^{N}=1$
- 消去律：$\omega_{N}^{N/2}=-1$

而在$\mathbb{Z}_{p}$的世界中，我们需要的原根则是
$$
\begin{align}
&g_{N}=g^{q} (mod~p), \\
&p=qN+1, \\
&N=2^{m}
\end{align}
$$
注意，$p$是一个质数，$N$在这里仍然表示多项式系数的数量，也间接着表示多项式的最高次数

由于$p-1=qN$，根据费马小定理，对于质数$p$，我们有$g^{p-1}\equiv 1(mod~p)$，因此，我们有
$$
g^{qN}\equiv 1(mod~p)
$$
也就是
$$
\begin{align}
g_{N}^{N}\equiv 1(mod~p)
\end{align}
$$


而由于$(g_{N}^{N/2})^{2}=g_{N}^{N}\equiv 1(mod~p)$，在模数的世界中，平方为1的数只有1或者-1（$p-1$），而由于原根的限制是$N$为满足$g_{N}^{N}\equiv 1(mod~p)$的最小数，因此$g_{N}^{N/2}$只能等于-1

所以，$g_{n}$具有如下的特性
$$
\begin{align}
g_{N}^{N}\equiv 1(mod~p) \\
g_{N}^{N/2}\equiv -1(mod~p)
\end{align}
$$

这些特性决定了我们能够使用类似FFT的方法加速运算

对于质数$p$，我们能找到足够的满足要求$p$进行计算，常见的$p$如下：
![[Pasted image 20251120175440.png]]

有些时候，N是非常大的，然而我们不需要那么多的多项式参数，因此我们可以令单位根为$g_{n}=g^{\frac{qN}{n}}$，然后我们有

$$
\begin{align}
g_{n}^{n}=g^{(qN/n)\cdot n}\equiv 1(mod~p) \\
g_{n}^{n/2}=g^{(qN/n)\cdot (n/2)}=g^{qN/2}\equiv -1(mod~p)
\end{align}
$$

使用变换后的单位根，相当于缩减了N的大小

## NTT的乘法
NTT在计算乘法的时候需要补零，原因在于多项式乘法时，多项式系数会加倍，因此导致原先的n个点无法还原2n长度的结果
![Pasted image 20251120214406.png](attachments/Pasted%20image%2020251120214406.png)

从卷积的角度来看，不补0会发生卷积混叠，也叫循环卷积
![Pasted image 20251120214454.png](attachments/Pasted%20image%2020251120214454.png)


## CKKS中的特殊NTT(Negacyclic NTT)
这个NTT不需要补零
![Pasted image 20251120214735.png](attachments/Pasted%20image%2020251120214735.png)

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
