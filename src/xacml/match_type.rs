use super::*;
use crate::utils::{XacmlError, XacmlErrorType};

/// 5.9 Match element
/// Shall contain a condition that must be fulfilled by the context to be applicable
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct MatchType {
    #[serde(rename = "@MatchId")]
    match_id: FunctionId,                        // More specific of URI type
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
    /// Has to be rewritten according to 7.6 (Match Evaluation)
    /// There are more functions that "simple" string match, comparable to the functions in Apply structures..
    pub fn match_request(&self, request: &RequestType) -> Result<TargetResult, XacmlError> {
        println!("Starting Match Evaluation");
        let request_attribute_value_results = 
        if self.attribute_designator.is_some() && self.attribute_selector.is_none(){
            request.attributes.iter()
                .map(|attributes| attributes.get_attribute_values_by_designator(self.attribute_designator.as_ref().unwrap()))
                .collect::<Vec<Result<Vec<&AttributeValueType>, XacmlError>>>()
        }
        else if self.attribute_selector.is_some() && self.attribute_designator.is_none() {
            request.attributes.iter()
                .map(|attributes| attributes.get_attribute_values_by_selector(self.attribute_selector.as_ref().unwrap()))
                .collect::<Vec<Result<Vec<&AttributeValueType>, XacmlError>>>()
        }
        else {
            return Err(XacmlError::new(XacmlErrorType::FormatError, "MatchType does not contain a valid attribute designator or selector, or both are defined".to_string()))
        };
        // Operational Errors -> Ideterminate
        if request_attribute_value_results.iter().any(|r| r.is_err()) {
            return Ok(TargetResult::Indeterminate)
        }
        // at this point, we have the necessary attribute values from the request
        let request_attribute_values = request_attribute_value_results.into_iter()
            .map(|r| r.unwrap())
            .flatten()
            .collect::<Vec<&AttributeValueType>>();
        // Designator and Selector did not bring a result
        if request_attribute_values.is_empty() {
            return Ok(TargetResult::NoMatch)
        }
        println!("Using the following attributes: {:?}", request_attribute_values);
        
        // One Match -> Match

        //todo: Application of the function in match_id, verify that return type is bool, error -> indeterminate
        let request_values: Vec<&Value> = request_attribute_values.into_iter().map(|v| &v.value).collect();
        
        let match_results: Vec<Result<Vec<Value>, XacmlError>> = request_values.into_iter().map(|r| self.match_id.apply_function(vec![&self.attribute_value.value, r])).collect();
        // This should only return one value per applied function, and that should be of type boolean
        if match_results.iter().any(|r| r.is_err()) {
            return Ok(TargetResult::Indeterminate)
        }
        let match_result_values: Vec<Value> = match_results.into_iter().flatten().flatten().collect();
        if match_result_values.iter().any(|r| match r {
            Value::Boolean(b) => return *b,
            _ => return false
        }) {
            return Ok(TargetResult::Match)
        }
        Ok(TargetResult::NoMatch)   //Did not find the attribute
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
                        .data_type(DataType::Integer)
                        .value(Value::Double(23.0))  //Should not happen, for test purpose..
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
            .match_id(FunctionId::IntegerEqual)
            .build().unwrap();

        assert_eq!(match_type.match_request(&request1).unwrap(), TargetResult::Match);
        assert_eq!(match_type.match_request(&request2).unwrap(), TargetResult::NoMatch);    // Only wrong evaluation
        assert_eq!(match_type.match_request(&request3).unwrap(), TargetResult::NoMatch);    // Designator returns empty bag (Id)
        assert_eq!(match_type.match_request(&request4).unwrap(), TargetResult::NoMatch);    // Designator returns empty bag (Category)
        assert_eq!(match_type.match_request(&request5).unwrap(), TargetResult::Indeterminate);  //Wrong data type given to function
    }
}   