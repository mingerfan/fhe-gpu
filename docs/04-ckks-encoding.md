# CKKS 编码：规范嵌入

## 核心思想

CKKS 将 n/2 个复数打包进一个次数为 n 的多项式，通过规范嵌入（Canonical Embedding）实现。

## Encoding & Decoding概述
[CKKS explained, Part 2: Full Encoding and Decoding – OpenMined](https://openmined.org/blog/ckks-explained-part-2-ckks-encoding-and-decoding/)

Encoding主要做的事情就是把message从vector的形式变为plaintext的形式，也就是
$$
C^{\frac{N}{2}}=>Z[X]/(X^{N}+1)
$$
其中，$X^{N}+1$是一个（2N次的）分圆多项式，也是一个不可约多项式

> [!summary] 
> ***最终的编码过程如下：***
>- $z \in C^{N/2}$
>- $\pi^{-1}(z) \in \mathbb{H}$
>- $\Delta \pi^{-1}(z)$
>- 投射到$\sigma(R)$中：$\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)} \in \sigma(R)$
>- 用$\sigma$进行编码：$m(X)=\sigma^{-1}(\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)}) \in R$
>
>***最终的解码过程如下：***
>$z=\pi \circ \sigma(\Delta^{-1}\cdot m)$


## 前置数学知识
### canonical embedding 典范嵌入
典范嵌入是一类泛指的嵌入，指的是 ***原环元素在扩张结构中的”自然对应“***
简单的典范嵌入有$Z->Q$
我们处理的典范嵌入则是如下图所示：
$$
\sigma:C[X]/(X^{N}+1)=>C^{N}
$$
典范嵌入将分圆多项式$\Phi_{M}(X)=X^{N}+1$的各个根$\xi,\xi^{3},...,\xi^{2N-1}$带入目标多项式$C[X]/(X^{N}+1)$中逐个evaluate，然后得到的根组合成$C^{N}$
也即：
$$
\begin{aligned}
\sigma(m)&=(m(\xi),m(\xi^{3}),...,m(\xi^{2N-1}))\\
&= (z_{1},...,z_{N})
\end{aligned}
$$
注意，这里的根是从1到2N-1而不是N的

典范嵌入σ定义了一个同构（也就是说它定义了一个双射同态），在计算上它是同态的，在映射上是双射的

## 精度分析

TODO: 填写编码精度与比特长度的关系

## 代码对应

- `crates/ckks/src/encoding/encoder.rs`
- `crates/ckks/src/encoding/slots.rs`

## TODO

- [ ] 实现 `CkksEncoder::encode`
- [ ] 实现 `CkksEncoder::decode`
- [ ] 通过 `test_encode_decode_roundtrip`
