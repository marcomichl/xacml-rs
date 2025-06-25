#![feature(test)]
extern crate test;
use test::Bencher;
use paste::paste;
use xacml_rs::{xacml::*, utils::*};

#[bench]
fn benchmark_subjective_logic_policy_evaluation(b: &mut Bencher) {
    let policy = create_policy_with_rules(subjective_logic_policy_rules_vec(0.8));
    let request = create_request_with_attributes(subjective_logic_request_attributes());
    b.iter(|| {
        let _ = policy.evaluate_policy(&request);
    });
}

// Helper macro to generate multiple benchmarks with different rule counts
macro_rules! bench_multiple_rule_policy {
    ($($n:expr),*) => {
        $(
            paste! {
                #[bench]
                fn [<bench_multiple_rule_policy_ $n>](b: &mut Bencher) {
                    let (policy, request) = create_policy_varying_attributes($n);
                    b.iter(|| {
                        let _ = policy.evaluate_policy(&request);
                    });
                }
            }
        )*
    };
}

// Use the macro to generate benchmarks for the desired rule counts
bench_multiple_rule_policy!(0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100);

fn create_policy_varying_attributes(n: i32) -> (PolicyType, RequestType) {
    let mut rules: Vec<RuleType> = Vec::new();
    let mut attribute_type_vec: Vec<AttributeType> = Vec::new();
    for i in 0..n {
        // Example: create n attribute designators and add to expressions
        let attr_id = create_urn(&format!("attribute:object:attr{}", i));
        let expr = RuleTypeBuilder::default()
            .rule_id(create_urn(&format!("rule_{}", i)))
            .effect(EffectType::Permit)
            .condition(
                ConditionTypeBuilder::default()
                    .expression(ExpressionType::Apply(ApplyTypeBuilder::default()
                    .function_id(FunctionId::IntegerEqual)
                    .expression(vec![
                        ExpressionType::AttributeDesignator(
                            AttributeDesignatorTypeBuilder::default()
                                .attribute_id(AttributeIdentifiers::Other(attr_id.clone()))
                                .data_type(DataType::Integer)
                                .category(Categories::Resource)
                                .must_be_present(true)
                                .build().unwrap()),
                        ExpressionType::AttributeValue(
                            AttributeValueTypeBuilder::default()
                                .data_type(DataType::Double)
                                .value(Value::Integer(i as i64))
                                .build().unwrap())
                        ])
                    .build().unwrap()))
                .build().unwrap()
                        )
            .build().unwrap();

        rules.push(expr);

        let attr_type = AttributeTypeBuilder::default()
            .attribute_id(AttributeIdentifiers::Other(attr_id))
            .include_in_result(false)
            .attribute_value(vec![
                AttributeValueTypeBuilder::default()
                    .data_type(DataType::Integer)
                    .value(Value::Integer(i as i64))
                    .build().unwrap()
            ])
            .build().unwrap();
        attribute_type_vec.push(attr_type);
    };

    let policy = create_policy_with_rules(rules);

    let attributes_vec = vec![
        AttributesTypeBuilder::default()
            .category(Categories::Resource)
            .attribute(attribute_type_vec)
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
                    .build().unwrap() // AttributeType
            ]) // Attributes vec
            .build().unwrap()
        ];
    
    let request =  create_request_with_attributes(attributes_vec);
    return (policy, request)
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
                                ExpressionType::Apply (
                                    ApplyTypeBuilder::default()
                                        .function_id(FunctionId::DoubleAdd)
                                        .description("Add belief to product of base rate and uncertainty")
                                        .expression(vec![
                                            ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:belief")))
                                                .data_type(DataType::Double)
                                                .category(Categories::Resource)
                                                .must_be_present(true)
                                                .build().unwrap()), // AttributeDesignator
                                            ExpressionType::Apply(ApplyTypeBuilder::default()
                                                .function_id(FunctionId::DoubleMultiply)
                                                .description("Multiply belief and base rate")
                                                .expression(vec![
                                                    ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                        .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:uncertainty")))
                                                        .data_type(DataType::Double)
                                                        .category(Categories::Resource)
                                                        .must_be_present(true)
                                                        .build().unwrap()), // AttributeDesignator
                                                    ExpressionType::AttributeDesignator(AttributeDesignatorTypeBuilder::default()
                                                        .attribute_id(AttributeIdentifiers::Other(create_urn("attribute:object:baserate")))
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
}