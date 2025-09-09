#[cfg(target_arch = "wasm32")]
wit_bindgen_rust::export!("simd.wit");

struct Simd;

use core::arch::wasm32::*;

impl simd::Simd for Simd {
    fn u64x2_scalar_mul(a: u64, b: Vec<u64>) -> Vec<u64> {
        let va: v128 = u64x2_splat(a);
        let n = b.len();
        let mut res: Vec<u64> = vec![0; b.len()];
        let mut i = 0;
        while i + 1 < n {
            let vb: v128 = u64x2(b[i], b[i + 1]);
            let s: v128 = u64x2_mul(va, vb);
            res[i] = u64x2_extract_lane::<0>(s);
            res[i + 1] = u64x2_extract_lane::<0>(s);
            i += 2;
        }
        for j in 1..(n % 2 + 1) {
            res[n - j] = a * b[n - j];
        }
        res
    }

    fn u64x2_dot(a: Vec<u64>, b: Vec<u64>) -> u64 {
        assert!(a.len() == b.len());
        let n = a.len();
        let mut sum: v128 = u64x2(0, 0);
        let mut i = 0;
        while i + 1 < n {
            let va: v128 = u64x2(a[i], a[i + 1]);
            let vb: v128 = u64x2(b[i], b[i + 1]);
            sum = u64x2_add(sum, u64x2_mul(va, vb));
            i += 2;
        }
        for j in 1..(n % 2 + 1) {
            return u64x2_extract_lane::<0>(sum)
                + u64x2_extract_lane::<1>(sum)
                + a[n - j] * b[n - j];
        }
        u64x2_extract_lane::<0>(sum) + u64x2_extract_lane::<1>(sum)
    }

    fn u64x2_inner(a: Vec<u64>, b: Vec<u64>) -> Vec<u64> {
        assert!(a.len() == b.len());
        let n = a.len();
        let mut res = vec![0; n];
        let mut i = 0;
        while i + 1 < n {
            let va: v128 = u64x2(a[i], a[i + 1]);
            let vb: v128 = u64x2(b[i], b[i + 1]);
            let m: v128 = u64x2_mul(va, vb);
            res[i] = u64x2_extract_lane::<0>(m);
            res[i + 1] = u64x2_extract_lane::<1>(m);
            i += 2;
        }
        for j in 1..(n % 2 + 1) {
            res[n - 1] = a[n - j] * b[n - j];
        }
        res
    }

    fn u64x2_mat_mul(a: Vec<Vec<u64>>, b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        assert!(a.len() > 0 && b.len() > 0);
        assert!(a[0].len() == b.len());

        let mut res = vec![vec![0; a.len()]; b[0].len()];
        let n = a.len();
        let m = b.len();
        for i in 0..n {
            for j in 0..b[0].len() {
                let mut k = 0;
                while k + 1 < m {
                    let va: v128 = u64x2(a[i][k], a[i][k + 1]);
                    let vb: v128 = u64x2(b[k][j], b[k + 1][j]);
                    let m: v128 = u64x2_mul(va, vb);
                    res[i][j] += u64x2_extract_lane::<0>(m) + u64x2_extract_lane::<1>(m);
                    k += 2;
                }
                for t in 1..(m % 2 + 1) {
                    res[i][j] += a[i][m - t] * b[m - t][j];
                }
            }
        }
        res
    }
}
