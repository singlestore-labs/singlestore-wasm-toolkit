wit_bindgen_rust::export!("s2regex.wit");
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

struct S2regex;

thread_local! {
    static COMPILED_RGXS: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

impl s2regex::S2regex for S2regex {
    fn capture(input: String, pattern: String) -> String {
        if !COMPILED_RGXS.with(|c| c.borrow().contains_key(&input)) {
            let re = Regex::new(pattern.as_str()).unwrap();
            COMPILED_RGXS.with(|c| c.borrow_mut().insert(pattern.clone(), re));
        }
        COMPILED_RGXS.with(|c| {
            let rgx_map = c.borrow();
            let re = rgx_map.get(pattern.as_str()).unwrap();
            match re.captures(&input) {
                Some(caps) => {
                    if caps.len() > 1 {
                        return caps[1].to_string();
                    }
                    return "".to_string();
                }
                None => "".to_string(),
            }
        })
    }
}

