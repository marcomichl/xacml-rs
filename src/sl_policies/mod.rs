mod trust_discounting;

use crate::xacml::*;
use crate::utils::*;


#[test]
#[ignore = "Creates file for paper use"]
fn store_policy() {
    let policy = create_policy_with_rules(subjective_logic_policy_rules_vec(0.6));
    serialize_to_xml_file(&policy, "sl_policy.xml").unwrap();
}

#[test]
#[ignore = "Create file for paper use"]
fn store_attribute(){
    let attribute =  AttributeTypeBuilder::default()
        .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:belief")))
        .include_in_result(false)
        .attribute_value(vec![
            AttributeValueTypeBuilder::default()
                .data_type(DataType::Double)
                .value(Value::Double(0.1.into()))
                .build().unwrap() // AttributeValue
        ]) // vec attribute_value
        .build().unwrap(); // AttributeType
    serialize_to_xml_file(&attribute, "sl_request_attribute.xml");
}

#[test]
#[ignore]
fn evaluate_policy() {
    let policy = create_policy_with_rules(subjective_logic_policy_rules_vec(0.8));
    let request = create_request_with_attributes(subjective_logic_request_attributes());
    assert_eq!(policy.evaluate_policy(&request).unwrap(), PolicyResult::Deny);
    let policy = create_policy_with_rules(subjective_logic_policy_rules_vec(0.6));
    assert_eq!(policy.evaluate_policy(&request).unwrap(), PolicyResult::Permit);
}


fn create_request_with_attributes(attributes: Vec<AttributesType>) -> RequestType {
    RequestTypeBuilder::default()
        .return_policy_id_list(false)
        .combined_decision(false)
        .attributes(attributes) //vec attributes
        .build().unwrap() // RequestType
}

fn create_policy_with_rules(rules: Vec<RuleType>) -> PolicyType {
    PolicyTypeBuilder::default()
        .policy_id(create_urn("policy:sl_check_projected_probability "))
        .version(VersionType("0.1".to_string()))
        .rule_combining_alg_id(RuleCombiningAlgorithms::DenyUnlessPermit)
        .description("Checks if the projected probability is higher than the RTL")
        .target(TargetTypeBuilder::default()
            .any_of(vec![AnyOfTypeBuilder::default()
                .all_of(vec![AllOfTypeBuilder::default()
                        ._match(vec![MatchTypeBuilder::default()
                            .attribute_value(AttributeValueTypeBuilder::default()
                                .data_type(DataType::String)
                                .value(Value::String("CIM_received_data".to_string()))
                                .build().unwrap()) //AttributeValueTypeBuilder
                            .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:context")))
                                .data_type(DataType::String)
                                .category(Categories::RecipientSubject)
                                .must_be_present(true)
                                .build().unwrap()) //AttributeDesignatorTypeBuilder    
                            .match_id(FunctionId::StringEqual)
                            .build().unwrap()]) // MatchTypeBuilder
                    .build().unwrap()])// AllOfTypeBuilder
                .build().unwrap()])  // AnyOfTypeBuilder
            .build().unwrap()) //TargetTypeBuilder
        .rule(rules)
        .build().unwrap() // PolicyType
}

fn subjective_logic_request_attributes() -> Vec<AttributesType> {
    vec![
        AttributesTypeBuilder::default()
            .category(Categories::Resource)
            .attribute(vec![
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:belief")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(0.5.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:uncertainty")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(0.2.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:baserate")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(1.0.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
            ]) // vec attribute
            .build().unwrap(), // AttributeType 
        AttributesTypeBuilder::default()
            .category(Categories::RecipientSubject)
            .attribute(vec![
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:context")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::String)
                            .value(Value::String("CIM_received_data".into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap()]) // AttributeType
            .build().unwrap()
    ] // Attributes vec
}

fn subjective_logic_policy_rules_vec(threshold: f64) -> Vec<RuleType> {
    vec![
        RuleTypeBuilder::default()
            .rule_id(create_urn("rule:compare_projected_probability"))
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
                                ExpressionType::Apply(subjective_logic_calculate_projected_probability_with_urn(
                                    create_urn("attribute:object:belief"),
                                    create_urn("attribute:object:uncertainty"),
                                    create_urn("attribute:object:baserate"))),
                                ExpressionType::AttributeValue(
                                    AttributeValueTypeBuilder::default()
                                        .data_type(DataType::Double)
                                        .value(Value::Double(threshold))
                                        .build().unwrap() // AttributeValue
                                ) // AttributeValue
                            ])
                            .build().unwrap()
                        )
                    )
                    .build().unwrap() // ConditionType
                )
            .build().unwrap() // RuleType
        ] // Rule Vec
}

fn subjective_logic_calculate_projected_probability_with_urn(belief_id: String, unceratainty_id: String, baserate_id: String) -> ApplyType {

    let belief = ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
        .attribute_id(AttributeIdentifiers::Other(belief_id.into()))
        .data_type(DataType::Double)
        .category(Categories::Resource)
        .must_be_present(true)
        .build().unwrap());
    let uncertainty = ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
        .attribute_id(AttributeIdentifiers::Other(unceratainty_id.into()))
        .data_type(DataType::Double)
        .category(Categories::Resource)
        .must_be_present(true)
        .build().unwrap());
    let baserate = ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
        .attribute_id(AttributeIdentifiers::Other(baserate_id.into()))
        .data_type(DataType::Double)
        .category(Categories::Resource)
        .must_be_present(true)
        .build().unwrap());
    subjective_logic_calculate_projected_probability_with_apply(belief, uncertainty, baserate)
}

fn subjective_logic_calculate_projected_probability_with_apply(belief: ExpressionType, uncertainty: ExpressionType, baserate: ExpressionType) -> ApplyType {
    ApplyTypeBuilder::default()
        .function_id(FunctionId::DoubleAdd)
        .description("Add belief to product of base rate and uncertainty")
        .expression(vec![
            belief, 
            ExpressionType::Apply(ApplyTypeBuilder::default()
                .function_id(FunctionId::DoubleMultiply)
                .description("Multiply belief and base rate")
                .expression(vec![
                    uncertainty, // AttributeDesignator
                    baserate, // AttributeDesignator
                ]) // vec expression
                .build().unwrap()) //ApplyType
        ]) //vec expression
        // multiply base rate and uncertainty
        // add belief
        .build().unwrap() // ApplyType

}