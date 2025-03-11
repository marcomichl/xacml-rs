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
            for attributes in &request.attributes {
                let result = attributes.get_attribute_value_by_designator(attribute_designator);
                if result.is_ok() {
                    return Ok(result.unwrap() == attribute);
                }
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