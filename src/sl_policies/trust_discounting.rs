use super::*;

#[test]
#[ignore = "Used for paper"]
fn create_discounting_uncertainty_file() {
    let policy = create_policy_with_rules(vec![create_discounting_rule(0.9)]);
    serialize_to_xml_file(&policy, "Discounting_trust_policy.xml");
}

#[test]
fn test_discounted_decision() {
    let policy = create_policy_with_rules(vec![create_discounting_rule(0.9)]);
    let request = create_request_with_attributes(create_discounting_request_attributes());
    assert_eq!(policy.evaluate_policy(&request).unwrap(), PolicyResult::Permit);
}

fn create_discounting_rule(threshold: f64) -> RuleType {
    RuleTypeBuilder::default()
        .rule_id(create_urn("rule:trust_discounting"))
        .effect(EffectType::Permit)
        .condition(
            ConditionTypeBuilder::default()
                .expression(
                    ExpressionType::Apply(
                        ApplyTypeBuilder::default()
                            .function_id(FunctionId::DoubleGreaterThan)
                            .expression(
                                vec![
                                    ExpressionType::Apply (
                                        subjective_logic_calculate_projected_probability_with_apply(
                                            create_discounted_belief(), 
                                            create_discounted_uncertainty(),
                                            create_discounted_baserate())
                                        ),
                                    ExpressionType::AttributeValue(
                                        AttributeValueTypeBuilder::default()
                                            .data_type(DataType::Double)
                                            .value(Value::Double(threshold))
                                            .build().unwrap() // AttributeValue
                                        )
                                    ]
                            )
                            .build().unwrap()
                    )
                )
                .build().unwrap()
        )
        .build().unwrap()
} 

fn create_discounted_uncertainty() -> ExpressionType {
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
                            ExpressionType::Apply(subjective_logic_calculate_projected_probability_with_urn(
                                    create_urn("attribute:object:belief"),
                                    create_urn("attribute:object:uncertainty"),
                                    create_urn("attribute:object:baserate"))),
                            ExpressionType::Apply(
                                ApplyTypeBuilder::default()
                                    .function_id(FunctionId::DoubleAdd)
                                    .description("Sum up belief in x=X to calculate uncertainty")
                                    .expression(vec![
                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id(AttributeIdentifiers::Other(create_urn("attributes:data:supporting_belief")))
                                            .data_type(DataType::Double)
                                            .category(Categories::AccessSubject)
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

fn create_discounted_belief() -> ExpressionType {
    ExpressionType::Apply(ApplyTypeBuilder::default()
        .function_id(FunctionId::DoubleMultiply)
        .description("Multiply supporting belief with projected probability")
        .expression(vec![
            ExpressionType::Apply(subjective_logic_calculate_projected_probability_with_urn(
                    create_urn("attribute:object:belief"),
                    create_urn("attribute:object:uncertainty"),
                    create_urn("attribute:object:baserate"))),
            ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                .attribute_id(AttributeIdentifiers::Other(create_urn("attributes:data:supporting_belief")))
                .data_type(DataType::Double)
                .category(Categories::AccessSubject)
                .must_be_present(true)
                .build().unwrap())
            ])
        .build().unwrap())
}

fn create_discounted_baserate() -> ExpressionType {
    ExpressionType::AttributeDesignator(
        AttributeDesignatorTypeBuilder::default()
            .attribute_id(AttributeIdentifiers::Other(create_urn("attributes:data:baserate")))
            .data_type(DataType::Double)
            .category(Categories::AccessSubject)
            .must_be_present(true)
            .build().unwrap()
    )
}


fn create_discounting_request_attributes() -> Vec<AttributesType> {
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
                            .value(Value::Double(0.9.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:uncertainty")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(0.05.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:baserate")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(0.9.into()))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(), // AttributeType
            ]) // vec attribute
            .build().unwrap(), // AttributeType 
        AttributesTypeBuilder::default()
            .category(Categories::AccessSubject)
            .attribute(vec![
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attributes:data:supporting_belief")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(1.0))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(),
                AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other(create_urn("attributes:data:baserate")))
                    .include_in_result(false)
                    .attribute_value(vec![
                        AttributeValueTypeBuilder::default()
                            .data_type(DataType::Double)
                            .value(Value::Double(0.5))
                            .build().unwrap() // AttributeValue
                    ]) // vec attribute_value
                    .build().unwrap(),
            ]) // AttributeType
            .build().unwrap(),
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
