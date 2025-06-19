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
       
        match target_result {
            TargetResult::NoMatch => {
                log(LogLevel::DEBUG, &format!("Rule {} did not match target, NotApplicable", self.rule_id));
                Ok(RuleResult::NotApplicable)
            },
            TargetResult::Indeterminate => {
                let return_value = match self.effect {
                    EffectType::Deny => RuleResult::IndeterminateD,
                    EffectType::Permit => RuleResult::IndeterminateP
                };
                log(LogLevel::DEBUG, &format!("Rule {} target evaluation indeterminate, {:?}", self.rule_id, &return_value));
                Ok(return_value)
            },
            TargetResult::Match => {
                match self.condition.is_none() || self.condition.as_ref().unwrap().evaluate(request)? {
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

mod rule_test{
    use super::*;

    #[test]
    fn test_emtpy_rule_evaluation() {
        let rule = create_empty_rule();
        println!("{:?}", rule.evaluate_rule(&create_request()).unwrap());
    }

    #[test]
    fn test_emtpy_rule_evaluation_conflicting_target() {
        let rule = create_rule_conflicting_target();
        assert_eq!(rule.evaluate_rule(&create_request()).unwrap(), RuleResult::IndeterminateP);
        let rule2 = RuleType {
            effect: EffectType::Deny,
            ..rule
        };
        assert_eq!(rule2.evaluate_rule(&create_request()).unwrap(), RuleResult::IndeterminateD);
    }

    #[test]
    fn test_rule_matching_target_and_condition() {
        let rule = create_rule_matching_target();
        assert_eq!(rule.evaluate_rule(&create_request()).unwrap(), RuleResult::Permit);
        let rule2 = RuleType {
            effect: EffectType::Deny,
            ..rule
        };
        assert_eq!(rule2.evaluate_rule(&create_request()).unwrap(), RuleResult::Deny);
    }

    fn create_empty_rule() -> RuleType {
        RuleTypeBuilder::default()
            .rule_id("test_rule_id")
            .effect(EffectType::Permit)
            .build().unwrap()
    }

    fn create_rule_conflicting_target() -> RuleType {
        RuleTypeBuilder::default()
            .rule_id("test_rule_id")
            .effect(EffectType::Permit)
            .target(TargetTypeBuilder::default()
                .any_of(vec![AnyOfTypeBuilder::default()
                    .all_of(vec![AllOfTypeBuilder::default()
                            ._match(vec![MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::String)
                                    .value(Value::String("vehicle_cam_acceptance".to_string()))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("request_context")
                                    .data_type(DataType::Integer)
                                    .category("request_context")
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::StringEqual)
                                .build().unwrap()]) // MatchTypeBuilder
                        .build().unwrap()])// AllOfTypeBuilder
                    .build().unwrap()])  // AnyOfTypeBuilder
                .build().unwrap()) //TargetTypeBuilder)
            .build().unwrap()
    }
    
    fn create_rule_matching_target() -> RuleType {
        RuleTypeBuilder::default()
            .rule_id("test_rule_id")
            .effect(EffectType::Permit)
            .target(TargetTypeBuilder::default()
                .any_of(vec![AnyOfTypeBuilder::default()
                    .all_of(vec![AllOfTypeBuilder::default()
                            ._match(vec![MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::Integer)
                                    .value(Value::Integer(5))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("request_context")
                                    .data_type(DataType::Integer)
                                    .category("request_context")
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::IntegerEqual)
                                .build().unwrap()]) // MatchTypeBuilder
                        .build().unwrap()])// AllOfTypeBuilder
                    .build().unwrap()])  // AnyOfTypeBuilder
                .build().unwrap()) //TargetTypeBuilder
            .condition(ConditionTypeBuilder::default()
                .expression(ExpressionType::Apply(ApplyTypeBuilder::default()
                    .function_id(FunctionId::IntegerEqual)
                    .expression(vec![ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                            .attribute_id("request_context")
                            .data_type(DataType::Integer)
                            .category("request_context")
                            .must_be_present(true)
                            .build().unwrap()), // AttributeDesignatoreType
                        ExpressionType::AttributeValue(AttributeValueTypeBuilder::default()
                            .data_type(DataType::Integer)
                            .value(Value::Integer(5))
                            .build().unwrap()) // AttributeValueType
                        ]) // vec expression
                    .build().unwrap())) // ApplyType
                .build().unwrap() // ConditionTypeBuilder
            )
            .build().unwrap()
    }

    fn create_request() -> RequestType {
        RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![
                 AttributesTypeBuilder::default()
                .category("request_context")
                .attribute(vec![
                    AttributeTypeBuilder::default()
                        .attribute_id("request_context")
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Integer)
                                .value(Value::Integer(5))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                ]).build().unwrap()
            ])
            .build()
            .unwrap()
    }
}