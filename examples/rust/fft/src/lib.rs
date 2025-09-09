use rustfft::{FftPlanner, FftPlannerScalar, num_complex::Complex64};
use fft::{Stcomplex64};

wit_bindgen_rust::export!("fft.wit");

struct Fft;

fn from_num_complex(list_num_complex: Vec<Complex64>) -> Vec<Stcomplex64> {
    list_num_complex.iter().map(|x| Stcomplex64 {
        re: x.re,
        im: x.im
    }).collect::<Vec<Stcomplex64>>()
}

fn from_st_complex(list_st_complex: Vec<Stcomplex64>) -> Vec<Complex64> {
    list_st_complex.iter().map(|x| Complex64::new(x.re, x.im)).collect::<Vec<Complex64>>()
}

impl crate::fft::Fft for Fft {
    fn st_process_forward(l: u8, buf: Vec<Stcomplex64>) -> Vec<Stcomplex64> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(usize::from(l));
        let mut buffer = from_st_complex(buf);
        fft.process(&mut buffer);
        from_num_complex(buffer)
    }
    fn st_process_inverse(l: u8, buf: Vec<Stcomplex64>) -> Vec<Stcomplex64> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(usize::from(l));
        let mut buffer = from_st_complex(buf);
        fft.process(&mut buffer);
        from_num_complex(buffer)
    }
    fn st_scalar_process_forward(l: u8, buf: Vec<Stcomplex64>) -> Vec<Stcomplex64> {
        let mut planner = FftPlannerScalar::new();
        let fft = planner.plan_fft_forward(usize::from(l));
        let mut buffer = from_st_complex(buf);
        fft.process(&mut buffer);
        from_num_complex(buffer)
    }
    fn st_scalar_process_inverse(l: u8, buf: Vec<Stcomplex64>) -> Vec<Stcomplex64> {
        let mut planner = FftPlannerScalar::new();
        let fft = planner.plan_fft_inverse(usize::from(l));
        let mut buffer = from_st_complex(buf);
        fft.process(&mut buffer);
        from_num_complex(buffer)
    }
}
