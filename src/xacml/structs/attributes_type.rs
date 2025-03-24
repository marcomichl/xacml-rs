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

impl AttributesType{
    pub fn get_attribute_value_by_designator(&self, designator: &AttributeDesignatorType) -> Result<&AttributeValueType, XacmlError> {
        if self.category != designator.category {
            return Err(XacmlError::new(XacmlErrorType::FormatError, "AttributeDesignator does not match the category of the request".to_string()));
        }
        let attribute_values = self.attribute.as_ref()
            .ok_or(XacmlError::new(XacmlErrorType::FormatError, "No attributes in the request".to_string()))?
                .iter()
                .filter(|attr| attr.attribute_id == designator.attribute_id)
                .filter(|attr| attr.issuer.as_ref() == designator.issuer.as_ref())
                .collect::<Vec<&AttributeType>>();

        for attribute in attribute_values {
            for value in &attribute.attribute_value {
                if value.data_type == designator.data_type {
                    return Ok(value);
                }
            }
        }
        
        Err(XacmlError::new(XacmlErrorType::FormatError, "AttributeDesignator does not match any attribute in the request".to_string()))
    }

    pub fn get_attribute_value_by_selector(&self, selector: &AttributeSelectorType) -> Result<&AttributeValueType, XacmlError> {
        Err(XacmlError::new(XacmlErrorType::NotImplemented, "AttributeSelector not implemented as Content is not implemented".to_string()))
    }
}
