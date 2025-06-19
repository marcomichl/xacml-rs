use super::*;
use crate::utils::*;

/// Shall appear in the policy set or the policy elements, may be contained in a rule element
/// Shall contain a conjunctive sequence of <AnyOf> elements, to be applicable one of these has to match the decision request.
/// Each AnyOf element contains a disjunctive AllOf element, that all have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct TargetType {
    #[serde(rename = "AnyOf", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub (super) any_of: Option<Vec<AnyOfType>>                  // Data type for elements, of which one must match the context to be applicable; if empty, the target is always applicable; might be changed to a simple vec, that can also be of length 0
}


impl TargetType {
    pub fn match_request(&self, request: &RequestType) -> Result<TargetResult, XacmlError> {
        if self.any_of.is_none() || self.any_of.as_ref().unwrap().is_empty() {       // Should not fail because of lazy evaluation
            return Ok(TargetResult::Match);
        }
        let any_of_results = self.any_of.as_ref().unwrap().iter()
            .map(|r| r.match_request(request))
            .collect::<Result<Vec<TargetResult>, XacmlError>>()?;
        if any_of_results.iter().all(|r| *r == TargetResult::Match)
        {
            return Ok(TargetResult::Match)
        }
        else if any_of_results.iter().any(|r| *r == TargetResult::NoMatch)
        {
            return Ok(TargetResult::NoMatch);
        }
        else 
        {
            return Ok(TargetResult::Indeterminate)    
        }
    }
}

mod test_target_type{
    use super::*;

    #[test]
    fn test_target_type_match_request() {
        let target = create_target_1();
        assert_eq!(target.match_request(&create_request()).unwrap(), TargetResult::Match);
    }

    /// Two contradicting match elements in AllOf type -> NoMatch
    #[test]
    fn test_target_type_match_request_no_match() {
        let target = create_target_2();
        assert_eq!(target.match_request(&create_request()).unwrap(), TargetResult::NoMatch);
    }

    #[test]
    fn test_target_type_match_request_indeterminate() { // Wrong function ID used (StringEqual with two Double Values)
        let target = create_target_3();
        assert_eq!(target.match_request(&create_request()).unwrap(), TargetResult::Indeterminate);
    }

    /// In this test a target type has two AnyOf Types, one matches one not
    /// According to 7.7 Table 1 the target's result shall be NoMatch
    #[test]
    fn test_target_type_match_request_contradicting_anyof_elements() {
        let target = create_target_4();
        assert_eq!(target.match_request(&create_request()).unwrap(), TargetResult::NoMatch);
    }

    /// In this test an AnyOf type has two AllOf elements, one matches one not
    /// According to 7.7 Table 2 the target's result shall be Match
    #[test]
    fn test_target_type_match_request_contradicting_allof_elements() {
        let target = create_target_5();
        assert_eq!(target.match_request(&create_request()).unwrap(), TargetResult::Match);
    }



    fn create_target_1() -> TargetType {
        TargetTypeBuilder::default()
            .any_of(vec![AnyOfTypeBuilder::default()
                .all_of(vec![AllOfTypeBuilder::default()
                        ._match(vec![MatchTypeBuilder::default()
                            .attribute_value(AttributeValueTypeBuilder::default()
                                .data_type(DataType::String)
                                .value(Value::String("vehicle_cam_acceptance".to_string()))
                                .build().unwrap()) //AttributeValueTypeBuilder
                            .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                .attribute_id("request_context")
                                .data_type(DataType::String)
                                .category(Categories::Resource)
                                .must_be_present(true)
                                .build().unwrap()) //AttributeDesignatorTypeBuilder    
                            .match_id(FunctionId::StringEqual)
                            .build().unwrap()]) // MatchTypeBuilder
                    .build().unwrap()])// AllOfTypeBuilder
                .build().unwrap()])  // AnyOfTypeBuilder
            .build().unwrap() //TargetTypeBuilder
    }

    fn create_target_2() -> TargetType {
        TargetTypeBuilder::default()
            .any_of(vec![AnyOfTypeBuilder::default()
                .all_of(vec![AllOfTypeBuilder::default()
                        ._match(vec![
                            MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::String)
                                    .value(Value::String("vehicle_cam_acceptance".to_string()))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("request_context")
                                    .data_type(DataType::String)
                                    .category(Categories::Resource)
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::StringEqual)
                                .build().unwrap(), // MatchTypeBuilder
                            MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::Double)
                                    .value(Value::Double(0.2.into()))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("object.belief")
                                    .data_type(DataType::Double)
                                    .category(Categories::Action)
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::DoubleEqual)
                                .build().unwrap(), // MatchTypeBuilder
                            ]) 
                    .build().unwrap()])// AllOfTypeBuilder
                .build().unwrap()])  // AnyOfTypeBuilder
            .build().unwrap() //TargetTypeBuilder
    }

    fn create_target_3() -> TargetType {
        TargetTypeBuilder::default()
            .any_of(vec![AnyOfTypeBuilder::default()
                .all_of(vec![AllOfTypeBuilder::default()
                        ._match(vec![
                            MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::String)
                                    .value(Value::String("vehicle_cam_acceptance".to_string()))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("request_context")
                                    .data_type(DataType::String)
                                    .category(Categories::Resource)
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::StringEqual)
                                .build().unwrap(), // MatchTypeBuilder
                            MatchTypeBuilder::default()
                                .attribute_value(AttributeValueTypeBuilder::default()
                                    .data_type(DataType::Double)
                                    .value(Value::Double(0.2.into()))
                                    .build().unwrap()) //AttributeValueTypeBuilder
                                .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                    .attribute_id("object.belief")
                                    .data_type(DataType::Double)
                                    .category(Categories::Action)
                                    .must_be_present(true)
                                    .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                .match_id(FunctionId::StringEqual)
                                .build().unwrap(), // MatchTypeBuilder
                            ]) 
                    .build().unwrap()])// AllOfTypeBuilder
                .build().unwrap()])  // AnyOfTypeBuilder
            .build().unwrap() //TargetTypeBuilder
    }

    fn create_target_4() -> TargetType {
        TargetTypeBuilder::default()
            .any_of(vec![
                    AnyOfTypeBuilder::default()
                        .all_of(vec![AllOfTypeBuilder::default()
                                ._match(vec![
                                    MatchTypeBuilder::default()
                                        .attribute_value(AttributeValueTypeBuilder::default()
                                            .data_type(DataType::Double)
                                            .value(Value::Double(0.5.into()))
                                            .build().unwrap()) //AttributeValueTypeBuilder
                                        .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id("object.belief")
                                            .data_type(DataType::Double)
                                            .category(Categories::Action)
                                            .must_be_present(true)
                                            .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                        .match_id(FunctionId::DoubleEqual)
                                        .build().unwrap(), // MatchTypeBuilder
                                    ]) 
                            .build().unwrap()])// AllOfTypeBuilder
                        .build().unwrap(),
                    AnyOfTypeBuilder::default()
                        .all_of(vec![AllOfTypeBuilder::default()
                                ._match(vec![
                                    MatchTypeBuilder::default()
                                        .attribute_value(AttributeValueTypeBuilder::default()
                                            .data_type(DataType::String)
                                            .value(Value::String("vehicle_cam_acceptance".to_string()))
                                            .build().unwrap()) //AttributeValueTypeBuilder
                                        .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id("request_context")
                                            .data_type(DataType::String)
                                            .category(Categories::Resource)
                                            .must_be_present(true)
                                            .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                        .match_id(FunctionId::StringEqual)
                                        .build().unwrap() // MatchTypeBuilder
                                    ]) 
                            .build().unwrap()])// AllOfTypeBuilder
                        .build().unwrap()
                ])  // AnyOfTypeBuilder
            .build().unwrap() //TargetTypeBuilder
    }

    fn create_target_5() -> TargetType {
        TargetTypeBuilder::default()
            .any_of(vec![
                    AnyOfTypeBuilder::default()
                        .all_of(vec![
                            AllOfTypeBuilder::default()
                                ._match(vec![
                                    MatchTypeBuilder::default()
                                        .attribute_value(AttributeValueTypeBuilder::default()
                                            .data_type(DataType::Double)
                                            .value(Value::Double(0.5.into()))
                                            .build().unwrap()) //AttributeValueTypeBuilder
                                        .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id("object.belief")
                                            .data_type(DataType::Double)
                                            .category(Categories::Action)
                                            .must_be_present(true)
                                            .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                        .match_id(FunctionId::DoubleEqual)
                                        .build().unwrap(), // MatchTypeBuilder
                                    ]) 
                                .build().unwrap(), // AllOfTypeBuilder
                            AllOfTypeBuilder::default()
                                ._match(vec![
                                    MatchTypeBuilder::default()
                                        .attribute_value(AttributeValueTypeBuilder::default()
                                            .data_type(DataType::String)
                                            .value(Value::String("vehicle_cam_acceptance".to_string()))
                                            .build().unwrap()) //AttributeValueTypeBuilder
                                        .attribute_designator(AttributeDesignatorTypeBuilder::default()
                                            .attribute_id("request_context")
                                            .data_type(DataType::String)
                                            .category(Categories::Resource)
                                            .must_be_present(true)
                                            .build().unwrap()) //AttributeDesignatorTypeBuilder    
                                        .match_id(FunctionId::StringEqual)
                                        .build().unwrap() // MatchTypeBuilder
                                    ]) 
                            .build().unwrap() // AllOfTypeBuilder
                            ]) // all_of vec
                        .build().unwrap()
                ])  // AnyOfTypeBuilder
            .build().unwrap() //TargetTypeBuilder
    }

    fn create_request() -> RequestType {
        RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![
                AttributesTypeBuilder::default()
                    .category(Categories::Action)
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
                    .build().unwrap(), // AttributeType 
                AttributesTypeBuilder::default()
                    .category(Categories::Resource)
                    .attribute(vec![
                        AttributeTypeBuilder::default()
                            .attribute_id("request_context")
                            .include_in_result(false)
                            .attribute_value(vec![
                                AttributeValueTypeBuilder::default()
                                    .data_type(DataType::String)
                                    .value(Value::String("vehicle_cam_acceptance".into()))
                                    .build().unwrap() // AttributeValue
                            ]) // vec attribute_value
                            .build().unwrap(), // AttributeType
                    ]) // Attributes vec
                    .build().unwrap()
            ]) //vec attributes
            .build().unwrap() // RequestType
}

}