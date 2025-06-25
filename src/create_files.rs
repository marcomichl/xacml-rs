#[cfg(test)]


use crate::xacml::RuleTypeBuilder;
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{utils::*, xacml::*};


#[test]
#[ignore]
fn create_policy(){
    let policy: PolicyType = PolicyTypeBuilder::default()
        .policy_id("urn:sl-xacml:policy:01")
        .version(VersionType("0.1".to_string()))
        .rule_combining_alg_id(RuleCombiningAlgorithms::DenyOverrides)
        .description("Example policy")
        .target(TargetTypeBuilder::default().build().unwrap())
        .rule(
            vec![
                    RuleTypeBuilder::default()
                    .rule_id("urn:sl-xacml:rule:01")
                    .effect(EffectType::Permit)
                    .description("First rule of the example policy")
                    .target(
                        TargetTypeBuilder::default()
                            .any_of(
                                vec![
                                    AnyOfTypeBuilder::default()
                                        .all_of(
                                            vec![
                                                AllOfTypeBuilder::default()
                                                    ._match(
                                                        vec![
                                                            MatchTypeBuilder::default()
                                                                .match_id(FunctionId::StringEqual)
                                                                .attribute_value(AttributeValueTypeBuilder::default()
                                                                    .data_type(DataType::String)
                                                                    .value(Value::String("employee".to_string()))
                                                                    .build().unwrap()
                                                                )
                                                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                                                    .category(Categories::Other("urn:sl-xacml:subject-category:access-subject".to_string()))
                                                                    .attribute_id(AttributeIdentifiers::Other("urn:sl-xacml:attribute:role".to_string()))
                                                                    .data_type(DataType::String)
                                                                    .must_be_present(false)
                                                                    .build().unwrap()
                                                                )
                                                                .build().unwrap()
                                                        ] 
                                                    )
                                                    .build().unwrap()
                                                ]
                                        ).build().unwrap()
                                ]
                            ).build().unwrap()
                    )
                    .build().unwrap()
            ]
        )
        .build().unwrap();
    serialize_to_xml_file(&policy, "policy01.xml").unwrap();
}

#[test]
fn test_evaluate_simple_policy(){
    let request = RequestTypeBuilder::default()
        .return_policy_id_list(false)
        .combined_decision(false)
        .attributes(
            vec![
                AttributesTypeBuilder::default()
                    .category(Categories::Other("urn:sl-xacml:subject-category:access-subject".to_string()))
                    .attribute(
                        vec![
                            AttributeTypeBuilder::default()
                                .attribute_id(AttributeIdentifiers::Other("urn:sl-xacml:attribute:role".to_string()))
                                .include_in_result(false)
                                .attribute_value(vec![
                                    AttributeValueTypeBuilder::default()
                                        .data_type(DataType::String)
                                        .value(Value::String("employee".to_string()))
                                        .build().unwrap()
                                    ])
                                .build().unwrap()
                        ])
                    .build().unwrap()
            ])
        .build().unwrap();
    let response = pdp::decide_request(request, "policy01.xml").unwrap();
    println!("{:?}", response);
}
