struct Power;
wit_bindgen_rust::export!("power.wit");
impl power::Power for Power {
    fn power_of(base: i32, exp: i32) -> i32 {
        let mut res = 1;
        for _i in 0..exp {
            res *= base;
        }
        res
    }
}

struct Debugger;
wit_bindgen_rust::export!("../../../debugger/debugger.wit");
impl debugger::Debugger for Debugger {
    fn handle_json(name: String, _json: Vec<u8>) -> Vec<u8> {
        let foo = 123;
        format!("{} {}", name, foo).into_bytes()
    }
}
