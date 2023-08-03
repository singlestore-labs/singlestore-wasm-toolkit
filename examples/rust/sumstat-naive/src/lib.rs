wit_bindgen_rust::export!("extension.wit");

struct Extension;

use crate::extension::SumstatRes;
use std::convert::TryInto;

type Handle = i32;

struct SumstatNaiveState {
    sum: f64,
    count: i32,
    sum_of_squares: f64,
    data: Vec<f64>,
}

// Helper function: extract a value representing the `pct` percentile of a sorted sample-set, using
// linear interpolation. If samples are not sorted, return nonsensical value.
// Taken from: https://doc.rust-lang.org/src/test/stats.rs.html#234-238
fn percentile_of_sorted(sorted_samples: &[f64], pct: f64) -> f64 {
    assert!(!sorted_samples.is_empty());
    if sorted_samples.len() == 1 {
        return sorted_samples[0];
    }
    let zero: f64 = 0.0;
    assert!(zero <= pct);
    let hundred = 100_f64;
    assert!(pct <= hundred);
    if pct == hundred {
        return sorted_samples[sorted_samples.len() - 1];
    }
    let length = (sorted_samples.len() - 1) as f64;
    let rank = (pct / hundred) * length;
    let lrank = rank.floor();
    let d = rank - lrank;
    let n = lrank as usize;
    let lo = sorted_samples[n];
    let hi = sorted_samples[n + 1];
    lo + (hi - lo) * d
}

#[inline]
fn sumstat_state_from(handle: Handle) -> &'static mut SumstatNaiveState {
    let ptr = handle as *mut SumstatNaiveState;
    unsafe { &mut *ptr }
}

#[inline]
fn with_sumstat_state(handle: Handle, f: impl FnOnce(&mut SumstatNaiveState)) -> Handle {
    let sumstat_state = sumstat_state_from(handle);
    f(sumstat_state);
    handle as *mut SumstatNaiveState as Handle
}

#[inline]
fn drop_sumstat_state(handle: Handle) {
    drop(unsafe { std::ptr::read(handle as *mut SumstatNaiveState) });
}

// unbiased estimator of population variance
fn unbiased_estimator_pop_var(sum: f64, count: i32, sum_of_squares: f64) -> f64 {
    let avg = sum / count as f64;

    (sum_of_squares - 2.0 * avg * sum + avg * avg * count as f64) / (count as f64 - 1.0)
}

impl crate::extension::Extension for Extension {
    // Initializes an empty handle
    fn sumstat_init_handle() -> Handle {
        let boxed = Box::new(SumstatNaiveState {
            sum: 0.0,
            count: 0, // technically this is not needed if we keep the the vector data embedded
            sum_of_squares: 0.0,
            data: vec![],
        });
        Box::leak(boxed) as *const SumstatNaiveState as i32
    }

    // Add a value to a handle
    fn sumstat_update_handle(handle: Handle, value: f64) -> Handle {
        with_sumstat_state(handle, |sumstat_state| {
            sumstat_state.sum += value;
            sumstat_state.count += 1;
            sumstat_state.sum_of_squares += value * value;
            sumstat_state.data.push(value);
        })
    }

    fn sumstat_merge_handle(left: Handle, right: Handle) -> i32 {
        with_sumstat_state(left, |left_state| {
            with_sumstat_state(right, |right_state| {
                left_state.sum += right_state.sum;
                left_state.count += right_state.count;
                left_state.sum_of_squares += right_state.sum_of_squares;
                left_state.data.extend_from_slice(&right_state.data);
            });
            drop_sumstat_state(right);
        })
    }

    // Compute summary statistics for a handle
    fn sumstat_summary(handle: Handle) -> SumstatRes {
        let state = sumstat_state_from(handle);
        let mut sorted_data = state.data.clone();

        // TODO: this is slow, have to sort the data everytime we query
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // get min, max, p99, p95, p50, p1, p5
        // crash if there are no values
        let cur_min = percentile_of_sorted(&sorted_data, 0.0);
        let cur_max = percentile_of_sorted(&sorted_data, 100.0);
        let cur_p99 = percentile_of_sorted(&sorted_data, 99.0);
        let cur_p95 = percentile_of_sorted(&sorted_data, 95.0);
        let cur_p50 = percentile_of_sorted(&sorted_data, 50.0);
        let cur_p5 = percentile_of_sorted(&sorted_data, 5.0);
        let cur_p1 = percentile_of_sorted(&sorted_data, 1.0);

        SumstatRes {
            avg: state.sum / state.count as f64,
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
    fn sumstat_serialize_handle(handle: Handle) -> extension::Blob {
        let state = sumstat_state_from(handle);
        let blob = f64::to_be_bytes(state.sum)
            .iter()
            .copied()
            .chain(i32::to_be_bytes(state.count))
            .chain(f64::to_be_bytes(state.sum_of_squares))
            .chain(state.data.clone().iter().flat_map(|val| val.to_be_bytes()))
            .collect();
        drop_sumstat_state(handle);
        blob
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

    // one-sample t-test
    fn sumstat_t_test_one(handle: Handle, mu: f64) -> f64 {
        let sumstat_state = sumstat_state_from(handle);
        let avg = sumstat_state.sum / sumstat_state.count as f64;
        let std = unbiased_estimator_pop_var(
            sumstat_state.sum,
            sumstat_state.count,
            sumstat_state.sum_of_squares,
        )
        .sqrt();
        let std_err = std / (sumstat_state.count as f64).sqrt();
        (avg - mu) / (std_err)
    }

    // independent two-sample t-test, distributions are assumed to have similar variances
    fn sumstat_t_test_indep(left: Handle, right: Handle) -> f64 {
        let left_state = sumstat_state_from(left);
        let right_state = sumstat_state_from(right);

        // computes average and unbiased estimator of population variance for two samples
        let left_avg = left_state.sum / left_state.count as f64;
        let right_avg = right_state.sum / right_state.count as f64;

        let left_estimator =
            unbiased_estimator_pop_var(left_state.sum, left_state.count, left_state.sum_of_squares);
        let right_estimator = unbiased_estimator_pop_var(
            right_state.sum,
            right_state.count,
            right_state.sum_of_squares,
        );
        let pooled_std = (((left_state.count as f64 - 1.0 as f64) * left_estimator
            + (right_state.count as f64 - 1.0 as f64) * right_estimator)
            / (left_state.count as f64 + right_state.count as f64 - 2.0))
            .sqrt();
        (left_avg - right_avg)
            / (pooled_std * (1.0 / left_state.count as f64 + 1.0 / right_state.count as f64).sqrt())
    }

    // indepent two-sample t-test, distributions are assumed to have unequal variances
    fn sumstat_t_test_indepu(left: Handle, right: Handle) -> f64 {
        let left_state = sumstat_state_from(left);
        let right_state = sumstat_state_from(right);
        let left_avg = left_state.sum / left_state.count as f64;
        let right_avg = right_state.sum / right_state.count as f64;

        let left_estimator =
            unbiased_estimator_pop_var(left_state.sum, left_state.count, left_state.sum_of_squares);
        let right_estimator = unbiased_estimator_pop_var(
            right_state.sum,
            right_state.count,
            right_state.sum_of_squares,
        );
        let pooled_std = (left_estimator / left_state.count as f64
            + right_estimator / right_state.count as f64)
            .sqrt();
        (left_avg - right_avg) / pooled_std
    }

    // dependent t-test for paired samples
    fn sumstat_t_test_paired(left: Handle, right: Handle, mu: f64) -> f64 {
        let left_state = sumstat_state_from(left);
        let right_state = sumstat_state_from(right);

        // check if size of both samples are equal for cross t-test
        assert!(left_state.count == right_state.count);

        let n = left_state.count as f64;
        let left_avg = left_state.sum / left_state.count as f64;
        let right_avg = right_state.sum / right_state.count as f64;

        // dif_avg = (sum_i (x_i - y_i)) / n = left_avg - right_avg
        let dif_avg = left_avg - right_avg;

        // sum_squares_of_dif   = sum_i (dif_i - dif_avg)^2
        //                      = sum_i (left_i - right_i - dif_avg)^2
        let sum_squares_of_dif: f64 = left_state
            .data
            .iter()
            .zip(right_state.data.iter())
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
    fn sumstat_chi_squared_test(handles: Vec<Handle>) -> f64 {
        let num_data = handles.len();
        let mut boxes: Vec<&mut SumstatNaiveState> = Vec::new();

        for handle in handles {
            let state = sumstat_state_from(handle);
            boxes.push(state);
        }

        // obs[j] = obs_of_class_j
        let num_classes = boxes[0].data.len();

        let mut obs: Vec<f64> = vec![0.0; num_classes];
        let mut total_obs: f64 = 0.0;

        for i in 0..num_data {
            //assert!(boxes[i].count as usize == num_classes);
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

    fn sumstat_batch_update(handle: i32, values: Vec<f64>) -> i32 {
        let mut res: i32 = handle;
        for value in values {
            res = <Extension as extension::Extension>::sumstat_update_handle(handle, value);
        }
        res
    }

    #[test]
    fn basic() {
        let state_a = <Extension as extension::Extension>::sumstat_init_handle();
        let state_a_1 = <Extension as extension::Extension>::sumstat_update_handle(state_a, 1.2);
        let summary = <Extension as extension::Extension>::sumstat_summary(state_a_1);
        assert_eq!(summary.avg, 1.2);
        assert_eq!(summary.min, 1.2);
        assert_eq!(summary.max, 1.2);
        assert_eq!(summary.p1, 1.2);
        assert_eq!(summary.p5, 1.2);
        assert_eq!(summary.p50, 1.2);
        assert_eq!(summary.p95, 1.2);
        assert_eq!(summary.p99, 1.2);
    }

    #[test]
    fn summary() {
        // random vector of 100 elements
        let random_data = vec![
            994.9452641411522,
            766.6186545351587,
            813.213664012328,
            433.2269457293671,
            560.0710480764311,
            609.3682904802079,
            857.0164925587948,
            268.74619164171384,
            591.6869856863519,
            619.0677837426701,
            996.2878585806029,
            709.2373291487877,
            807.4635140778136,
            789.2140021575082,
            371.5583055741912,
            737.8013033404671,
            577.8552617709787,
            368.5584670229196,
            519.1315001105025,
            963.3167361277914,
            761.3000233523791,
            526.1789374160034,
            826.7224734940145,
            309.5286328611194,
            175.871143120405,
            893.9109954936678,
            902.5467756574709,
            852.8196759403289,
            410.8834441280925,
            840.5876317097997,
            514.0836609785409,
            988.215320164302,
            900.5869102888327,
            786.6487496337388,
            98.4497203664137,
            337.9101453828799,
            196.02393678937327,
            343.3852147316866,
            228.61084717597535,
            496.01128033405905,
            484.46024471169073,
            577.3978517237514,
            193.62613503638968,
            976.9513601576443,
            557.0319628490743,
            515.7793005837365,
            182.94121452633078,
            195.32933900749268,
            855.7333485998589,
            761.1573695545053,
            817.0380407440597,
            96.45572597170778,
            115.1020952013544,
            791.6332202405757,
            818.7063481959054,
            190.98256027770864,
            681.1445388317891,
            102.58982790947441,
            401.5595342667784,
            300.16732214068634,
            973.6171925572582,
            39.89862681030719,
            897.0210842225349,
            535.7462476372015,
            89.94969933489372,
            984.9807700619301,
            46.73096765064694,
            425.4277433684052,
            94.02898321842434,
            732.3299200831053,
            110.60029392664482,
            224.35826440001716,
            732.218797981821,
            347.8574619154717,
            906.1859796915038,
            332.84767608044064,
            713.8500822679961,
            41.840663005699035,
            464.3317063269512,
            5.594170705148166,
            333.6988050001494,
            295.51478868765884,
            731.6580746898314,
            452.1575940061012,
            690.08726334307,
            51.527004416932435,
            536.3411543196304,
            212.81676987684253,
            94.47135478418436,
            722.8612959362675,
            490.06651787599446,
            169.07750238092618,
            697.0922056160813,
            421.50454597818646,
            192.5168474349297,
            874.1899101237918,
            402.2002319605863,
            282.6400404831922,
            732.6929750286873,
            932.5793703712158,
        ];
        let state = <Extension as extension::Extension>::sumstat_init_handle();
        let state_with_data = sumstat_batch_update(state, random_data);

        let summary = <Extension as extension::Extension>::sumstat_summary(state_with_data);
        assert_eq!((summary.avg - 523.7555904162601).abs() < 1e-9, true);
        assert_eq!(summary.min, 5.594170705148166);
        assert_eq!(summary.max, 996.2878585806029);
        assert_eq!(summary.p1, 39.5555822492556);
        assert_eq!(summary.p5, 88.02856458899566);
        assert_eq!(summary.p50, 522.655218763253);
        assert_eq!(summary.p95, 973.7839009372775);
        assert_eq!(summary.p99, 994.9586900855467);
    }

    #[test]
    fn chisq_test() {
        let state_a = <Extension as extension::Extension>::sumstat_init_handle();
        let state_b = <Extension as extension::Extension>::sumstat_init_handle();
        let state_c = <Extension as extension::Extension>::sumstat_init_handle();
        let state_d = <Extension as extension::Extension>::sumstat_init_handle();

        let state_a_with_data = sumstat_batch_update(state_a, vec![90.0, 30.0, 30.0]);
        let state_b_with_data = sumstat_batch_update(state_b, vec![60.0, 50.0, 40.0]);
        let state_c_with_data = sumstat_batch_update(state_c, vec![104.0, 51.0, 45.0]);
        let state_d_with_data = sumstat_batch_update(state_d, vec![95.0, 20.0, 35.0]);

        let states = vec![
            state_a_with_data,
            state_b_with_data,
            state_c_with_data,
            state_d_with_data,
        ];

        let chisq_obs = <Extension as extension::Extension>::sumstat_chi_squared_test(states);
        println!("chisq_obs = {}", chisq_obs);
        assert_eq!((chisq_obs - 24.57).abs() < 1e-2, true);
    }

    #[test]
    fn t_test_one() {
        let data = vec![
            20.70, 27.46, 22.15, 19.85, 21.29, 24.75, 20.75, 22.91, 25.34, 20.33, 21.54, 21.08,
            22.14, 19.56, 21.10, 18.04, 24.12, 19.95, 19.72, 18.28, 16.26, 17.46, 20.53, 22.12,
            25.06, 22.44, 19.08, 19.88, 21.39, 22.33, 25.79,
        ];
        let state = <Extension as extension::Extension>::sumstat_init_handle();
        let state_with_data = sumstat_batch_update(state, data);
        let t_obs = <Extension as extension::Extension>::sumstat_t_test_one(state_with_data, 20.0);
        println!("t_one_obs = {}", t_obs);
        assert_eq!((t_obs - 3.07).abs() < 1e-2, true);
    }

    #[test]
    fn t_test_indep() {
        let data_1 = vec![30.02, 29.99, 30.11, 29.97, 30.01, 29.99];
        let data_2 = vec![29.89, 29.93, 29.72, 29.98, 30.02, 29.98];
        let state_1 = <Extension as extension::Extension>::sumstat_init_handle();
        let state_2 = <Extension as extension::Extension>::sumstat_init_handle();
        let state_1_with_data = sumstat_batch_update(state_1, data_1);
        let state_2_with_data = sumstat_batch_update(state_2, data_2);

        let t_obs_indep = <Extension as extension::Extension>::sumstat_t_test_indep(
            state_1_with_data,
            state_2_with_data,
        );
        println!("t_obs_indep = {}", t_obs_indep);
        assert_eq!((t_obs_indep - 1.959).abs() < 1e-3, true);
    }

    #[test]
    fn t_test_indepu() {
        let data_1 = vec![30.02, 29.99, 30.11, 29.97, 30.01, 29.99];
        let data_2 = vec![29.89, 29.93, 29.72, 29.98, 30.02, 29.98];
        let state_1 = <Extension as extension::Extension>::sumstat_init_handle();
        let state_2 = <Extension as extension::Extension>::sumstat_init_handle();
        let state_1_with_data = sumstat_batch_update(state_1, data_1);
        let state_2_with_data = sumstat_batch_update(state_2, data_2);

        let t_obs_indepu = <Extension as extension::Extension>::sumstat_t_test_indepu(
            state_1_with_data,
            state_2_with_data,
        );
        println!("t_obs_indepu = {}", t_obs_indepu);
        assert_eq!((t_obs_indepu - 1.959).abs() < 1e-3, true);
    }
    #[test]
    fn t_test_paired() {
        let pre_data: Vec<f64> = vec![
            18.0, 21.0, 16.0, 22.0, 19.0, 24.0, 17.0, 21.0, 23.0, 18.0, 14.0, 16.0, 16.0, 19.0,
            18.0, 20.0, 12.0, 22.0, 15.0, 17.0,
        ];
        let post_data: Vec<f64> = vec![
            22.0, 25.0, 17.0, 24.0, 16.0, 29.0, 20.0, 23.0, 19.0, 20.0, 15.0, 15.0, 18.0, 26.0,
            18.0, 24.0, 18.0, 25.0, 19.0, 16.0,
        ];
        let state_pre = <Extension as extension::Extension>::sumstat_init_handle();
        let state_post = <Extension as extension::Extension>::sumstat_init_handle();
        let state_pre_with_data = sumstat_batch_update(state_pre, pre_data);
        let state_post_with_data = sumstat_batch_update(state_post, post_data);
        let t_obs_paired = <Extension as extension::Extension>::sumstat_t_test_paired(
            state_pre_with_data,
            state_post_with_data,
            0.0,
        );
        println!("t_obs_paired = {}", t_obs_paired);
        assert_eq!((t_obs_paired - (-3.231)).abs() < 1e-3, true);
    }

    #[test]
    fn t_test_paired_2() {
        let pre_data: Vec<f64> = vec![30.0, 31.0, 34.0, 40.0, 36.0, 35.0, 34.0, 30.0, 28.0, 29.0];

        let post_data: Vec<f64> = vec![30.0, 31.0, 32.0, 38.0, 32.0, 31.0, 32.0, 29.0, 28.0, 30.0];
        let state_pre = <Extension as extension::Extension>::sumstat_init_handle();
        let state_post = <Extension as extension::Extension>::sumstat_init_handle();
        let state_pre_with_data = sumstat_batch_update(state_pre, pre_data);
        let state_post_with_data = sumstat_batch_update(state_post, post_data);
        let t_obs_paired = <Extension as extension::Extension>::sumstat_t_test_paired(
            state_pre_with_data,
            state_post_with_data,
            0.0,
        );
        println!("t_obs_paired_2 = {}", t_obs_paired);
        assert_eq!((t_obs_paired - 2.584921310565987).abs() < 1e-9, true);
    }
}
