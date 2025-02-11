#![allow(unused_imports)] 
// Disable unused import warnings for tests
#[cfg(test)]
use super::*;
use quick_xml::{de::from_str, se::to_string};

#[test]
fn test_simple_policy() {
    let policy = r#"<?xml version="1.0" encoding="UTF-8"?>
<Policy
    xmlns="urn:oasis:names:tc:xacml:3.0:core:schema:wd-17"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="urn:oasis:names:tc:xacml:3.0:core:schema:wd-17
    http://docs.oasis-open.org/xacml/3.0/xacml-core-v3-schema-wd-17.xsd"
    PolicyId="urn:oasis:names:tc:xacml:3.0:example:SimplePolicy1"
    Version="1.0"
    RuleCombiningAlgId="urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:deny-overrides">
    <Description>
        Medi Corp access control policy
    </Description>
    <Target/>
    <Rule
        RuleId= "urn:oasis:names:tc:xacml:3.0:example:SimpleRule1"
        Effect="Permit">
        <Description>
            Any subject with an e-mail name in the med.example.com domain
            can perform any action on any resource.
        </Description>
        <Target>
            <AnyOf>
                <AllOf>
                    <Match
                        MatchId="urn:oasis:names:tc:xacml:1.0:function:rfc822Name-match">
                    <AttributeValue
                        DataType="http://www.w3.org/2001/XMLSchema#double"
                        >0.9</AttributeValue>
                    <AttributeDesignator
                        MustBePresent="false"
                        Category="urn:oasis:names:tc:xacml:1.0:subject-category:access-subject"
                        AttributeId="urn:oasis:names:tc:xacml:1.0:subject:subject-id"
                        DataType="urn:oasis:names:tc:xacml:1.0:data-type:rfc822Name"/>
                    </Match>
                </AllOf>
            </AnyOf>
        </Target>
    </Rule>
 </Policy>"#;
    let policy_object: Policy = from_str(policy).unwrap();
    let String = to_string(&policy_object).unwrap();
    print!("Policy Struct: \n \n {:?}\n\n", policy_object);
    print!("Serialized Struct: \n \n {}", String);
}

