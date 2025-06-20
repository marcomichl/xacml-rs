use super::*;


/// 5.29 AttributeDesignatorType definition
/// Used to retrieve a bag of attributes from the request context
/// The attribute id must match the id of an attribute in the request context
/// In case it is not contained, an error is raised according to the MustBePresent attribute
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeDesignatorType{
    #[serde(rename = "@AttributeId")]
    pub (super) attribute_id: AttributeIdentifiers,       // More specific of URI type
    #[serde(rename = "@DataType")]
    pub (super) data_type: DataType,          // More specific of URI type
    #[serde(rename = "@Category")]
    pub (super) category: Categories,           // More specific of URI type
    #[serde(rename = "@MustBePresent")]
    pub (super) must_be_present: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub (super) issuer: Option<String>
}

impl AttributeDesignatorType {
    /// Evaluate the attribute designator
    pub fn evaluate(&self, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
        let values: Vec<Value> = request.attributes.iter()
            .flat_map(|attr| attr.get_values_by_designator(self))
            .flatten()
            .collect();
        if values.len() > 0 {
            return Ok(values);
        }
        // This code is reached of there are no matching attributes
        if self.must_be_present {
            return Ok(vec![Value::Indeterminate]);
        }
        else {
            return Ok(Vec::new());
        }
    }
}

#[cfg(test)]
mod attribute_designator_type_test {
    use super::*;

    #[test]
    fn attribute_designator_type_evaluate_test() {
        let request = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category(Categories::Resource)
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other("Test-ID".to_string()))
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
        let designator = AttributeDesignatorTypeBuilder::default()
            .attribute_id(AttributeIdentifiers::Other("Test-ID".to_string()))
            .data_type(DataType::Integer)
            .category(Categories::Resource)
            .must_be_present(false)
            .build().unwrap();
        let result = designator.evaluate(&request).unwrap();
        println!("{:?}", result);
    }

}