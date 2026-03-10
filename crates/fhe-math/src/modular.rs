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
/// 注意：这个只能用在素数模上，如果不是素数模是不对的！
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

/// 扩展欧几里得算法
/// # Learning Resources
/// - [EN] Computing m' via extended GCD: https://cp-algorithms.com/algebra/extended-euclid-algorithm.html
/// - [CN] 扩展欧几里得算法（OI-Wiki）: https://oi-wiki.org/math/number-theory/gcd
pub fn extend_gcd(a: u64, b: u64) -> (u64, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (res, x1, y1) = extend_gcd(b, a % b);
    let q = (a / b) as i128;

    (res, y1, x1 - q * y1)
}

/// 实现更加通用的模逆
/// 使用扩展欧几里得算法
pub fn general_mod_inv(a: u64, m: u64) -> Option<u64> {
    if m <= 1 {
        return None;
    }

    let a = a % m;
    let (gcd, x, _) = extend_gcd(a, m);

    if gcd != 1 {
        // 不互素，计算模逆有问题
        None
    } else {
        Some(x.rem_euclid(m as i128) as u64)
    }
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
    /// - [EN] Barrett reduction implementation notes: N/A
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

pub fn newton_lifting(a: u64, log2_m: u64) -> u64 {
    assert!((1..=64).contains(&log2_m));
    assert!(a & 1 == 1);
    let mut x = 1u64; // x0
    let mut bits = 1u64;

    while bits < log2_m {
        x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x))); // 相当于模2^64
        bits <<= 1;
    }

    if log2_m == 64 {
        x
    } else {
        x & ((1u64 << log2_m) - 1)
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
///   m'   = -m^{-1} mod R   (precomputed once, computed by extended GCD)
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
/// - [CN] Montgomery 算法（知乎详解）: https://zhuanlan.zhihu.com/p/581656171
/// - [CN] Montgomery 乘法实现（OI-Wiki）: N/A
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
    pub fn new(modulus: u64) -> Self {
        assert!(modulus & 1 == 1, "Montgomery modulus must be odd");
        // R = 2^64
        let r_mod = mod_pow(2, 64, modulus);
        // 重复平方算法
        let r2 = mod_mul(r_mod, r_mod, modulus);
        let m_prime = newton_lifting(modulus, 64).wrapping_neg(); // 这个只有在模2^64下才可以用wrapping_neg这个计算
        Self {
            modulus,
            r2,
            m_prime,
        }
    }

    /// Montgomery reduction REDC: given `T < m * R`, return `T * R^{-1} mod m`.
    ///
    /// # Learning Resources
    /// - [EN] REDC algorithm: https://en.wikipedia.org/wiki/Montgomery_modular_multiplication#The_REDC_algorithm
    /// - [CN] REDC 算法步骤解析: N/A
    pub fn redc(&self, t: u128) -> u64 {
        assert!((self.modulus as u128 * (1u128 << 64)) > t);
        let tmp = (t as u64).wrapping_mul(self.m_prime);
        let tm = (tmp as u128) * (self.modulus as u128);
        let (_, carry) = (t as u64).overflowing_add(tm as u64);
        let u = (t >> 64) + (tm >> 64) + (carry as u128);
        if u >= self.modulus as u128 {
            (u - self.modulus as u128) as u64
        } else {
            u as u64
        }
    }

    /// Convert `a` (in normal form) to Montgomery form `a * R mod m`.
    pub fn to_montgomery(&self, a: u64) -> MontgomeryInt {
        MontgomeryInt {
            val: self.redc((a as u128) * (self.r2 as u128)),
        }
    }

    /// Convert `a` from Montgomery form back to normal form.
    pub fn from_montgomery(&self, a: MontgomeryInt) -> u64 {
        self.redc(a.val as u128)
    }

    /// Multiply two Montgomery-form integers: `REDC(a.val * b.val)`.
    pub fn mont_mul(&self, a: MontgomeryInt, b: MontgomeryInt) -> MontgomeryInt {
        MontgomeryInt {
            val: self.redc(a.val as u128 * b.val as u128),
        }
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
    fn test_extend_gcd_bezout_identity() {
        let (g, x, y) = extend_gcd(240, 46);
        assert_eq!(g, 2);
        assert_eq!(240i128 * x + 46i128 * y, g as i128);

        let (g, x, y) = extend_gcd(3, 11);
        assert_eq!(g, 1);
        assert_eq!(3i128 * x + 11i128 * y, g as i128);
    }

    #[test]
    fn test_general_mod_inv_exists_and_normalized() {
        assert_eq!(general_mod_inv(3, 11), Some(4));
        assert_eq!(general_mod_inv(10, 17), Some(12));
        assert_eq!(general_mod_inv(14, 3), Some(2));

        let pairs = [(3u64, 11u64), (10, 17), (14, 3), (123456789, 998244353)];
        for (a, m) in pairs {
            let inv = general_mod_inv(a, m).unwrap();
            assert!(inv < m, "inverse should be canonical residue");
            assert_eq!(mod_mul(a % m, inv, m), 1);
        }
    }

    #[test]
    fn test_general_mod_inv_non_coprime_or_invalid_modulus() {
        assert_eq!(general_mod_inv(6, 15), None);
        assert_eq!(general_mod_inv(0, 17), None);
        assert_eq!(general_mod_inv(5, 1), None);
        assert_eq!(general_mod_inv(5, 0), None);
    }

    #[test]
    fn test_general_mod_inv_matches_fermat_for_prime_modulus() {
        let p = 998_244_353u64;
        for a in [1u64, 2, 3, 5, 17, 123456789, p - 1] {
            assert_eq!(general_mod_inv(a, p), Some(mod_inv(a, p)));
        }
    }

    #[test]
    fn test_newton_lifting_matches_general_inverse_for_small_powers_of_two() {
        for log2_m in 1u64..=16 {
            let modulus = 1u64 << log2_m;
            for a in (1u64..modulus).step_by(2) {
                let lifted = newton_lifting(a, log2_m);
                let expected = general_mod_inv(a, modulus).unwrap();
                assert_eq!(lifted, expected, "a = {a}, log2_m = {log2_m}");
            }
        }
    }

    #[test]
    fn test_newton_lifting_returns_inverse_mod_2_pow_64() {
        for a in [1u64, 3, 5, 17, 0xffff_ffff_ffff_fffb, u64::MAX] {
            let lifted = newton_lifting(a, 64);
            assert_eq!(a.wrapping_mul(lifted), 1, "a = {a}");
        }
    }

    #[test]
    fn test_newton_lifting_masks_to_requested_precision() {
        for (a, log2_m) in [(3u64, 3u64), (5, 5), (17, 7), (123, 9), (255, 11)] {
            let lifted = newton_lifting(a, log2_m);
            let modulus = 1u64 << log2_m;
            assert!(lifted < modulus, "a = {a}, log2_m = {log2_m}");
            assert_eq!((a * lifted) % modulus, 1, "a = {a}, log2_m = {log2_m}");
        }
    }

    #[test]
    #[should_panic]
    fn test_newton_lifting_rejects_even_inputs() {
        let _ = newton_lifting(2, 3);
    }

    #[test]
    fn test_montgomery_roundtrip() {
        let p = 998_244_353u64;
        let params = MontgomeryParams::new(p);
        for a in [0u64, 1, 2, 3, 5, 17, 123_456_789, p - 1] {
            let mont_a = params.to_montgomery(a);
            assert_eq!(params.from_montgomery(mont_a), a, "a = {a}");
        }
    }

    #[test]
    fn test_montgomery_mul_matches_mod_mul() {
        let p = 998_244_353u64;
        let params = MontgomeryParams::new(p);
        let cases = [
            (0u64, 0u64),
            (0, 1),
            (1, 1),
            (2, 3),
            (5, 17),
            (123_456_789, 987_654_321 % p),
            (p - 1, p - 1),
            (p - 2, p - 3),
        ];

        for (a, b) in cases {
            let mont_a = params.to_montgomery(a);
            let mont_b = params.to_montgomery(b);
            let product = params.mont_mul(mont_a, mont_b);
            assert_eq!(
                params.from_montgomery(product),
                mod_mul(a, b, p),
                "a = {a}, b = {b}"
            );
        }
    }

    #[test]
    fn test_montgomery_with_large_64bit_modulus() {
        let m = u64::MAX - 58;
        let params = MontgomeryParams::new(m);
        let values = [
            0u64,
            1,
            2,
            3,
            1 << 32,
            1_234_567_890_123_456_789,
            m / 2,
            m - 2,
            m - 1,
        ];

        for a in values {
            let mont_a = params.to_montgomery(a);
            assert_eq!(params.from_montgomery(mont_a), a, "roundtrip a = {a}");
        }

        for (a, b) in [
            (1u64, m - 1),
            (2, m - 2),
            (1 << 32, 1 << 33),
            (1_234_567_890_123_456_789, m - 2),
            (m - 2, m - 1),
        ] {
            let mont_a = params.to_montgomery(a);
            let mont_b = params.to_montgomery(b);
            let product = params.mont_mul(mont_a, mont_b);
            assert_eq!(
                params.from_montgomery(product),
                mod_mul(a, b, m),
                "a = {a}, b = {b}"
            );
        }
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
