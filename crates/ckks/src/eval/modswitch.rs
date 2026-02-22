//! Modulus switching: reduce level without rescaling (changes noise, not scale).
//!
//! `mod_switch` is different from `rescale`:
//! - `rescale` divides by q_l (reduces scale: Δ² → Δ)
//! - `mod_switch` reduces the modulus without dividing (keeps scale, but adds noise)
//!
//! # Use Cases
//! - Matching levels of two ciphertexts before addition
//! - Budget management in non-multiplicative circuits
//!
//! # Mathematical Specification
//! ```text
//! To switch from level l to level l' < l:
//! For each coefficient j, each limb i ≤ l':
//!   Round: r[j] = round(ct.coeffs[j] * Q_{l'} / Q_l)   (in floating point)
//!   Store: new_ct.limbs[i][j] = r[j] mod q_i
//! ```
//!
//! # Learning Resources
//! - [EN] Modulus switching (BGV): https://eprint.iacr.org/2011/277.pdf §4
//! - [EN] ModSwitch in full RNS: https://eprint.iacr.org/2018/931.pdf §2
//! - [CN] 模数切换与 Rescale 的区别: N/A

use crate::{core::{params::CkksParams, Ciphertext}, CkksError};

/// Switch the ciphertext modulus from `ct.level` down to `target_level`.
///
/// Both ciphertexts in an addition must be at the same level.
/// Use this to bring a higher-level ciphertext down to match a lower-level one.
pub fn mod_switch(
    ct: &Ciphertext,
    target_level: usize,
    params: &CkksParams,
) -> Result<Ciphertext, CkksError> {
    if target_level >= ct.level {
        return Err(CkksError::Eval(format!(
            "mod_switch: target level {target_level} must be less than current level {}", ct.level
        )));
    }
    todo!(
        "for each limb drop, apply ModDown-like operation; \
         note: scale is NOT adjusted (unlike rescale)"
    )
}
