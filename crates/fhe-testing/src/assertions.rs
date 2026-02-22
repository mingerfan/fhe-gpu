//! Custom assertion macros for FHE testing.

/// Assert that two complex slot vectors are close within a relative tolerance.
///
/// # Usage
/// ```rust,ignore
/// assert_slots_close!(decoded, expected, rel_tol = 1e-4, abs_tol = 1e-6);
/// ```
#[macro_export]
macro_rules! assert_slots_close {
    ($got:expr, $expected:expr, rel_tol = $rel:expr) => {
        $crate::assert_slots_close!($got, $expected, rel_tol = $rel, abs_tol = 1e-10)
    };
    ($got:expr, $expected:expr, rel_tol = $rel:expr, abs_tol = $abs:expr) => {{
        let got = &$got;
        let expected = &$expected;
        assert_eq!(
            got.len(), expected.len(),
            "slot count mismatch: got {}, expected {}", got.len(), expected.len()
        );
        for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            let re_err = (g.re - e.re).abs();
            let im_err = (g.im - e.im).abs();
            let scale = e.re.abs().max(e.im.abs()).max(1.0);
            let tol = ($rel * scale).max($abs);
            assert!(
                re_err <= tol && im_err <= tol,
                "slot {i}: got ({:.6}, {:.6}i) expected ({:.6}, {:.6}i) \
                 re_err={re_err:.2e} im_err={im_err:.2e} tol={tol:.2e}",
                g.re, g.im, e.re, e.im
            );
        }
    }};
}

/// Assert that two f64 polynomials (as coefficient vectors) are close.
#[macro_export]
macro_rules! assert_poly_close {
    ($got:expr, $expected:expr, tol = $tol:expr) => {{
        let got = &$got;
        let expected = &$expected;
        assert_eq!(got.len(), expected.len(), "polynomial length mismatch");
        for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            let err = (g - e).abs();
            assert!(
                err <= $tol,
                "coeff {i}: got {g:.6} expected {e:.6} err={err:.2e} tol={:.2e}",
                $tol
            );
        }
    }};
}
