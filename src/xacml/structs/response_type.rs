use super::*;

/// 5.47 ResponseType
/// Standard return type for a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ResponseType {
    #[serde(rename = "Result")]
    result: Vec<ResultType>
}