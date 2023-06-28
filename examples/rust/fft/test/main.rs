// Generate json testing input / output for make test
use rustfft::{FftPlanner};
use num_complex::{Complex64};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Complex64Def {
    pub re: f64,
    pub im: f64,
}

impl From<Complex64> for Complex64Def {
    fn from(def: Complex64) -> Complex64Def {
        Complex64Def { re : def.re, im : def.im }
    }
}

fn from_num_complex(list_num_complex: Vec<Complex64>) -> Vec<Complex64Def> {
    list_num_complex.iter().map(|x| Complex64Def {
        re: x.re,
        im: x.im
    }).collect::<Vec<Complex64Def>>()
}

fn print_json_debug<'a, T: Serialize + Deserialize<'a>>(type_with_serialize: T) {
    let json_buffer = serde_json::to_string(&type_with_serialize).unwrap();
    println!("{}", json_buffer);
}

fn test_forward(l: u8, mut buffer: Vec<Complex64>) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(usize::from(l));
    fft.process(&mut buffer);
    print_json_debug(from_num_complex(buffer));
}

fn test_inverse(l: u8, mut buffer: Vec<Complex64>) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(usize::from(l));
    fft.process(&mut buffer);
    print_json_debug(from_num_complex(buffer));
}

fn main() {
    //test_forward(1, vec![Complex64{ re: 1.0, im: 2.0 }; 1]);
    //test_inverse(1, vec![Complex64{ re: 1.0, im: 2.0 }; 1]);
    let vtor_input = vec![Complex64{ re: 1.0, im: 2.0 }, Complex64{ re: 1.5, im: 2.5 }];
    let json_input = serde_json::to_string(&from_num_complex(vtor_input.clone())).unwrap();
    println!("{}", json_input);
    test_forward(1, vtor_input.clone());
    test_forward(2, vtor_input.clone());
    test_inverse(2, vtor_input.clone());
}

