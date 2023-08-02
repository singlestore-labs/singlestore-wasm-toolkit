wit_bindgen_rust::export!("extension.wit");

struct Extension;

use crate::extension::SumstatNaiveRes;
use std::convert::TryInto;

struct SumstatNaiveState {
    sum: f64,
    count: i32,
    sum_of_squares: f64,
    data: Vec<f64>,
}

// unbiased estimator of population variance
fn unbiased_estimator_pop_var(sum: f64, count: i32, sum_of_squares: f64) -> f64 {
    let avg = sum / count as f64;

    (sum_of_squares - 2.0 * avg * sum + avg * avg * count as f64) / (count as f64 - 1.0)
}

impl crate::extension::Extension for Extension {
    // Initializes an empty handle
    fn sumstat_init_handle() -> extension::State {
        let boxed = Box::new(SumstatNaiveState {
            sum: 0.0,
            count: 0, // technically this is not needed if we keep the the vector data embedded
            sum_of_squares: 0.0,
            data: vec![],
        });
        Box::leak(boxed) as *const SumstatNaiveState as i32
    }

    // Clone a handle
    fn sumstat_clone_handle(state: extension::State) -> i32 {
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

    // Destroy a handle
    fn sumstat_destroy_handle(state: extension::State) -> i32 {
        unsafe {
            let _ = Box::from_raw(state as *mut SumstatNaiveState);
        };
        0
    }

    // Add a value to a handle
    fn sumstat_update_handle(state: extension::State, value: f64) -> i32 {
        let ptr = state as *mut SumstatNaiveState;
        let st = unsafe { &mut *ptr };
        st.sum += value;
        st.count += 1;
        st.sum_of_squares += value * value;
        st.data.push(value);

        state
    }

    fn sumstat_iters_handle(state: extension::State, values: Vec<f64>) -> i32 {
        let ptr = state as *mut SumstatNaiveState;
        let st = unsafe { &mut *ptr };
        for value in values {
            st.sum += value;
            st.count += 1;
            st.sum_of_squares += value * value;
            st.data.push(value);
        }
        state
    }

    // merge two handles
    fn sumstat_merge_handle(left: extension::State, right: extension::State) -> i32 {
        let left_ptr = left as *mut SumstatNaiveState;
        let right_ptr = right as *mut SumstatNaiveState;
        let left_state = unsafe { Box::from_raw(left_ptr) };
        let mut right_state = unsafe { Box::from_raw(right_ptr) };
        right_state.sum += left_state.sum;
        right_state.count += left_state.count;
        right_state.sum_of_squares += left_state.sum_of_squares;
        right_state.data.extend(left_state.data);
        Box::leak(right_state) as *const SumstatNaiveState as i32
    }

    // compute summary statistics for a handle
    fn sumstat_term_handle(state: extension::State) -> SumstatNaiveRes {
        let ptr = state as *mut SumstatNaiveState;
        let mut boxed = unsafe { Box::from_raw(ptr) };
        boxed.data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // get min, max, p99, p95, p50, p1, p5
        // if the array is empty, return f64::INFINITY or
        // if max function return -f64::INFINITY
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

    // serialize a state to bytes
    fn sumstat_serialize_handle(state: extension::State) -> extension::Blob {
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

    // deserialize bytes to a state
    fn sumstat_deserialize_handle(bytes: extension::Blob) -> extension::State {
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

    //  merge two handles and copy into a new state
    fn sumstat_copymerge_handle(left: extension::State, right: extension::State) -> i32 {
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

    // one-sample t-test
    fn sumstat_t_test_one(state: extension::State, mu: f64) -> f64 {
        let ptr = state as *mut SumstatNaiveState;
        let boxed = unsafe { Box::from_raw(ptr) };
        let avg = boxed.sum / boxed.count as f64;
        let std = unbiased_estimator_pop_var(boxed.sum, boxed.count, boxed.sum_of_squares).sqrt();
        let std_err = std / (boxed.count as f64).sqrt();
        (avg - mu) / (std_err)
    }

    // independent two-sample t-test, distributions are assumed to have similar variances
    fn sumstat_t_test_indep(left: extension::State, right: extension::State) -> f64 {
        let left_ptr = left as *mut SumstatNaiveState;
        let left_boxed = unsafe { Box::from_raw(left_ptr) };
        let right_ptr = right as *mut SumstatNaiveState;
        let right_boxed = unsafe { Box::from_raw(right_ptr) };

        // computes average and unbiased estimator of population variance for two samples
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

    // indepent two-sample t-test, distributions are assumed to have unequal variances
    fn sumstat_t_test_indepu(left: extension::State, right: extension::State) -> f64 {
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

    // dependent t-test for paired samples
    fn sumstat_t_test_paired(left: extension::State, right: extension::State, mu: f64) -> f64 {
        let left_boxed = unsafe { Box::from_raw(left as *mut SumstatNaiveState) };
        let right_boxed = unsafe { Box::from_raw(right as *mut SumstatNaiveState) };

        // check if size of both samples are equal for cross t-test
        assert!(left_boxed.count == right_boxed.count);

        let n = left_boxed.count as f64;
        let left_avg = left_boxed.sum / left_boxed.count as f64;
        let right_avg = right_boxed.sum / right_boxed.count as f64;

        // dif_avg = (sum_i (x_i - y_i)) / n = left_avg - right_avg
        let dif_avg = left_avg - right_avg;

        // sum_squares_of_dif   = sum_i (dif_i - dif_avg)^2
        //                      = sum_i (left_i - right_i - dif_avg)^2
        let sum_squares_of_dif: f64 = left_boxed
            .data
            .iter()
            .zip(right_boxed.data.iter())
            .map(|(l, r)| (l - r - dif_avg).powf(2.0))
            .sum();

        // std_dif = sqrt( 1/(n - 1) * sum (dif_i - dif_avg)^2 )
        //         = sqrt( 1/(n - 1) * sum_squares_of_dif)
        let std_dif: f64 = ((1.0 / (n - 1.0 as f64)) * sum_squares_of_dif as f64).sqrt();
        let stderr_dif = std_dif / n.sqrt();

        (dif_avg - mu) / stderr_dif
    }

    // Pearson's chi-squared test
    /*
        Assume:
        > Each row is according to a class
        > data[0], data[1], .., data[n - 1] are state_0.data, state_1.data, ..., state_{n - 1}.data
        > total[0], total[1], ..., total[n - 1] are state_0.sum, state_1.sum,..., state_{n - 1}.sum
        > num_of_classes = data[0].len() == data[1].len() == ... == data[n - 1].len()

        We then have:
        total_obs = sum_i { state[i].sum }
        obs_of_class_j = sum_i { data[i][j] }
        p_ij = prob_obs_of_class_j_in_data_i = obs_of_class_j / total_obs
        m_ij = expected_obs_of_class_j_in_data_i = total[i] * p_i

        chisq_obs = sum_{ij} (data[i][j] - m_ij)^2 / m_{ij}
    */
    fn sumstat_chi_squared_test(states: Vec<extension::State>) -> f64 {
        let num_data = states.len();
        let mut boxes: Vec<Box<SumstatNaiveState>> = Vec::new();

        for state in states {
            let boxed = unsafe { Box::from_raw(state as *mut SumstatNaiveState) };
            boxes.push(boxed);
        }

        // obs[j] = obs_of_class_j
        let num_classes = boxes[0].data.len();

        let mut obs: Vec<f64> = vec![0.0; num_classes];
        let mut total_obs: f64 = 0.0;

        for i in 0..num_data {
            assert!(boxes[i].count as usize == num_classes);
            for j in 0..num_classes {
                obs[j] += boxes[i].data[j];
            }
            total_obs += boxes[i].sum;
        }

        let mut chisq_obs = 0.0;
        for i in 0..num_data {
            for j in 0..num_classes {
                let m_ij = boxes[i].sum * (obs[j] / total_obs);
                chisq_obs += ((boxes[i].data[j] - m_ij).powf(2.0)) / m_ij;
            }
        }
        chisq_obs
    }
}

#[cfg(test)]
mod tests {
    use crate::{extension, Extension};

    #[test]
    fn basic_test() {
        let state_a = <Extension as extension::Extension>::sumstat_init_handle();
        let state_a_1 = <Extension as extension::Extension>::sumstat_update_handle(state_a, 1.0);
        let summary = <Extension as extension::Extension>::sumstat_term_handle(state_a_1);
        assert_eq!(summary.avg, 1.0);
        assert_eq!(summary.min, 1.0);
        assert_eq!(summary.max, 1.0);
    }

    //#[test]
    //fn chisq_test() {
    //let state_a = <Extension as extension::Extension>::sumstat_init_handle();
    //let state_b = <Extension as extension::Extension>::sumstat_init_handle();
    //let state_c = <Extension as extension::Extension>::sumstat_init_handle();
    //let state_d = <Extension as extension::Extension>::sumstat_init_handle();

    //let state_a_with_data = <Extension as extension::Extension>::sumstat_iters_handle(
    //state_a,
    //vec![90.0, 30.0, 30.0],
    //);
    //let state_b_with_data = <Extension as extension::Extension>::sumstat_iters_handle(
    //state_b,
    //vec![60.0, 50.0, 40.0],
    //);
    //let state_c_with_data = <Extension as extension::Extension>::sumstat_iters_handle(
    //state_c,
    //vec![104.0, 51.0, 45.0],
    //);
    //let state_d_with_data = <Extension as extension::Extension>::sumstat_iters_handle(
    //state_d,
    //vec![95.0, 20.0, 35.0],
    //);

    //let states = vec![
    //state_a_with_data,
    //state_b_with_data,
    //state_c_with_data,
    //state_d_with_data,
    //];
    //let chisq_obs = <Extension as extension::Extension>::sumstat_chi_squared_test(states);

    //assert_eq!(chisq_obs, 24.57);
    //}
}
