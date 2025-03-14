use crate::utils::*;

use super::*;

/// 5.21 RuleType
/// Defines a rule in a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RuleType {
    #[serde(rename = "@RuleId")]
    rule_id: String,
    #[serde(rename = "@Effect")]
    effect: EffectType,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    target: Option<TargetType>,
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    condition: Option<ConditionType>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligation_expressions: Option<ObligationExpressionsType>,
    #[serde(rename = "AdviceExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    advice_expressions: Option<AdviceExpressionsType>
}

impl RuleType {
    /// 7.11 Rule Evaluation
    pub (crate) fn evaluate_rule(&self, request: &RequestType) -> Result<DecisionType, XacmlError> {
        return Err(XacmlError::new(XacmlErrorType::NotImplemented, "Rule evaluation not implemented".to_string()));
    }
}