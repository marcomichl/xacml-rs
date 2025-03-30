use super::*;


///5.42 RequestType
/// Contains the request for a decision
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RequestType {
    #[serde(rename = "@ReturnPolicyIdList")]
    pub(super) return_policy_id_list: bool,
    #[serde(rename = "@CombinedDecision")]
    pub(super) combined_decision: bool,
    #[serde(rename = "RequestDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(super) request_defaults: Option<RequestDefaultsType>,
    #[serde(rename = "Attributes")]
    pub(super) attributes: Vec<AttributesType>,
    #[serde(rename = "MultiRequests", skip_serializing_if = "UnimplementedField::is_none")]
    #[builder(default = "UnimplementedField(None)")]
    pub(super) multi_requests: UnimplementedField // Is not yet implemented and optional, will fail if present
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct UnimplementedField(pub Option<String>);
impl<'de> Deserialize<'de> for UnimplementedField {
    fn deserialize<D>(deserializer: D) -> Result<UnimplementedField, D::Error>
    where
        D: Deserializer<'de>,
    {
        let x: Option<String> = Deserialize::deserialize(deserializer)?;
        if x.is_some() {
            Err(serde::de::Error::custom(format!("Field must not be present: {}", x.unwrap())))
        }
        else {
            Ok(Self(None))
        }
    
    }
}
impl UnimplementedField {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
}