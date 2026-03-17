# CKKS 加密与解密
## LWE 
[LWE加解密流程及实现](https://zhuanlan.zhihu.com/p/480326595)
LWE问题主要是将$(a_{i},b_{i})=(a_{i},\langle a_{i},s \rangle+e_{i})$从真随机的$\mathbb{Z}_{q}^{n}\times \mathbb{Z}_{q}$中区分开，其中$a_{i},s\in \mathbb{Z}_{q}^{n}$，$a_{i}$是均匀采样的，而$s$则是我们的秘密(secret，一般用作secret key)，$e_{i}\in \mathbb{Z}_{q}$而是随机的小噪声

这是一个困难问题，***如果没有噪声 ($e_i = 0$)： 问题会退化为一个标准的线性方程组***：
$$
\mathbf{a}_i \cdot \mathbf{s} = b_i
$$
在这种情况下，我们只需要足够多的 $(\mathbf{a}_i, b_i)$ 对，就可以使用高斯消元法 (Gaussian elimination) 等标准方法，轻松地求解出秘密向量 $\mathbf{s}$。


**有了噪声 ($e_i \neq 0$)：** 噪声使等式变成了**近似关系**，使得标准的线性代数方法失效。求解这个系统在计算上被认为是**困难的 (Hard)**，因为噪声有效地**混淆**了秘密 $s$ 的信息。


我们将$s$作为私钥，然后发布n对$(a_{i},\langle a_{i},s \rangle+e_{i})$，在这种情况下，这些对子可以被写为矩阵形式$(A, A\cdot s+e),A\in \mathbb{Z}_{q}^{n \times n}, e\in \mathbb{Z}_{q}^{n}$，由于我们很难获取到私钥，因此我们将其这些密钥对作为公钥p，而实际上用的公钥如下所示

$$
p = (-A\cdot s+e, A)
$$

我们的message $\mu\in \mathbb{Z}_{q}^{n}$在LWE的加密即是如下：
$$
c=(\mu,0)+p=(\mu-A\cdot s+e,A)=(c_{0},c_{1})
$$

而我们的解密则是如下所示：
$$
\tilde{\mu}=c_{0}+c_{1}\cdot s=\mu-A\cdot s+e+A\cdot s=\mu+e\approx \mu
$$

LWE最大的问题是效率低

## RLWE
与LWE在$\mathbb{Z}_{q}^{n}$上工作不同，RLWE在$\mathbb{Z}_{q}[X]/(X^{N}+1)$上工作
我们有$a,s,e\in \mathbb{Z}_{q}[X]/(X^{N}+1)$，其中，$a$是均匀采样，$s$是一个小的秘密多项式，$e$是一个小的噪声多项式
$v$为随机向量，$ct = [c_{0},c_{1}]$，$sk=[1,s]$，$m$表示信息，公钥$pk=[-as+e, a]$，其中$b=-as +e$

$$
\begin{aligned}
c&=[v\cdot pk +(m+e_{0},e_{1})]_{q} \\
c_{0}&=[v\cdot b+m+e_{0}]_{q} \\
c_{1} &= [v\cdot a+e_{1}]_{q} \\
\end{aligned}
$$

解密：
$$
\begin{aligned}
m &\approx[<ct,sk>]_{q} \\
  &=[v(-a\cdot s + e)+m+e_{0}+v\cdot a\cdot s + e_{1}s]_{q} \\
  &=[m+ve+e_{0}+e_{1}s]_{q}
\end{aligned}
$$

我们令$ve+e_{0}+e_{1}s=e$
则$\langle ct, sk \rangle = m+e+qr$，根据mit讲义，$r$是非常小的

## CKKS加密步骤
ckks的加密实际上就是上文提到的RLWE加密步骤，但是有一些细节需要表述清楚

$\mathcal{D}G(\sigma^{2})$：从$Z^{N}$空间中生成的向量随机采样，其每个坐标系数均从方差为$\sigma^{2}$的离散高斯分布中独立抽取

$\mathcal{H}WT(h)$：对于一个正整数$h$，$\mathcal{H}WT(h)$是$\{0,\pm 1\}^{N}$中汉明权重恰好为h的带符号二进制向量集合

> [!note] 什么是汉明权重(Hamming weight)？
> 指的是一个符号串中非零符号的个数，对于二进制数据位串，即串中1的个数

$\mathcal{Z}O(\rho)$：对于$\rho \in [0,1]$，$\mathcal{Z}O(\rho)$从集合$\{0,\pm 1\}^{N}$中抽取向量中的每一个元素，其中-1和+1的概率各为$\rho/2$，而取值为0的概率则是$1-\rho$

$q_{l}=q^{l}\cdot q_{0}$，$0<l\le L$

$\lambda$：安全参数，对于每一个$\lambda$，我们选择一个与$\lambda$和$q_{L}$相关的$M=M(\lambda,q_{L})$作为分圆多项式的$M$

> [!warning] 
> 注意，以上函数采样的是多项式的系数，但是生成结果均表示多项式


### 密钥生成$KeyGen()$
生成一个私钥$sk$，一个公钥$pk$，以及一个评估密钥$evk$
> [!note] 评估密钥的作用
> - 执行重线性化
> - 执行密文旋转

第一步，给定$\lambda$，生成2的幂次$M=M(\lambda,q_{L})$，整数$h=h(\lambda,q_{L})$，整数$P=P(\lambda,q_{L})$以及一个实数$\sigma=\sigma(\lambda,q_{L})$

> [!question] $q_{L}$是什么？
> $$q_L = p_0 \cdot p_1 \cdot p_2 \cdots p_L$$
> **计算预算：** $q_L$ 的大小决定了密文在解密失败前可以进行多少次乘法运算（Rescaling 或 Modulus Switching）。随着同态计算的进行，模数会逐渐从 $q_L$ 减小到 $q_{L-1}, \dots, q_0$。

第二步，采样$s,a,e$，生成私钥与公钥。$s \leftarrow \mathcal{H}WT(h)$，$a \leftarrow \mathcal{R}_{q_{L}}$以及$e \leftarrow \mathcal{D}G(\sigma^{2})$。私钥$sk \leftarrow (1,s)$，公钥$pk \leftarrow (b,a)\in \mathcal{R}_{q_{L}}^{2}$，其中$b \leftarrow -as+e(mod q_{L})$

>[!question] $R_{q_{L}}$是什么？
>$$\mathcal{R}_{q_L} = (\mathbb{Z}_{q_L}[X]) / (X^N + 1)$$

第三步，采样$a^{\prime},e^{\prime}$，生成$evk$。$a^{\prime}\leftarrow \mathcal{R}_{P\cdot q_{L}}$，$e^{\prime}\leftarrow \mathcal{D}G(\sigma^{2})$。而$evk \leftarrow (b^{\prime},a^{\prime})\in \mathcal{R}_{P\cdot q_{L}}^{2}$，其中$b^{\prime}\leftarrow -a^{\prime}s+e^{\prime}+Ps^{2}(mod~P\cdot q_{L})$

### 编码
见解码与编码部分

### 加密$Enc_{pk}(m)$
采样$v \leftarrow \mathcal{Z}O(0.5)$与$e_{0},e_{1}\leftarrow \mathcal{D}G(\sigma^{2})$，输出$v\cdot pk+(m+e_{0},e_{1}) (mod~q_{L})$（注意，$v\cdot pk$输出的是类似$(a,b)$的形式）

### 解密
见RLWE部分

# 噪声分析

TODO: 新鲜密文的噪声界

## 代码对应

- `crates/ckks/src/crypto/encrypt.rs`
- `crates/ckks/src/crypto/decrypt.rs`

## TODO

- [ ] 实现 `encrypt`
- [ ] 实现 `encrypt_symmetric`
- [ ] 实现 `decrypt`
