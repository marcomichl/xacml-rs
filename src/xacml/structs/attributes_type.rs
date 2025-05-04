use std::sync::Arc;

use super::*;

/// 5.44 AttributesType
/// Contains a set of attributes
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributesType {
    #[serde(rename = "@Category")]
    category: String,           //Specifies for what type of entity this attributes are defined
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xml_id: Option<String>,     // Unique identifier for the attributes
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    content: Option<Content>,        // Type 5.45, defined as sequence with 0 or 1 occurance
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute: Option<Vec<AttributeType>>    // Type 5.46, defined as sequence with ANY number
}

// todo: adapt functions to allow returning of multiple values; do not fail if the attributes do not match
impl AttributesType{


    pub fn get_attribute_values_by_designator(&self, designator: &AttributeDesignatorType) -> Result<Vec<&AttributeValueType>, XacmlError> {
        if self.category != designator.category || self.attribute.is_none() {
            return Ok(vec![])       // Category of whole AttributesType does not match
        }
        // Verify attribute-field, AttributeType::attribute_id and issuer
        let attribute_values: Vec<&AttributeValueType> = self.attribute.as_ref()
            .ok_or(XacmlError::new(XacmlErrorType::FormatError, "No attributes in the request, should not happen!".to_string()))?
                // if an error is thrown here, something went wrong as this was tested above
                .iter()
                .flat_map(|attribute| attribute.get_attribute_values_by_designator(designator))
                .flatten()
                .collect();
        Ok(attribute_values)
    }

    pub fn get_values_by_designator(&self, designator: &AttributeDesignatorType) -> Result<Vec<Value>, XacmlError> {
        self.get_attribute_values_by_designator(designator)?
            .iter()
            .map(|attribute_value| attribute_value.get_value())
            .collect()
    }

    pub fn get_attribute_values_by_selector(&self, selector: &AttributeSelectorType) -> Result<Vec<&AttributeValueType>, XacmlError> {
        Err(XacmlError::new(XacmlErrorType::NotImplemented, "AttributeSelector not implemented as Content is not implemented".to_string()))
    }
}

#[cfg(test)]
mod attributes_type_test{
    use super::*;

    #[test]
    fn get_value_by_selector_test() {
        let designator1 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category("Test-Category")
            .data_type(DataType::Boolean)
            .must_be_present(true)
            .build().unwrap();
        let designator2 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category("Test-Category")
            .data_type(DataType::Integer)
            .must_be_present(true)
            .build().unwrap();
        let designator3 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category("Test-Category")
            .data_type(DataType::Double)
            .must_be_present(true)
            .build().unwrap();
        let designator4 = AttributeDesignatorTypeBuilder::default()
            .attribute_id("Test-ID")
            .category("Test-Category")
            .issuer("Test-Issuer")
            .data_type(DataType::Double)
            .must_be_present(true)
            .build().unwrap();
        let attribute = AttributeTypeBuilder::default()
            .attribute_id("Test-ID")
            .include_in_result(true)
            .attribute_value(vec![AttributeValueTypeBuilder::default()
                .data_type(DataType::Boolean)
                .value(Value::Boolean(true))
                .build().unwrap(), 
                AttributeValueTypeBuilder::default()
                .data_type(DataType::Integer)
                .value(Value::Integer(23))
                .build().unwrap()
                ])
            .build().unwrap();
        let attributes = AttributesTypeBuilder::default()
            .category("Test-Category")
            .attribute(vec![attribute])
            .build().unwrap();
        assert_eq!(attributes.get_values_by_designator(&designator1).unwrap(), vec![Value::Boolean(true)]);
        assert_eq!(attributes.get_values_by_designator(&designator2).unwrap(), vec![Value::Integer(23)]);
        assert_eq!(attributes.get_values_by_designator(&designator3).unwrap(), vec![]);
        assert_eq!(attributes.get_values_by_designator(&designator4).unwrap(), vec![]);

    }
}