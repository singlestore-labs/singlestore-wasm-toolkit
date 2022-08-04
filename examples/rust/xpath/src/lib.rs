wit_bindgen_rust::export!("xpath.wit");
struct Xpath;

extern crate sxd_document;
extern crate sxd_xpath;

use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

impl xpath::Xpath for Xpath {
    fn eval_xpath(xml: String, expr: String) -> String {
        let package = parser::parse(xml.as_str()).expect("failed to parse XML");
        let document = package.as_document();
        match evaluate_xpath(&document, expr.as_str()).expect("XPath evaluation failed") {
            sxd_xpath::Value::Nodeset(v) => {
                v.into_iter().map(|s| s.string_value()).collect::<Vec<String>>().join("\n")
            },
            v => v.string()
        }
    }

    fn eval_xpaths(xml: String, expr: String) -> Vec<String> {
        let package = parser::parse(xml.as_str()).expect("failed to parse XML");
        let document = package.as_document();
        match evaluate_xpath(&document, expr.as_str()).expect("XPath evaluation failed") {
            sxd_xpath::Value::Nodeset(v) => {
                v.into_iter().map(|s| s.string_value()).collect()
            },
            v => vec!(v.string())
        }
    }
}
