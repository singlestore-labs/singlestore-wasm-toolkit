wit_bindgen_rust::export!("power.wit");

struct Power;

#[debugger_macro::export_debug_handler]
impl power::Power for Power {
    fn power_of(base: i32, exp: i32) -> i32 {
        let mut res = 1;
        for _i in 0..exp {
            res *= base;
        }
        res
    }
}
