use serde::{Deserialize, Serialize};
use super::JsonValue;

/// Request parameters
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Params {
    /// No parameters
    None,
    /// Array of values
    Array(Vec<JsonValue>),
    /// Map of values
    Map(serde_json::Map<String, JsonValue>),
}

impl From<Params> for JsonValue {
    fn from(params: Params) -> JsonValue {
        match params {
            Params::Array(vec) => JsonValue::Array(vec),
            Params::Map(map) => JsonValue::Object(map),
            Params::None => JsonValue::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Params;
    use crate::common::{Error, ErrorCode, JsonValue};
    use serde_json;

    #[test]
    fn params_deserialization() {
        let s = r#"[null, true, -1, 4, 2.3, "hello", [0], {"key": "value"}, []]"#;
        let deserialized: Params = serde_json::from_str(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("key".to_string(), JsonValue::String("value".to_string()));

        assert_eq!(
            Params::Array(vec![
                JsonValue::Null,
                JsonValue::Bool(true),
                JsonValue::from(-1),
                JsonValue::from(4),
                JsonValue::from(2.3),
                JsonValue::String("hello".to_string()),
                JsonValue::Array(vec![JsonValue::from(0)]),
                JsonValue::Object(map),
                JsonValue::Array(vec![]),
            ]),
            deserialized
        );
    }

    #[test]
    fn should_return_meaningful_error_when_deserialization_fails() {
        // given
        let s = r#"[1, true]"#;
        let params = || serde_json::from_str::<Params>(s).unwrap();

        // when
        let v1: Result<(Option<u8>, String), Error> = params().parse();
        let v2: Result<(u8, bool, String), Error> = params().parse();
        let err1 = v1.unwrap_err();
        let err2 = v2.unwrap_err();

        // then
        assert_eq!(err1.code, ErrorCode::InvalidParams);
        assert_eq!(
            err1.message,
            "Invalid params: invalid type: boolean `true`, expected a string."
        );
        assert_eq!(err1.data, None);
        assert_eq!(err2.code, ErrorCode::InvalidParams);
        assert_eq!(
            err2.message,
            "Invalid params: invalid length 2, expected a tuple of size 3."
        );
        assert_eq!(err2.data, None);
    }

    #[test]
    fn single_param_parsed_as_tuple() {
        let params: (u64,) = Params::Array(vec![JsonValue::from(1)]).parse().unwrap();
        assert_eq!(params, (1,));
    }
}
