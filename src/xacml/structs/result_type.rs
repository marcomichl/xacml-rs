use super::*;

/// 5.48 ResultType
/// Contains the result of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ResultType {
    #[serde(rename = "Decision")]
    decision: DecisionType,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status: Option<StatusType>,
    #[serde(rename = "Obligations", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligations: Option<ObligationsType>,
    #[serde(rename = "AssociatedAdvice", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    associated_advice: Option<AssociatedAdviceType>,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attributes: Option<Vec<AttributesType>>,
    #[serde(rename = "PolicyIdentifierList", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_identifier_list: Option<PolicyIdentifierListType> // If set the return_policy_id_list true, this list contains policies that are fully-applicable
}