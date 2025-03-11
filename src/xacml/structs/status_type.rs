use super::*;

/// 5.54 StatusType
/// Contains the status of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct StatusType {
    #[serde(rename = "StatusCode")]
    status_code: StatusCodeType,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_message: Option<StatusMessageType>,
    #[serde(rename = "StatusDetail", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_detail: Option<StatusDetailType>
}