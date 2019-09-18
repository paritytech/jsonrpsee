use super::{Error, Id, JsonValue, Version};
use serde::{Deserialize, Serialize};

/// Successful response
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Success {
    /// Protocol version
    pub jsonrpc: Version,
    /// Result
    pub result: JsonValue,
    /// Correlation id
    pub id: Id,
}

/// Unsuccessful response
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
    /// Protocol version
    pub jsonrpc: Version,
    /// Error
    pub error: Error,
    /// Correlation id
    pub id: Id,
}

/// Represents output - failure or success
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Output {
    /// Success
    Success(Success),
    /// Failure
    Failure(Failure),
}

impl Output {
    /// Creates new output given `Result`, `Id` and `Version`.
    pub fn new(result: Result<JsonValue, Error>, id: Id, jsonrpc: Version) -> Self {
        match result {
            Ok(result) => Output::Success(Success {
                id,
                jsonrpc,
                result,
            }),
            Err(error) => Output::Failure(Failure { id, jsonrpc, error }),
        }
    }
}

impl From<Output> for Result<JsonValue, Error> {
    /// Convert into a result. Will be `Ok` if it is a `Success` and `Err` if `Failure`.
    fn from(output: Output) -> Result<JsonValue, Error> {
        match output {
            Output::Success(s) => Ok(s.result),
            Output::Failure(f) => Err(f.error),
        }
    }
}

/// Synchronous response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
    /// Single response
    Single(Output),
    /// Response to batch request (batch of responses)
    Batch(Vec<Output>),
}

impl From<Failure> for Response {
    fn from(failure: Failure) -> Self {
        Response::Single(Output::Failure(failure))
    }
}

impl From<Success> for Response {
    fn from(success: Success) -> Self {
        Response::Single(Output::Success(success))
    }
}

#[test]
fn success_output_serialize() {
    use serde_json;
    use serde_json::Value;

    let so = Output::Success(Success {
        jsonrpc: Version::V2,
        result: Value::from(1),
        id: Id::Num(1),
    });

    let serialized = serde_json::to_string(&so).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","result":1,"id":1}"#);
}

#[test]
fn success_output_deserialize() {
    use serde_json;
    use serde_json::Value;

    let dso = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

    let deserialized: Output = serde_json::from_str(dso).unwrap();
    assert_eq!(
        deserialized,
        Output::Success(Success {
            jsonrpc: Version::V2,
            result: Value::from(1),
            id: Id::Num(1)
        })
    );
}

#[test]
fn failure_output_serialize() {
    use serde_json;

    let fo = Output::Failure(Failure {
        jsonrpc: Version::V2,
        error: Error::parse_error(),
        id: Id::Num(1),
    });

    let serialized = serde_json::to_string(&fo).unwrap();
    assert_eq!(
        serialized,
        r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#
    );
}

#[test]
fn failure_output_deserialize() {
    use serde_json;

    let dfo = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#;

    let deserialized: Output = serde_json::from_str(dfo).unwrap();
    assert_eq!(
        deserialized,
        Output::Failure(Failure {
            jsonrpc: Version::V2,
            error: Error::parse_error(),
            id: Id::Num(1)
        })
    );
}

#[test]
fn single_response_deserialize() {
    use serde_json;
    use serde_json::Value;

    let dsr = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

    let deserialized: Response = serde_json::from_str(dsr).unwrap();
    assert_eq!(
        deserialized,
        Response::Single(Output::Success(Success {
            jsonrpc: Version::V2,
            result: Value::from(1),
            id: Id::Num(1)
        }))
    );
}

#[test]
fn batch_response_deserialize() {
    use serde_json;
    use serde_json::Value;

    let dbr = r#"[{"jsonrpc":"2.0","result":1,"id":1},{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}]"#;

    let deserialized: Response = serde_json::from_str(dbr).unwrap();
    assert_eq!(
        deserialized,
        Response::Batch(vec![
            Output::Success(Success {
                jsonrpc: Version::V2,
                result: Value::from(1),
                id: Id::Num(1)
            }),
            Output::Failure(Failure {
                jsonrpc: Version::V2,
                error: Error::parse_error(),
                id: Id::Num(1)
            })
        ])
    );
}

#[test]
fn handle_incorrect_responses() {
    use serde_json;

    let dsr = r#"
{
	"id": 2,
	"jsonrpc": "2.0",
	"result": "0x62d3776be72cc7fa62cad6fe8ed873d9bc7ca2ee576e400d987419a3f21079d5",
	"error": {
		"message": "VM Exception while processing transaction: revert",
		"code": -32000,
		"data": {}
	}
}"#;

    let deserialized: Result<Response, _> = serde_json::from_str(dsr);
    assert!(
        deserialized.is_err(),
        "Expected error when deserializing invalid payload."
    );
}

#[test]
fn should_parse_empty_response_as_batch() {
    use serde_json;

    let dsr = r#""#;

    let deserialized1: Result<Response, _> = serde_json::from_str(dsr);
    let deserialized2: Result<Response, _> = Response::from_json(dsr);
    assert!(
        deserialized1.is_err(),
        "Empty string is not valid JSON, so we should get an error."
    );
    assert_eq!(deserialized2.unwrap(), Response::Batch(vec![]));
}
