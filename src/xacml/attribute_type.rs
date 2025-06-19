use super::*;

/// 5.46 AttributeType
/// Contains a single attribute metadata and value
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeType {
    #[serde(rename = "@AttributeId")]
    pub attribute_id: String,       // Pre-defined URIs in the Annex B, but contain only commonly used; might be implemented as enum
    #[serde(rename = "@IncludeInResult", default = "default_false")]
    pub include_in_result: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub issuer: Option<String>,    
    #[serde(rename = "AttributeValue")]
    pub attribute_value: Vec<AttributeValueType>
}

fn default_false() -> bool {
    false
}   

impl AttributeType{
    pub (super) fn get_values_by_designator(&self, designator: &AttributeDesignatorType) -> Result<Vec<Value>, XacmlError> {
        self.get_attribute_values_by_designator(designator)?
            .iter()
            .map(|attribute_value| attribute_value.get_value())
            .collect()
    }

    pub (super) fn get_attribute_values_by_designator(&self, designator: &AttributeDesignatorType) -> Result<Vec<&AttributeValueType>, XacmlError> {
        if (self.attribute_id != designator.attribute_id || self.issuer != designator.issuer) {
            return Ok(vec![])
        }
        let attribute_values: Vec<&AttributeValueType> = self.attribute_value.iter()
            .filter(|attribute_value| attribute_value.match_data_type(&designator.data_type))
            .collect();
        return Ok(attribute_values)

    }

}

#[cfg(test)]
mod attribute_type_test {
    use super::*;

    #[test]
    fn get_attribute_values_by_designator_test() {
        let designator = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category(Categories::Resource)
            .data_type(DataType::Boolean)
            .must_be_present(true)
            .build().unwrap();
        let attribute = AttributeTypeBuilder::default()
            .attribute_id("Test-ID")
            .include_in_result(true)
            .attribute_value(vec![AttributeValueTypeBuilder::default()
                .data_type(DataType::Boolean)
                .value(Value::Boolean(true))
                .build().unwrap()
                ])
            .build().unwrap();
        assert_eq!(attribute.get_values_by_designator(&designator).unwrap(), vec![Value::Boolean(true)]);
        assert_eq!(attribute.get_attribute_values_by_designator(&designator).unwrap(), vec![&AttributeValueTypeBuilder::default()
            .data_type(DataType::Boolean)
            .value(Value::Boolean(true))
            .build().unwrap()
            ]);
        assert_ne!(attribute.get_values_by_designator(&designator).unwrap(), vec![Value::Boolean(false)]);
        assert_ne!(attribute.get_values_by_designator(&designator).unwrap(), vec![Value::Integer(23)]);
        assert_ne!(attribute.get_attribute_values_by_designator(&designator).unwrap(), vec![&AttributeValueTypeBuilder::default()
            .data_type(DataType::String)
            .value(Value::Boolean(true))
            .build().unwrap()
            ]);
    }

    #[test]
    fn get_attribute_values_by_designator_empty_test() {
        let designator1 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Not Test-ID")
            .category(Categories::Resource)
            .data_type(DataType::Boolean)
            .must_be_present(true)
            .build().unwrap();
        let designator2 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category(Categories::Resource)
            .issuer("test_issuer")
            .data_type(DataType::Boolean)
            .must_be_present(true)
            .build().unwrap();
        let designator3 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category(Categories::Resource)
            .data_type(DataType::Integer)
            .must_be_present(true)
            .build().unwrap();
        let attribute = AttributeTypeBuilder::default()
            .attribute_id("Test-ID")
            .include_in_result(true)
            .attribute_value(vec![AttributeValueTypeBuilder::default()
                .data_type(DataType::Boolean)
                .value(Value::Boolean(true))
                .build().unwrap()
                ])
            .build().unwrap();
        let attribute2 = AttributeTypeBuilder::default()
            .attribute_id("Test-ID")
            .include_in_result(true)
            .issuer("test_issuer")
            .attribute_value(vec![AttributeValueTypeBuilder::default()
                .data_type(DataType::Boolean)
                .value(Value::Boolean(true))
                .build().unwrap()
                ])
            .build().unwrap();
        assert_eq!(attribute.get_values_by_designator(&designator1).unwrap(), vec![]);
        assert_eq!(attribute.get_values_by_designator(&designator2).unwrap(), vec![]);
        assert_eq!(attribute.get_values_by_designator(&designator3).unwrap(), vec![]);
        assert_eq!(attribute2.get_values_by_designator(&designator1).unwrap(), vec![]);
        assert_eq!(attribute2.get_values_by_designator(&designator2).unwrap(), vec![Value::Boolean(true)]);
        assert_eq!(attribute2.get_values_by_designator(&designator3).unwrap(), vec![]);
    }


}