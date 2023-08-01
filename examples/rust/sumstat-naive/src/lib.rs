wit_bindgen_rust::export!("extension.wit");

type Handle = i32;

struct Extension;

use crate::extension::SumstatNaiveRes;
use std::convert::TryInto;

struct SumstatNaiveState {
    sum: f64,
    count: i32,
    sum_of_squares: f64,
    data: Vec<f64>,
}

fn unbiased_estimator_pop_var(sum: f64, count: i32, sum_of_squares: f64) -> f64 {
    let avg = sum / count as f64;

    (sum_of_squares - 2.0 * avg * sum + avg * avg * count as f64) / (count as f64 - 1.0)
}

impl crate::extension::Extension for Extension {
    // Initializes an empty handle
    fn sumstat_naive_handle_init() -> Handle {
        let boxed = Box::new(SumstatNaiveState {
            sum: 0.0,
            count: 0,
            sum_of_squares: 0.0,
            data: vec![],
        });
        Box::leak(boxed) as *const SumstatNaiveState as i32
    }

    fn sumstat_naive_handle_clone(state: i32) -> i32 {
        let ptr = state as *mut SumstatNaiveState;
        let st = unsafe { Box::from_raw(ptr) };
        let res = Box::new(SumstatNaiveState {
            sum: st.sum,
            count: st.count,
            sum_of_squares: st.sum_of_squares,
            data: st.data.clone(),
        });
        Box::leak(st);
        Box::leak(res) as *const SumstatNaiveState as i32
    }

    fn sumstat_naive_handle_destroy(state: i32) -> i32 {
        unsafe {
            let _ = Box::from_raw(state as *mut SumstatNaiveState);
        };
        0
    }

    fn sumstat_naive_handle_iter(state: i32, value: f64) -> i32 {
        let ptr = state as *mut SumstatNaiveState;
        let st = unsafe { &mut *ptr };
        st.sum += value;
        st.count += 1;
        st.sum_of_squares += value * value;
        st.data.push(value);

        state
    }
    fn sumstat_naive_handle_merge(left: i32, right: i32) -> i32 {
        let left_ptr = left as *mut SumstatNaiveState;
        let right_ptr = right as *mut SumstatNaiveState;
        let left_state = unsafe { Box::from_raw(left_ptr) };
        let mut right_state = unsafe { Box::from_raw(right_ptr) };
        right_state.sum += left_state.sum;
        right_state.count += left_state.count;
        right_state.data.extend(left_state.data);
        right_state.sum_of_squares += left_state.sum_of_squares;
        Box::leak(right_state) as *const SumstatNaiveState as i32
    }

    fn sumstat_naive_handle_term(state: i32) -> SumstatNaiveRes {
        let ptr = state as *mut SumstatNaiveState;
        let mut boxed = unsafe { Box::from_raw(ptr) };
        boxed.data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // get min, max, p99, p95, p50, p1, p5: if the array is empty, return f64::INFINITY or
        // -f64::INFINITY for max value
        let cur_min = match boxed.data.first() {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        let cur_max = match boxed.data.last() {
            None => -f64::INFINITY,
            Some(&x) => x,
        };
        let cur_p99 = match boxed
            .data
            .get(((boxed.count as f32 * 0.99).ceil() - 1.0) as usize)
        {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        let cur_p95 = match boxed
            .data
            .get(((boxed.count as f32 * 0.95).ceil() - 1.0) as usize)
        {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        let cur_p50 = match boxed
            .data
            .get(((boxed.count as f32 * 0.50).ceil() - 1.0) as usize)
        {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        let cur_p1 = match boxed
            .data
            .get(((boxed.count as f32 * 0.01).ceil() - 1.0) as usize)
        {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        let cur_p5 = match boxed
            .data
            .get(((boxed.count as f32 * 0.05).ceil() - 1.0) as usize)
        {
            None => f64::INFINITY,
            Some(&x) => x,
        };
        SumstatNaiveRes {
            avg: boxed.sum / boxed.count as f64,
            min: cur_min,
            max: cur_max,
            p1: cur_p1,
            p5: cur_p5,
            p50: cur_p50,
            p95: cur_p95,
            p99: cur_p99,
        }
    }

    fn sumstat_naive_handle_serialize(state: i32) -> Vec<u8> {
        let ptr = state as *mut SumstatNaiveState;
        let boxed = unsafe { Box::from_raw(ptr) };
        f64::to_be_bytes(boxed.sum)
            .iter()
            .copied()
            .chain(i32::to_be_bytes(boxed.count))
            .chain(f64::to_be_bytes(boxed.sum_of_squares))
            .chain(boxed.data.clone().iter().flat_map(|val| val.to_be_bytes()))
            .collect()
    }

    fn sumstat_naive_handle_deserialize(bytes: Vec<u8>) -> i32 {
        let sum = f64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let count = i32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let sum_of_squares = f64::from_be_bytes(bytes[12..20].try_into().unwrap());

        let mut data: Vec<f64> = vec![];

        let mut i = 20;
        while i < bytes.len() {
            data.push(f64::from_be_bytes(bytes[i..(i + 8)].try_into().unwrap()));
            i += 8;
        }

        let boxed = Box::new(SumstatNaiveState {
            sum,
            count,
            sum_of_squares,
            data,
        });
        Box::leak(boxed) as *const SumstatNaiveState as i32
    }

    fn sumstat_naive_handle_copymerge(left: i32, right: i32) -> i32 {
        let left_ptr = left as *mut SumstatNaiveState;
        let right_ptr = right as *mut SumstatNaiveState;
        let left_state = unsafe { Box::from_raw(left_ptr) };
        let right_state = unsafe { Box::from_raw(right_ptr) };

        let mut new_data = left_state.data.clone();
        new_data.extend(right_state.data.clone());

        let new_state = Box::new(SumstatNaiveState {
            sum: left_state.sum + right_state.sum,
            count: left_state.count + right_state.count,
            sum_of_squares: left_state.sum_of_squares + right_state.sum_of_squares,
            data: new_data,
        });
        Box::leak(left_state);
        Box::leak(right_state);
        Box::leak(new_state) as *const SumstatNaiveState as i32
    }

    fn sumstat_naive_t_test_one(state: Handle, mu: f64) -> f64 {
        let ptr = state as *mut SumstatNaiveState;
        let boxed = unsafe { Box::from_raw(ptr) };
        let avg = boxed.sum / boxed.count as f64;
        let std = unbiased_estimator_pop_var(boxed.sum, boxed.count, boxed.sum_of_squares).sqrt();
        let std_err = std / (boxed.count as f64).sqrt();
        (avg - mu) / (std_err)
    }

    fn sumstat_naive_t_test_indep(left: Handle, right: Handle) -> f64 {
        let left_ptr = left as *mut SumstatNaiveState;
        let left_boxed = unsafe { Box::from_raw(left_ptr) };
        let right_ptr = right as *mut SumstatNaiveState;
        let right_boxed = unsafe { Box::from_raw(right_ptr) };

        let left_avg = left_boxed.sum / left_boxed.count as f64;
        let right_avg = right_boxed.sum / right_boxed.count as f64;

        let left_estimator =
            unbiased_estimator_pop_var(left_boxed.sum, left_boxed.count, left_boxed.sum_of_squares);
        let right_estimator = unbiased_estimator_pop_var(
            right_boxed.sum,
            right_boxed.count,
            right_boxed.sum_of_squares,
        );
        let pooled_std = (((left_boxed.count as f64 - 1.0 as f64) * left_estimator
            + (right_boxed.count as f64 - 1.0 as f64) * right_estimator)
            / (left_boxed.count as f64 + right_boxed.count as f64 - 2.0))
            .sqrt();
        (left_avg - right_avg)
            / (pooled_std * (1.0 / left_boxed.count as f64 + 1.0 / right_boxed.count as f64).sqrt())
    }

    fn sumstat_naive_t_test_indepu(left: Handle, right: Handle) -> f64 {
        let left_ptr = left as *mut SumstatNaiveState;
        let left_boxed = unsafe { Box::from_raw(left_ptr) };
        let right_ptr = right as *mut SumstatNaiveState;
        let right_boxed = unsafe { Box::from_raw(right_ptr) };

        let left_avg = left_boxed.sum / left_boxed.count as f64;
        let right_avg = right_boxed.sum / right_boxed.count as f64;

        let left_estimator =
            unbiased_estimator_pop_var(left_boxed.sum, left_boxed.count, left_boxed.sum_of_squares);
        let right_estimator = unbiased_estimator_pop_var(
            right_boxed.sum,
            right_boxed.count,
            right_boxed.sum_of_squares,
        );
        let pooled_std = (left_estimator / left_boxed.count as f64
            + right_estimator / right_boxed.count as f64)
            .sqrt();
        (left_avg - right_avg) / pooled_std
    }

    fn sumstat_naive_t_test_paired(left: Handle, right: Handle, mu: f64) -> f64 {
        let left_ptr = left as *mut SumstatNaiveState;
        let left_boxed = unsafe { Box::from_raw(left_ptr) };
        let right_ptr = right as *mut SumstatNaiveState;
        let right_boxed = unsafe { Box::from_raw(right_ptr) };

        assert!(left_boxed.count == right_boxed.count);

        let n = left_boxed.count as f64;
        let left_avg = left_boxed.sum / left_boxed.count as f64;
        let right_avg = right_boxed.sum / right_boxed.count as f64;

        // dif_avg = (sum_i (x_i - y_i)) / n = left_avg - right_avg
        let dif_avg = left_avg - right_avg;

        // std_dif = sqrt( 1/(n - 1) * sum (dif_i - dif_avg)^2 )
        let sum_squares_of_dif: f64 = left_boxed
            .data
            .iter()
            .zip(right_boxed.data.iter())
            .map(|(l, r)| (l - r - dif_avg) * (l - r - dif_avg))
            .sum();
        let std_dif: f64 = ((1.0 / (n - 1.0 as f64)) * sum_squares_of_dif as f64).sqrt();
        let stderr_dif = std_dif / n.sqrt();

        (dif_avg - mu) / stderr_dif
    }
}
