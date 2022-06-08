use debugger_macro::debugger;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str;

struct Power;
wit_bindgen_rust::export!("power.wit");

#[debugger]
impl power::Power for Power {
    fn power_of(base: i32, exp: i32) -> i32 {
        let mut res = 1;
        for _i in 0..exp {
            res *= base;
        }
        res
    }
}
