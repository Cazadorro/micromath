//! Sine approximation, implemented in terms of `cos(x)`

use super::cos::cos_approx;
use core::f32::consts::PI;

/// Approximates `sin(x)` in radians with a maximum error of `0.06`
pub(super) fn sin_approx(x: f32) -> f32 {
    cos_approx(x - PI / 2.0)
}

#[cfg(test)]
mod tests {
    use super::sin_approx;
    use crate::f32ext::abs::abs;

    /// Maximum error in radians
    const MAX_ERROR: f32 = 0.06;

    /// Sine test vectors - `(input, output)`
    const TEST_VECTORS: &[(f32, f32)] = &[
        (0.000, 0.000),
        (0.140, 0.139),
        (0.279, 0.276),
        (0.419, 0.407),
        (0.559, 0.530),
        (0.698, 0.643),
        (0.838, 0.743),
        (0.977, 0.829),
        (1.117, 0.899),
        (1.257, 0.951),
        (1.396, 0.985),
        (1.536, 0.999),
        (1.676, 0.995),
        (1.815, 0.970),
        (1.955, 0.927),
        (2.094, 0.866),
        (2.234, 0.788),
        (2.374, 0.695),
        (2.513, 0.588),
        (2.653, 0.469),
        (2.793, 0.342),
        (2.932, 0.208),
        (3.072, 0.070),
        (3.211, -0.070),
        (3.351, -0.208),
        (3.491, -0.342),
        (3.630, -0.469),
        (3.770, -0.588),
        (3.910, -0.695),
        (4.049, -0.788),
        (4.189, -0.866),
        (4.328, -0.927),
        (4.468, -0.970),
        (4.608, -0.995),
        (4.747, -0.999),
        (4.887, -0.985),
        (5.027, -0.951),
        (5.166, -0.899),
        (5.306, -0.829),
        (5.445, -0.743),
        (5.585, -0.643),
        (5.725, -0.530),
        (5.864, -0.407),
        (6.004, -0.276),
        (6.144, -0.139),
        (6.283, 0.000),
    ];

    #[test]
    fn sanity_check() {
        for (x, expected) in TEST_VECTORS {
            let sin_x = sin_approx(*x);
            let delta = abs(sin_x - expected);

            assert!(
                delta <= MAX_ERROR,
                "delta {} too large: {} vs {}",
                delta,
                sin_x,
                expected
            );
        }
    }
}