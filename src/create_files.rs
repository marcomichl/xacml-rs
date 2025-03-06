#[cfg(test)]


use crate::xacml::structs::RuleTypeBuilder;
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::xacml::structs::*;
#[allow(unused_imports)]
use crate::xacml::enums::{combining_algorithms::*, data_types::*};


#[test]
fn create_policy(){
    let policy: PolicyType = PolicyTypeBuilder::default()
        .policy_id("urn:sl-xacml:policy:01")
        .version(VersionType("0.1".to_string()))
        .rule_combining_alg_id(RuleCombiningAlgorithms::DenyOverrides)
        .description("Policy 01 for the sl-xacml paper")
        .target(Vec::<TargetType>::new())
        .rule(
            vec![
                    RuleTypeBuilder::default()
                    .rule_id("urn:sl-xacml:rule:01")
                    .effect(EffectType::Permit)
                    .description("Rule 01 for the sl-xacml paper")
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
                                                                .match_id("urn:oasis:names:tc:xacml:1.0:function:string-equal")
                                                                .attribute_value(AttributeValueTypeBuilder::default()
                                                                    .data_type(DataType::String)
                                                                    .value(Value::String("role".to_string()))
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