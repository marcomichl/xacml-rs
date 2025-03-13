use super::*;

/// 5.46 AttributeType
/// Contains a single attribute metadata and value
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeType {
    #[serde(rename = "@AttributeId")]
    pub attribute_id: String,       // Pre-defined URIs in the Annex B, but contain only commonly used; might be implemented as enum
    #[serde(rename = "@IncludeInResult", default = "default_false")]
    pub include_in_result: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub issuer: Option<String>,    
    #[serde(rename = "AttributeValue")]
    pub attribute_value: Vec<AttributeValueType>
}

fn default_false() -> bool {
    false
}   