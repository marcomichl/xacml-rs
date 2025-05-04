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
    pub (crate) fn evaluate_rule(&self, request: &RequestType) -> Result<RuleResult, XacmlError> {
        // Rule has to evaluate the target and the condition to decide if the effect applies
        let mut result: RuleResult ;
        let mut reason= "Condition";
        let target_result = self.target.as_ref().unwrap_or(&TargetType{any_of: None}).match_request(request)?;
        if target_result == TargetResult::NoMatch
        {
            result = RuleResult::NotApplicable;
            reason = "Target";
        }
        else if target_result == TargetResult::Indeterminate
        {
            result = match self.effect {
                EffectType::Deny => RuleResult::IndetermianteD,
                EffectType::Permit => RuleResult::IdeterminateP
            };
            reason = "Target";
        }
        else if self.condition.as_ref().unwrap().evaluate(request)?
        {
            result = match self.effect {
                EffectType::Deny => RuleResult::Deny,
                EffectType::Permit => RuleResult::Permit
            };
        }
        else {
            result =RuleResult::NotApplicable;
        };
        log(LogLevel::DEBUG, &format!("Rule {} evaluated to {:?} because of {}", self.rule_id, result, reason));
        return Ok(result)
        // Todo: Extended Indetermination not yet implemented
    }
}