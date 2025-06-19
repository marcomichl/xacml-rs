use super::*;

/// 5.55 StatusCodeType
/// Contains the status code of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct StatusCodeType {
    #[serde(rename = "@Value")]
    value: String,          // see Annex B.8 for values / implementation as enum 
    #[serde(rename = "StatusCode", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_code: Option<Vec<StatusCodeType>>     //Minor codes
}