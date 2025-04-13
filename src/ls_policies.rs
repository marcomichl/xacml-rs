#[cfg(test)]
use crate::xacml::structs::*;
use crate::xacml::enums::*;

#[test]
#[ignore = "Creates file for paper use"]
fn sl_threshold_check() {
    let policy = PolicyTypeBuilder::default()
        .policy_id("sl_check_projected_probability")
        .version(VersionType("0.1".to_string()))
        .rule_combining_alg_id(RuleCombiningAlgorithms::DenyOverrides)
        .description("Checks if the projected probability matches the required value")
        .rule(vec![
            RuleTypeBuilder::default()
                .rule_id("calculate_check_projected_probability")
                .effect(EffectType::Permit)
                .description("Rule calculates and compares projected probability")
                .condition(
                    ConditionTypeBuilder::default()
                        .expression(
                            ExpressionType::Apply(
                            ApplyTypeBuilder::default()
                                .function_id(FunctionId::DoubleGreaterThan)
                                .description("Compare actual and required value")
                                .expression(vec![
                                    ExpressionType::AttributeValue(
                                        AttributeValueTypeBuilder::default()
                                            // Value of required projected probability
                                            .build().unwrap() // AttributeValue
                                    ), // AttributeValue
                                    ExpressionType::Apply (
                                        ApplyTypeBuilder::default()
                                            // multiply base rate and uncertainty
                                            // add belief
                                            .build().unwrap() // ApplyType
                                    )   //Apply
                                ])
                                .build().unwrap()
                            )
                        )
                        .build().unwrap() // ConditionType
                    )
                .build().unwrap() // RuleType
            ] // Rule Vec
        )
        .build().unwrap(); // PolicyType

}