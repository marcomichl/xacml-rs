use super::*;
use crate::utils::{XacmlError, XacmlErrorType};

/// 5.9 Match element
/// Shall contain a condition that must be fulfilled by the context to be applicable
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct MatchType {
    #[serde(rename = "@MatchId")]
    match_id: String,                        // More specific of URI type
    #[serde(rename = "AttributeValue")]
    attribute_value: AttributeValueType,
    #[serde(rename = "AttributeDesignator", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_designator: Option<AttributeDesignatorType>,   // Either this or the attributeSelector must be present, not both and not none
    #[serde(rename = "AttributeSelector", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_selector: Option<AttributeSelectorType>
}

impl MatchType{
    pub fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
        let attribute = &self.attribute_value;
        if self.attribute_designator.is_some() {
            let attribute_designator = self.attribute_designator.as_ref().unwrap();
            if !request.attributes.iter()
                .flat_map(|attributes| attributes.get_attribute_values_by_designator(attribute_designator))
                .flatten()
                .filter(|filtered_attribute_value| filtered_attribute_value == &attribute)
                .collect::<Vec<&AttributeValueType>>().is_empty() 
            {
                return Ok(true)
            }
        }
        else if self.attribute_selector.is_some() {
            let attribute_selector = self.attribute_selector.as_ref().unwrap();
            for attributes in &request.attributes {
                let result = attributes.get_attribute_value_by_selector(attribute_selector);
                if result.is_ok() {
                    return Ok(result.unwrap() == attribute);
                }
            }
        }
        else {
            return Err(XacmlError::new(XacmlErrorType::FormatError, "MatchType does not contain a valid attribute designator or selector".to_string()))
        };
        Ok(false)   //Did not find the attribute
    }
}

#[cfg(test)]
mod test_match_type {
    use super::*;

    #[test]
    fn match_request_test () {
        let request1 = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category("TestCategory")
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id("Test-ID")
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(23))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        let request2 = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category("TestCategory")
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id("Test-ID")
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(21))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        let request3 = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category("TestCategory")
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id("Wrong-ID")
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(23))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        let request4 = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category("WrongCategory")
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id("Test-ID")
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(23))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        let request5 = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category("TestCategory")
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id("Test-ID")
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Double)
                        .value(Value::Integer(23))  //Should not happen, for test purpose..
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        let match_type = MatchTypeBuilder::default()
            .attribute_designator(AttributeDesignatorTypeBuilder::default()
                .category("TestCategory")
                .attribute_id("Test-ID")
                .data_type(DataType::Integer)
                .must_be_present(true)
                .build().unwrap()
            )
            .attribute_value(AttributeValueType{data_type: DataType::Integer, value: Value::Integer(23)})
            .match_id("Test-Match")
            .build().unwrap();

        assert_eq!(match_type.match_request(&request1).unwrap(), true);
        assert_eq!(match_type.match_request(&request2).unwrap(), false);
        assert_eq!(match_type.match_request(&request3).unwrap(), false);
        assert_eq!(match_type.match_request(&request4).unwrap(), false);
        assert_eq!(match_type.match_request(&request5).unwrap(), false);
    }
}   