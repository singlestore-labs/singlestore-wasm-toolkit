wit_bindgen_rust::export!("jsonpath.wit");
struct Jsonpath;

extern crate jsonpath_lib;
extern crate serde_json;

impl jsonpath::Jsonpath for Jsonpath {
    fn eval_jsonpath(json: String, expr: String) -> String {
        let v = serde_json::from_str(json.as_str()).unwrap();
        let out = jsonpath_lib::select(&v, expr.as_str()).unwrap();
        if out.len() > 1 {
            format!("[{}]", out.into_iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", "))
        } else {
            out.into_iter().map(|s| s.to_string()).collect::<String>()
        }
    }

    fn eval_jsonpaths(json: String, expr: String) -> Vec<String> {
        let v = serde_json::from_str(json.as_str()).unwrap();
        let out = jsonpath_lib::select(&v, expr.as_str()).unwrap();
        out.into_iter().map(|s| s.to_string()).collect()
    }
}
