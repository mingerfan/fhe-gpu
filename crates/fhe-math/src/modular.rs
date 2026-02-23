//! Modular arithmetic primitives.
//!
//! This module provides:
//! - `mod_add`, `mod_sub`, `mod_mul` — safe wrappers using `u128`
//! - `mod_pow` — fast exponentiation
//! - `mod_inv` — modular inverse via Fermat's little theorem (prime modulus only)
//! - `barrett_reduce` — Barrett reduction for repeated reduction with fixed modulus
//! - `MontgomeryInt` — Montgomery form integers for efficient modular multiplication

use crate::MathError;

// ────────────────────────────────────────────────────────────
// Basic modular arithmetic (simple but correct — use these first)
// ────────────────────────────────────────────────────────────

/// Compute `(a + b) mod m`.
#[inline]
pub fn mod_add(a: u64, b: u64, m: u64) -> u64 {
    debug_assert!(a < m && b < m, "inputs must be reduced mod m");
    let s = a + b;
    if s >= m {
        s - m
    } else {
        s
    }
}

/// Compute `(a - b) mod m`.
#[inline]
pub fn mod_sub(a: u64, b: u64, m: u64) -> u64 {
    debug_assert!(a < m && b < m, "inputs must be reduced mod m");
    if a >= b {
        a - b
    } else {
        a + m - b
    }
}

/// Compute `(a * b) mod m` using 128-bit intermediate.
///
/// This is the "slow but obviously correct" reference implementation.
/// Use `MontgomeryInt` or `barrett_reduce` for hot paths.
#[inline]
pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Compute `base^exp mod m` via binary exponentiation.
///
/// # Mathematical Specification
/// ```text
/// mod_pow(b, e, m) = b^e mod m
/// Uses right-to-left binary method: result *= b when bit is set, b = b^2 each step.
/// ```
///
/// # Learning Resources
/// - [EN] Binary exponentiation (CP-Algorithms): https://cp-algorithms.com/algebra/binary-exp.html
/// - [CN] 快速幂（OI-Wiki）: https://oi-wiki.org/math/quick-pow/
pub fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

/// Compute the modular inverse of `a` modulo prime `m`.
///
/// # Mathematical Specification
/// ```text
/// mod_inv(a, m) = a^(m-2) mod m   [Fermat's little theorem, m prime]
/// ```
///
/// # Panics
/// Panics if `a == 0` (no inverse exists).
///
/// # Learning Resources
/// - [EN] Modular inverse (CP-Algorithms): https://cp-algorithms.com/algebra/module-inverse.html
/// - [CN] 乘法逆元（OI-Wiki）: https://oi-wiki.org/math/inverse/
pub fn mod_inv(a: u64, m: u64) -> u64 {
    assert_ne!(a, 0, "0 has no modular inverse");
    mod_pow(a, m - 2, m)
}

// ────────────────────────────────────────────────────────────
// Barrett Reduction
// ────────────────────────────────────────────────────────────

/// Precomputed Barrett reduction context for a fixed modulus.
///
/// Barrett reduction computes `x mod m` without division by replacing it with
/// multiplications and shifts using a precomputed "magic" constant.
///
/// # Mathematical Specification
/// ```text
/// Given modulus m, input x, 0 < x < m^2, m is not power of 2,
/// precompute: k = ceil(log2(m)), magic = floor(2^(2k) / m)
///
/// barrett_reduce(x):
///   q  = (x * magic) >> (2k)      -- approximation of floor(x / m)
///   r  = x - q * m                -- remainder (may be slightly off)
///   if r >= m: r -= m             -- correction step
///   return r
/// ```
/// # Learning Resources
/// - [EN] Barrett reduction (Wikipedia): https://en.wikipedia.org/wiki/Barrett_reduction
/// - [CN] Barrett 约简（知乎）: https://zhuanlan.zhihu.com/p/621388087
/// - [CN] 高效模运算（CSDN）: N/A
pub struct BarrettReducer {
    pub modulus: u64,
    magic: u128,
    shift: u32,
}

impl BarrettReducer {
    /// Create a new Barrett reducer for `modulus`.
    pub fn new(modulus: u64) -> Self {
        // 安全断言：避免 x * magic 溢出 u128（要求 3k+1 ≤ 128，即 k ≤ 42）
        assert!(
            modulus < (1u64 << 42),
            "BarrettReducer: modulus must be < 2^42, got {modulus}. \
             For larger moduli use mod_mul() which uses u128 division."
        );
        // k = bit length of modulus = floor(log2(m)) + 1，用位运算精确计算
        let k = 64 - modulus.leading_zeros();
        let magic = (1u128 << (2 * k)) / modulus as u128;
        Self {
            modulus,
            magic,
            shift: k,
        }
    }

    /// Reduce `x` modulo `self.modulus`.
    ///
    /// # Implementation Notes
    /// `x` must satisfy `x < modulus^2` for the approximation to be correct.
    ///
    /// # Learning Resources
    /// - [EN] Barrett reduction implementation notes: N/A §2
    /// - [CN] Barrett 约简实现要点: N/A
    pub fn reduce(&self, x: u128) -> u64 {
        let q = (x * self.magic) >> (self.shift * 2);
        let rem = (x - q * self.modulus as u128) as u64;
        if rem >= self.modulus {
            rem - self.modulus
        } else {
            rem
        }
    }

    /// Compute `(a * b) mod self.modulus` using Barrett reduction.
    pub fn mul_reduce(&self, a: u64, b: u64) -> u64 {
        let product = a as u128 * b as u128;
        self.reduce(product)
    }
}

// ────────────────────────────────────────────────────────────
// Montgomery Form
// ────────────────────────────────────────────────────────────

/// A value stored in Montgomery form: `x * R mod m` where `R = 2^64`.
///
/// Montgomery form enables efficient modular multiplication:
/// `MontMul(a̅, b̅) = a̅ * b̅ * R^{-1} mod m`
/// which avoids expensive division.
///
/// # Mathematical Specification
/// ```text
/// Montgomery parameters (for 64-bit modulus m, m odd):
///   R    = 2^64
///   R2   = R^2 mod m          (precomputed once)
///   m'   = -m^{-1} mod 2^64   (precomputed once, computed by extended GCD)
///
/// Enter Montgomery form:
///   a̅ = REDC(a * R2)          where REDC is Montgomery reduction
///
/// Exit Montgomery form:
///   a = REDC(a̅)
///
/// REDC(T):
///   u  = (T mod R) * m' mod R
///   t  = (T + u*m) / R
///   if t >= m: return t - m
///   else:      return t
/// ```
///
/// # Learning Resources
/// - [EN] Montgomery modular multiplication (Wikipedia): https://en.wikipedia.org/wiki/Montgomery_modular_multiplication
/// - [EN] Fast Montgomery multiplication (Koç et al.): N/A
/// - [CN] Montgomery 算法（知乎详解）: N/A
/// - [CN] Montgomery 乘法实现（OI-Wiki）: https://oi-wiki.org/math/montgomery/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MontgomeryInt {
    /// Value stored as `x * R mod m`.
    pub val: u64,
}

/// Precomputed parameters for a fixed Montgomery modulus.
#[derive(Clone, Debug)]
pub struct MontgomeryParams {
    /// The modulus `m` (must be odd).
    pub modulus: u64,
    /// `R^2 mod m`, used to convert into Montgomery form.
    pub r2: u64,
    /// `-m^{-1} mod 2^64`, the Montgomery constant.
    pub m_prime: u64,
}

impl MontgomeryParams {
    /// Compute Montgomery parameters for `modulus`.
    ///
    /// # Panics
    /// Panics if `modulus` is even (Montgomery requires odd modulus).
    ///
    /// # Learning Resources
    /// - [EN] Computing m' via extended GCD: https://cp-algorithms.com/algebra/extended-euclid-algorithm.html
    /// - [CN] 扩展欧几里得算法（OI-Wiki）: https://oi-wiki.org/math/ext-gcd/
    pub fn new(modulus: u64) -> Self {
        assert!(modulus & 1 == 1, "Montgomery modulus must be odd");
        todo!("compute r2 = R^2 mod m and m_prime = -m^{{-1}} mod 2^64 using iterative method")
    }

    /// Montgomery reduction REDC: given `T < m * R`, return `T * R^{-1} mod m`.
    ///
    /// # Learning Resources
    /// - [EN] REDC algorithm: https://en.wikipedia.org/wiki/Montgomery_modular_multiplication#The_REDC_algorithm
    /// - [CN] REDC 算法步骤解析: N/A
    pub fn redc(&self, t: u128) -> u64 {
        todo!("REDC: u = (T mod R) * m_prime mod R; t = (T + u*m) >> 64; conditional subtract")
    }

    /// Convert `a` (in normal form) to Montgomery form `a * R mod m`.
    pub fn to_montgomery(&self, a: u64) -> MontgomeryInt {
        todo!("return REDC(a * r2)")
    }

    /// Convert `a` from Montgomery form back to normal form.
    pub fn from_montgomery(&self, a: MontgomeryInt) -> u64 {
        todo!("return REDC(a.val as u128), which computes a.val * R^{{-1}} mod m")
    }

    /// Multiply two Montgomery-form integers: `REDC(a.val * b.val)`.
    pub fn mont_mul(&self, a: MontgomeryInt, b: MontgomeryInt) -> MontgomeryInt {
        todo!("MontgomeryInt {{ val: self.redc(a.val as u128 * b.val as u128) }}")
    }
}

// ────────────────────────────────────────────────────────────
// Primitive root finding
// ────────────────────────────────────────────────────────────

/// Find a primitive root (generator) of the multiplicative group Z_p^*.
///
/// A primitive root `g` of prime `p` satisfies: the order of `g` is `p-1`,
/// i.e., `g^k ≠ 1 (mod p)` for all `0 < k < p-1`.
///
/// # Mathematical Specification
/// ```text
/// To verify g is a primitive root of prime p:
/// - Factorize p-1 = q1^e1 * q2^e2 * ... * qk^ek
/// - For each prime factor qi: check g^((p-1)/qi) ≠ 1 (mod p)
/// ```
///
/// # Learning Resources
/// - [EN] Primitive root (CP-Algorithms): https://cp-algorithms.com/algebra/primitive-root.html
/// - [CN] 原根（OI-Wiki）: https://oi-wiki.org/math/primitive-root/
pub fn find_primitive_root(p: u64) -> Result<u64, MathError> {
    // Check p is prime (basic check)
    if p < 2 {
        return Err(MathError::NotPrime(p));
    }
    // Trial: find smallest g that is a primitive root
    // For NTT-friendly primes, g is typically small (3, 5, or 7)
    'outer: for g in 2..p {
        // Check g^((p-1)/q) != 1 for all prime factors q of p-1
        let pm1 = p - 1;
        let factors = prime_factors(pm1);
        for q in &factors {
            if mod_pow(g, pm1 / q, p) == 1 {
                continue 'outer;
            }
        }
        return Ok(g);
    }
    Err(MathError::NotPrime(p))
}

/// Return the distinct prime factors of `n`.
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_add() {
        assert_eq!(mod_add(3, 4, 7), 0);
        assert_eq!(mod_add(5, 6, 11), 0);
    }

    #[test]
    fn test_mod_mul() {
        assert_eq!(mod_mul(3, 4, 7), 5);
        assert_eq!(mod_mul(100, 200, 97), mod_pow(100, 1, 97) * 200 % 97);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 0, 7), 1);
        assert_eq!(mod_pow(0, 5, 7), 0);
    }

    #[test]
    fn test_mod_inv() {
        let p = 998_244_353u64;
        let a = 123456789u64;
        let inv = mod_inv(a, p);
        assert_eq!(mod_mul(a, inv, p), 1);
    }

    #[test]
    #[ignore = "implement MontgomeryParams::new first"]
    fn test_montgomery_roundtrip() {
        let p = 998_244_353u64;
        let params = MontgomeryParams::new(p);
        let a = 123456789u64;
        let mont_a = params.to_montgomery(a);
        assert_eq!(params.from_montgomery(mont_a), a);
    }

    #[test]
    fn test_barrett_reduce() {
        let m = 998_244_353u64;
        let reducer = BarrettReducer::new(m);
        for x in [0u64, 1, m - 1, m, m + 1, m * m - 1] {
            let expected = (x as u128 % m as u128) as u64;
            assert_eq!(reducer.reduce(x as u128), expected, "x = {x}");
        }
    }

    #[test]
    fn test_primitive_root() {
        let p = 998_244_353u64; // = 119 * 2^23 + 1, primitive root = 3
        let g = find_primitive_root(p).unwrap();
        // Verify: g^(p-1) = 1 and g^((p-1)/2) != 1
        assert_eq!(mod_pow(g, p - 1, p), 1);
        assert_ne!(mod_pow(g, (p - 1) / 2, p), 1);
    }
}
