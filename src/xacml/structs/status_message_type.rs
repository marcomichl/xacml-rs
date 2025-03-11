use super::*;


/// 5.56 StatusMessageType
/// Contains the status message of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct StatusMessageType {
    #[serde(rename = "$value")]
    value: String
}

impl StatusMessageType {
    pub fn new(value: String) -> StatusMessageType {
        StatusMessageType {
            value
        }
    }
}