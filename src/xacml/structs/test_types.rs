#![allow(unused_imports)] 
// Disable unused import warnings for tests
#[cfg(test)]
use super::*;
use quick_xml::{de::from_str, se::to_string};
use std::fs;
use std::path::PathBuf;
use std::env;
use serde::Deserialize;

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
    let policy_object: PolicyType = from_str(policy).unwrap();
    to_string(&policy_object).unwrap();
    //let string = to_string(&policy_object).unwrap();
    //print!("Policy Struct: \n \n {:?}\n\n", policy_object);
    //print!("Serialized Struct: \n \n {}", string);
}

#[test]
fn test_single_file() {
    let base_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("helper/tests/xacml-3.0-from-2.0-ct/mandatory");
    let test_case_path = base_path.join("IIF310_FIXED_NO_XPATH");
    let test_file_path = test_case_path.join("Request.xml");

    let policy_object: RequestType = deserialize_file(&test_file_path).unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
//#[serde(untagged)]
enum PolicySetOrPolicy {
    #[serde(rename = "PolicySet")]
    PolicySet(PolicySetType),
    #[serde(rename = "Policy")]
    Policy(PolicyType),
}

#[test]
fn test_xacml_deserialization() {
    let base_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("helper/tests/xacml-3.0-from-2.0-ct/");
    let categories = ["mandatory", "optional", "unsupported"];

    let mut failed_tests = Vec::new();
    let mut number_failed_tests = 0;
    let mut successful_tests = Vec::new();
    let mut number_successful_tests = 0;

    for category in &categories {
        // as of now, only include mandatory tests
        if category != &"mandatory" {
            continue;
        }
        let category_path = base_path.join(category);

        if let Ok(entries) = fs::read_dir(&category_path) {
            for entry in entries.flatten() {
                let test_case_dir = entry.path();

                if test_case_dir.is_dir() {
                    let policy_file: PathBuf = if test_case_dir.to_str().unwrap().contains("IIE00") {
                        test_case_dir.join("Policies/Policy.xml")
                    }
                    else {
                        test_case_dir.join("Policy.xml")
                    };
                    let request_file = test_case_dir.join("Request.xml");
                    let response_file = test_case_dir.join("Response.xml");
                    let mut success = true;

                    if let Err(e) = deserialize_file::<PolicySetOrPolicy>(&policy_file) {
                        if let quick_xml::DeError::Custom(msg) = &e {
                            if msg == "IGNORED" {
                                println!("Skipping test {:?} due to unimplemented data type", test_case_dir);
                                continue;
                            }
                        }
                        println!("Failed to deserialize Policy.xml in {:?}: {}", test_case_dir, e);
                        success = false;
                        number_failed_tests += 1;
                    } else {
                        number_successful_tests += 1;
                    }
                     
                    
                    if let Err(e) = deserialize_file::<RequestType>(&request_file) {
                        if let quick_xml::DeError::Custom(msg) = &e {
                            if msg == "No such file or directory (os error 2)" || msg == "IGNORED" {
                                println!("Skipping test 'Request' {:?} due to unavailable file or unimplemented data type", test_case_dir);
                                continue;
                            }
                        }
                        println!("Failed to deserialize Request.xml in {:?}: {}", test_case_dir, e);
                        success = false;
                        number_failed_tests += 1;
                    } else {
                        number_successful_tests += 1;
                    } 
                    /* 
                    if let Err(e) = deserialize_file::<ResponseType>(&response_file) {
                        if let quick_xml::DeError::Custom(msg) = &e {
                            if msg == "No such file or directory (os error 2)" || msg == "IGNORED" {
                                println!("Skipping test 'Response' {:?} due to unavailable file or unimplemented data type", test_case_dir);
                                continue;
                            }
                        }
                        println!("Failed to deserialize Response.xml in {:?}: {}", test_case_dir, e);
                        success = false;
                        number_failed_tests += 1;
                    } else {
                        number_successful_tests += 1;
                    }
                    */
                    if success {
                        successful_tests.push(test_case_dir.display().to_string());
                    } else {
                        failed_tests.push(test_case_dir.display().to_string());
                    }
                    
                }
            }
        }
    }

    if !failed_tests.is_empty() {
        panic!(
            "Deserialization failed for {} Testcases:\n{}\n{} successful tests:\n{}",
            number_failed_tests,
            failed_tests.join("\n"),
            number_successful_tests,
            successful_tests.join("\n")
        );
    }
    else {
        println!("All {} tests passed successfully", number_successful_tests);
    }
}

fn deserialize_file<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T, quick_xml::DeError> {
    let content = fs::read_to_string(path).map_err(|e| quick_xml::DeError::Custom(e.to_string()))?;
    match from_str::<T>(&content) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            if let quick_xml::DeError::Custom(msg) = &e {
                if msg == "Unimplemented data type" {
                    return Err(quick_xml::DeError::Custom("IGNORED".to_string()));
                }
            }
            Err(e)
        }
    }
}
