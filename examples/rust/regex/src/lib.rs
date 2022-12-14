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
        COMPILED_RGXS.with(|c| {
            let mut map = c.borrow_mut();
            let re = map
                .entry(pattern)
                .or_insert_with_key(|pattern| Regex::new(pattern).unwrap());
            re.captures(&input)
                .and_then(|c| c.get(1))
                .map(|c| c.as_str())
                .unwrap_or_default()
                .to_string()
        })
    }
}
