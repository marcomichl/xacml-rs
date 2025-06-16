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
    pub (super) fn evaluate_rule(&self, request: &RequestType) -> Result<RuleResult, XacmlError> {
        let target_result = self.target.as_ref().unwrap_or(&TargetType{any_of: None}).match_request(request)?;
       
        return match target_result {
            TargetResult::NoMatch => {
                log(LogLevel::DEBUG, &format!("Rule {} did not match target, NotApplicable", self.rule_id));
                Ok(RuleResult::NotApplicable)
            },
            TargetResult::Indeterminate => {
                let return_value = match self.effect {
                    EffectType::Deny => RuleResult::IndetermianteD,
                    EffectType::Permit => RuleResult::IndeterminateP
                };
                log(LogLevel::DEBUG, &format!("Rule {} target evaluation indeterminate, {:?}", self.rule_id, &return_value));
                Ok(return_value)
            },
            TargetResult::Match => {
                match self.condition.as_ref().unwrap().evaluate(request)? {
                    true => {
                        log(LogLevel::DEBUG, &format!("Rule {} match target and condition, {:?}", self.rule_id, self.effect));
                        match self.effect {
                            EffectType::Deny =>  Ok(RuleResult::Deny),
                            EffectType::Permit =>  Ok(RuleResult::Permit)
                        }
                    },
                    false => {
                        log(LogLevel::DEBUG, &format!("Rule {} match target but not condition, NotApplicable", self.rule_id));
                        Ok(RuleResult::NotApplicable)
                    }
                }
            }
        }
    }
}