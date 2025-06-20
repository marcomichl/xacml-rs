use crate::xacml::*;
use crate::utils::*;

#[test]
#[ignore = "Creates file for paper use"]
fn store_policy() {
    let policy = create_policy(0.6);
    serialize_to_xml_file(&policy, "sl_policy.xml").unwrap();
}

#[test]
#[ignore = "Create file for paper use"]
fn store_attribute(){
    let attribute =  AttributeTypeBuilder::default()
        .attribute_id(AttributeIdentifiers::Other(AttributeIdentifiers::Other("object.belief".to_string()).to_string()))
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
    let policy = create_policy(0.8);
    let request = create_request();
    assert_eq!(policy.evaluate_policy(&request).unwrap(), PolicyResult::Deny);
    let policy = create_policy(0.6);
    assert_eq!(policy.evaluate_policy(&request).unwrap(), PolicyResult::Permit);
}

fn create_policy(threshold: f64) -> PolicyType {
    PolicyTypeBuilder::default()
        .policy_id("sl_check_projected_probability")
        .version(VersionType("0.1".to_string()))
        .rule_combining_alg_id(RuleCombiningAlgorithms::DenyUnlessPermit)
        .description("Checks if the projected probability matches the required value")
        .target(TargetTypeBuilder::default()
            .any_of(vec![AnyOfTypeBuilder::default()
                .all_of(vec![AllOfTypeBuilder::default()
                        ._match(vec![MatchTypeBuilder::default()
                            .attribute_value(AttributeValueTypeBuilder::default()
                                .data_type(DataType::String)
                                .value(Value::String("vehicle_cam_acceptance".to_string()))
                                .build().unwrap()) //AttributeValueTypeBuilder
                            .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                .attribute_id(AttributeIdentifiers::Other(AttributeIdentifiers::Other("request_context".to_string()).to_string()))
                                .data_type(DataType::String)
                                .category(Categories::Action)
                                .must_be_present(true)
                                .build().unwrap()) //AttributeDesignatorTypeBuilder    
                            .match_id(FunctionId::StringEqual)
                            .build().unwrap()]) // MatchTypeBuilder
                    .build().unwrap()])// AllOfTypeBuilder
                .build().unwrap()])  // AnyOfTypeBuilder
            .build().unwrap()) //TargetTypeBuilder
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
                                    ExpressionType::Apply (
                                        ApplyTypeBuilder::default()
                                            .function_id(FunctionId::DoubleAdd)
                                            .description("Add belief to product of base rate and uncertainty")
                                            .expression(vec![
                                                ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                    .attribute_id(AttributeIdentifiers::Other("object.belief".to_string()))
                                                    .data_type(DataType::Double)
                                                    .category(Categories::Resource)
                                                    .must_be_present(true)
                                                    .build().unwrap()), // AttributeDesignator
                                                ExpressionType::Apply(ApplyTypeBuilder::default()
                                                    .function_id(FunctionId::DoubleMultiply)
                                                    .description("Multiply belief and base rate")
                                                    .expression(vec![
                                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                            .attribute_id(AttributeIdentifiers::Other("object.uncertainty".to_string()))
                                                            .data_type(DataType::Double)
                                                            .category(Categories::Resource)
                                                            .must_be_present(true)
                                                            .build().unwrap()), // AttributeDesignator
                                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                            .attribute_id(AttributeIdentifiers::Other("object.baserate".to_string()))
                                                            .data_type(DataType::Double)
                                                            .category(Categories::Resource)
                                                            .must_be_present(true)
                                                            .build().unwrap()), // AttributeDesignator
                                                    ]) // vec expression
                                                    .build().unwrap()) //ApplyType
                                            ]) //vec expression
                                            // multiply base rate and uncertainty
                                            // add belief
                                            .build().unwrap() // ApplyType
                                    ),   //Apply
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
        )
        .build().unwrap() // PolicyType
}

fn create_request() -> RequestType {
    RequestTypeBuilder::default()
        .return_policy_id_list(false)
        .combined_decision(false)
        .attributes(vec![
            AttributesTypeBuilder::default()
                .category(Categories::Resource)
                .attribute(vec![
                    AttributeTypeBuilder::default()
                        .attribute_id(AttributeIdentifiers::Other("object.belief".to_string()))
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Double(0.5.into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                    AttributeTypeBuilder::default()
                        .attribute_id(AttributeIdentifiers::Other("object.uncertainty".to_string()))
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Double(0.2.into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                    AttributeTypeBuilder::default()
                        .attribute_id(AttributeIdentifiers::Other("object.baserate".to_string()))
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
                .category(Categories::Action)
                .attribute(vec![
                    AttributeTypeBuilder::default()
                        .attribute_id(AttributeIdentifiers::Other("request_context".to_string()))
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::String)
                                .value(Value::String("vehicle_cam_acceptance".into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap() // AttributeType
                ]) // Attributes vec
                .build().unwrap()
        ]) //vec attributes
        .build().unwrap() // RequestType
}

#[test]
#[ignore = "Used for paper"]
fn create_discounting_belief_file() {
    let expression = create_discounting_belief();
    serialize_to_xml_file(&expression, "Expression_discount_belief.xml");
}

fn create_discounting_belief() -> ExpressionType {
    ExpressionType::Apply(
        ApplyTypeBuilder::default()
            .function_id(FunctionId::DoubleMultiply)
            .description("Calculate data-based belief value")
            .expression(vec![
                ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other("node_trust_projected_probability".to_string()))
                    .data_type(DataType::Double)
                    .category(Categories::Resource)
                    .must_be_present(true)
                    .build().unwrap()),
                ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other("data_trust_belief".to_string()))
                    .data_type(DataType::Double)
                    .category(Categories::Resource)
                    .must_be_present(true)
                    .build().unwrap())
            ])
        .build().unwrap()
        )
}

#[test]
#[ignore = "Used for paper"]
fn create_discounting_uncertainty_file() {
    let expression = create_discounting_uncertainty();
    serialize_to_xml_file(&expression, "Expression_discount_uncertainty.xml");
}

fn create_discounting_uncertainty() -> ExpressionType {
    ExpressionType::Apply(
        ApplyTypeBuilder::default()
            .function_id(FunctionId::DoubleSubtract)
            .description("Calculate data-based uncertainty value")
            .expression(vec![
                ExpressionType::AttributeValue(
                    AttributeValueTypeBuilder::default()
                        .data_type(DataType::Double)
                        .value(Value::Double(1.0))
                        .build().unwrap() // AttributeValue
                ), // AttributeValue
                ExpressionType::Apply(
                    ApplyTypeBuilder::default()
                        .function_id(FunctionId::DoubleMultiply)
                        .description("Multiply supporting belief with projected probability")
                        .expression(vec![
                            ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                .attribute_id(AttributeIdentifiers::Other("node_trust_projected_probability".to_string()))
                                .data_type(DataType::Double)
                                .category(Categories::Resource)
                                .must_be_present(true)
                                .build().unwrap()),
                            ExpressionType::Apply(
                                ApplyTypeBuilder::default()
                                    .function_id(FunctionId::DoubleAdd)
                                    .description("Sum up belief in x=X to calculate uncertainty")
                                    .expression(vec![
                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id(AttributeIdentifiers::Other("Belief supporting trust in data".to_string()))
                                            .data_type(DataType::Double)
                                            .category(Categories::Resource)
                                            .must_be_present(true)
                                            .build().unwrap())
                            ])
                        .build().unwrap()
                        )
                    ])
                .build().unwrap()
                )
            ])
        .build().unwrap()
        )
}