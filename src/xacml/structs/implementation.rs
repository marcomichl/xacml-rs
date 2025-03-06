use crate::utils::{XacmlError, XacmlErrorType};
use super::*;

use super::MatchType;


impl MatchType{
    fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
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
        if self.attribute_selector.is_some() {
            let attribute_selector = self.attribute_selector.as_ref().unwrap();
            for attributes in &request.attributes {
                let result = attributes.get_attribute_value_by_selector(attribute_selector);
                if result.is_ok() {
                    return Ok(result.unwrap() == attribute);
                }
            }
        }
        Err(XacmlError::new(XacmlErrorType::FormatError, "MatchType does not contain a valid attribute designator or selector".to_string()))
    }
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
