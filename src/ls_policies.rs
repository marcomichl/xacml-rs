use crate::xacml::structs::*;
use crate::xacml::enums::{data_types::DataType, *};
use crate::utils::*;

#[test]
#[ignore = "Creates file for paper use"]
fn store_policy() {
    let policy = create_policy();
    serialize_to_xml_file(&policy, "sl_policy.xml").unwrap();
}

#[test]
fn evaluate_policy() {
    let policy = create_policy();
    let request = create_request();

    let result = policy.evaluate_policy(&request).unwrap();
    println!("{:?}", result)
}

fn create_policy() -> PolicyType {
    PolicyTypeBuilder::default()
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
                                    ExpressionType::Apply (
                                        ApplyTypeBuilder::default()
                                            .function_id(FunctionId::DoubleAdd)
                                            .description("Add belief to product of base rate and uncertainty")
                                            .expression(vec![
                                                ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                    .attribute_id("object.belief")
                                                    .data_type(DataType::Double)
                                                    .category("subject")
                                                    .must_be_present(true)
                                                    .build().unwrap()), // AttributeDesignator
                                                ExpressionType::Apply(ApplyTypeBuilder::default()
                                                    .function_id(FunctionId::DoubleMultiply)
                                                    .description("Multiply belief and base rate")
                                                    .expression(vec![
                                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                            .attribute_id("object.uncertainty")
                                                            .data_type(DataType::Double)
                                                            .category("subject")
                                                            .must_be_present(true)
                                                            .build().unwrap()), // AttributeDesignator
                                                        ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                            .attribute_id("object.baserate")
                                                            .data_type(DataType::Double)
                                                            .category("subject")
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
                                            .value(Value::Double(0.9.into()))
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
                .category("subject")
                .attribute(vec![
                    AttributeTypeBuilder::default()
                        .attribute_id("object.belief")
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Double(0.1.into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                    AttributeTypeBuilder::default()
                        .attribute_id("object.uncertainty")
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Double(0.2.into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                    AttributeTypeBuilder::default()
                        .attribute_id("object.baserate")
                        .include_in_result(false)
                        .attribute_value(vec![
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Double(1.0.into()))
                                .build().unwrap() // AttributeValue
                        ]) // vec attribute_value
                        .build().unwrap(), // AttributeType
                ]) // vec attribute
                .build().unwrap() // AttributeType
        ]) //vec attributes
        .build().unwrap() // RequestType
}