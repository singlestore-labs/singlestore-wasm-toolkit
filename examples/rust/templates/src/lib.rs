wit_bindgen_rust::export!("templates.wit");
struct Templates;

extern crate serde_json;
extern crate tera;
extern crate skyscraper;
extern crate jsonpath_lib;
extern crate yaml_rust;

use std::collections::HashMap;

use serde_json::Map;
use skyscraper::html;
use skyscraper::xpath;
use tera::{Tera, Context, Function, Result, Value, from_value, to_value};
use yaml_rust::{YamlLoader, yaml};

// Convert an html::DocumentNode to a serde_json::Value recursively.
fn element_to_value(doc: &html::HtmlDocument, elem: &html::DocumentNode) -> Value {
    let html_node = doc.get_html_node(&elem).expect("document does not contain node");
    match html_node {
        html::HtmlNode::Tag(tag) => {
            let out = &mut Map::<String, Value>::new();
            out.insert("name".to_string(), Value::String(tag.name.to_string()));
            out.insert("children".to_string(), elem.children(doc).into_iter().map(|n| element_to_value(doc, &n)).collect());
            let attrs = &mut Map::<String, Value>::new();
            for (key, value) in tag.attributes.iter() {
                attrs.insert(key.to_string(), Value::String(value.to_string()));
            }
            out.insert("attributes".to_string(), Value::Object(attrs.to_owned()));
            // This shouldn't be needed, but the XPath library doesn't support extracting text...
            out.insert("text".to_string(), Value::String(elem.get_all_text(doc).unwrap_or("".to_string())));
            Value::Object(out.to_owned())
        },
        html::HtmlNode::Text(txt) => {
            Value::String(txt.to_string())
        }
    }
}

fn yaml_to_value(doc: &yaml::Yaml) -> Value {
    match doc {
        yaml::Yaml::Real(v) => Value::from(v.to_string().parse::<f64>().unwrap()),
        yaml::Yaml::Integer(v) => Value::from(v.to_string().parse::<i64>().unwrap()),
        yaml::Yaml::String(v) => Value::String(v.to_string()),
        yaml::Yaml::Boolean(v) => Value::Bool(*v),
        yaml::Yaml::Array(v) => {
            Value::Array(v.into_iter().map(|x| yaml_to_value(x)).collect::<Vec<Value>>())
        },
        yaml::Yaml::Hash(v) => {
            let out = &mut Map::<String, Value>::new();
            for (key, val) in v.iter() {
                out.insert(key.as_str().unwrap().to_string(), yaml_to_value(val));
            }
            Value::Object(out.to_owned())
        },
        yaml::Yaml::Alias(_) => Value::Null,
        yaml::Yaml::Null => Value::Null,
        yaml::Yaml::BadValue => Value::Null,
    }
}

// Create an XPath function for use in the template.
fn query_xml(document: html::HtmlDocument) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("path") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(path) => {
                    let xpath = xpath::parse(path.as_str()).expect("xpath is invalid");
                    let nodes = xpath.apply(&document).expect("could not apply xpath to document");
                    if nodes.len() == 0 {
                        Ok(to_value::<Vec<Value>>(vec![]).unwrap())
                    } else if nodes.len() > 1 {
                        Ok(to_value(nodes.into_iter().map(|n| element_to_value(&document, &n)).collect::<Vec<Value>>()).expect("could not convert to value"))
                    } else {
                        Ok(element_to_value(&document, &nodes[0]))
                    }
                },
                Err(_) => Err("could not get path argument".into()),
            },
            None => Err("could not get path argument".into()),
        }
    })
}

// Create a JSONPath function for use in the template.
fn query_json(obj: Value) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("path") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(path) => {
                    Ok(jsonpath_lib::select(&obj, path.as_str()).expect("could not execute JSONPath"))
                },
                Err(_) => Err("could not get path argument".into()),
            },
            None => Err("could not get path argument".into()),
        }
    })
}

// Render the input XML/JSON string using the given template.
fn render(txt: String, infmt: &str, template: String) -> String {
    let mut tera = Tera::default();

    // Using a temporary name here with an extension for the auto-escaping capabilities
    // in the templating engine which get set based on the extension.
    tera.add_raw_template("x", template.as_str()).unwrap();

    // Turn off auto-escaping of special HTML characters.
    tera.autoescape_on(vec![]);

    let mut context = Context::new();

    match infmt {
        "json" => {
            let parsed: Value = serde_json::from_str(txt.as_str()).expect("failed to parse JSON input");
            let obj: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();
            tera.register_function("q", query_json(Value::Object(obj.clone())));
            context.insert("_", &obj);
        },
        "xml" => {
            let obj = html::parse(txt.as_str()).expect("failed to parse XML input");
            let json_obj = element_to_value(&obj, &obj.root_node);
            tera.register_function("q", query_xml(obj));
            context.insert("_", &json_obj);
        },
        "yaml" => {
            let obj = YamlLoader::load_from_str(txt.as_str()).expect("failed to parse yaml input");
            let json_obj = yaml_to_value(&obj[0]).as_object().unwrap().to_owned();
            tera.register_function("q", query_json(Value::Object(json_obj.clone())));
            context.insert("_", &json_obj);
        },
        _ => {
            panic!();
        }
    }

    return tera.render("x", &context).unwrap();
}

impl templates::Templates for Templates {

    fn render_json(json: String, template: String) -> String {
        return render(json, "json", template);
    }

    fn render_xml(xml: String, template: String) -> String {
        return render(xml, "xml", template);
    }

    fn render_yaml(yaml: String, template: String) -> String {
        return render(yaml, "yaml", template);
    }

}
