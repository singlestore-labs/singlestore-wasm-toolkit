wit_bindgen_rust::export!("extension.wit");
use boon::{Compiler, Schemas};
use serde_json::Value;

struct Extension;

const DEFAULT_SCHEMA_URL: &str = "http://tmp/schema.json";

impl extension::Extension for Extension {
    fn match_schema(schema_str: String, instance_str: String) -> bool {
        let schema: Value = serde_json::from_str(schema_str.as_str()).unwrap();
        let instance: Value = serde_json::from_str(instance_str.as_str()).unwrap();

        let mut schemas = Schemas::new();
        let mut compiler = Compiler::new();
        compiler.add_resource(DEFAULT_SCHEMA_URL, schema).unwrap();
        let sch_index = compiler.compile(DEFAULT_SCHEMA_URL, &mut schemas).unwrap();
        let result = schemas.validate(&instance, sch_index);
        if !result.is_err() {
            return true;
        }
        false
    }

    fn is_valid_schema(schema_str: String) -> bool {
        let schema: Value = serde_json::from_str(schema_str.as_str()).unwrap();
        let mut compiler = Compiler::new();
        let is_valid = compiler.add_resource(DEFAULT_SCHEMA_URL, schema);
        if !is_valid.is_err() {
            return true;
        }
        false
    }
}

// sanity tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_naive() {
        let naive_schema = r#"{"type": "object"}"#.to_string();
        let naive_input = r#"{"foo": "bar"}"#.to_string();
        assert_eq!(
            <Extension as extension::Extension>::match_schema(naive_schema, naive_input),
            true
        );
    }

    #[test]
    fn test_from_string_number() {
        let number_schema = r#"{"type": "number"}"#.to_string();
        let number_str_pass = "123".to_string();
        let number_str_fail = r#""meow""#.to_string();

        assert_eq!(
            <Extension as extension::Extension>::match_schema(
                number_schema.clone(),
                number_str_pass
            ),
            true
        );
        assert_eq!(
            <Extension as extension::Extension>::match_schema(number_schema, number_str_fail),
            false
        );
    }

    #[test]
    fn test_from_string_required_field() {
        let required_schema = r#"{
                                    "type": "object",
                                    "properties": {
                                        "foo": {
                                            "type": "string"
                                        }
                                    },
                                    "required": ["foo"],
                                    "additionalProperties": false
                                }"#
        .to_string();
        let required_input_pass = r#"{"foo": "bar"}"#.to_string();
        let required_input_fail = r#"{"hi" : "bar"}"#.to_string();
        let required_input_fail_2 = r#"{"foo": "bar", "hi": "bar"}"#.to_string();
        assert_eq!(
            <Extension as extension::Extension>::match_schema(
                required_schema.clone(),
                required_input_pass
            ),
            true
        );
        assert_eq!(
            <Extension as extension::Extension>::match_schema(
                required_schema.clone(),
                required_input_fail
            ),
            false
        );
        assert_eq!(
            <Extension as extension::Extension>::match_schema(
                required_schema,
                required_input_fail_2
            ),
            false
        );
    }

    #[test]
    fn test_is_valid_schema() {
        let valid_schema = r#"{"type": "integer"}"#.to_string();
        let not_valid_schema = r#"{"type": "int"}"#.to_string();
        assert_eq!(
            <Extension as extension::Extension>::is_valid_schema(valid_schema),
            true
        );
        assert_eq!(
            <Extension as extension::Extension>::is_valid_schema(not_valid_schema),
            false
        );
    }
}
