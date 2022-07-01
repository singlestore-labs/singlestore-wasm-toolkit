wit_bindgen_rust::export!("split.wit");
use crate::split::Subphrase;
struct Split;

impl split::Split for Split {
    fn split_str(phrase: String, delim: String) -> Vec<Subphrase> {
        phrase
            .split(&delim)
            .scan(0, |idx, s| {
                let current = Subphrase {
                    str: s.to_string(),
                    idx: *idx as i32,
                };
                *idx += (s.len() + delim.len()) as i32;
                Some(current)
            })
            .collect()
    }
}
