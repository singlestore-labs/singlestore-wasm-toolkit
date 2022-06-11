#[cfg(target_arch = "wasm32")]
wit_bindgen_rust::export!("simd.wit");

struct Simd;

use core::arch::wasm32::*;

impl simd::Simd for Simd {
    fn mul(a: u64, b: u64) -> u64 {
        let va: v128 = u64x2_splat(a);
        let vb: v128 = u64x2_splat(b);
        let c = u64x2_extract_lane::<1>(i64x2_mul(va, vb));
        c
    }
    fn dot(a: Vec<u64>, b: Vec<u64>) -> u64 {
        assert!(a.len() == b.len());
        let mut sum: u64 = 0;
        for i in 0..a.len() {
            sum += Self::mul(a[i], b[i]);
        }
        sum
    }
    fn inner(a: Vec<u64>, b: Vec<u64>) -> Vec<u64> {
        assert!(a.len() == b.len());
        let mut res = vec![0; a.len()];
        for i in 0..a.len() {
            res[i] = Self::mul(a[i], b[i]);
        }
        res
    }
    fn mmul(a: Vec<Vec<u64>>, b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        if a.len() == 0 && b.len() == 0 {
            return Vec::with_capacity(0);
        }
        assert!(a[0].len() == b.len());

        let mut res = vec![vec![0; a.len()]; b[0].len()];
        for i in 0..a.len() {
            for j in 0..b[0].len() {
                for k in 0..b.len() {
                    res[i][j] += Self::mul(a[i][k], b[k][j]);
                }
            }
        }
        res
    }
}
