use serde::{ser::SerializeMap, Serialize, Serializer};

wit_bindgen_rust::export!("split.wit");

use crate::split::Subphrase;
impl Serialize for Subphrase {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("str", &self.str)?;
        map.serialize_entry("idx", &self.idx)?;
        map.end()
    }
}

struct Split;

#[debugger_macro::export_debug_handler]
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
