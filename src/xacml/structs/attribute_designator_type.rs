use super::*;


/// 5.29 AttributeDesignatorType definition
/// Used to retrieve a bag of attributes from the request context
/// The attribute id must match the id of an attribute in the request context
/// In case it is not contained, an error is raised according to the MustBePresent attribute
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeDesignatorType{
    #[serde(rename = "@AttributeId")]
    pub (super) attribute_id: String,       // More specific of URI type
    #[serde(rename = "@DataType")]
    pub (super) data_type: DataType,          // More specific of URI type
    #[serde(rename = "@Category")]
    pub (super) category: String,           // More specific of URI type
    #[serde(rename = "@MustBePresent")]
    pub (super) must_be_present: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub (super) issuer: Option<String>
}

impl AttributeDesignatorType {
    /// Evaluate the attribute designator
    pub fn evaluate(&self, request: &RequestType) -> Result<Value, XacmlError> {
        return Err(XacmlError::new(XacmlErrorType::NotImplemented, "AttributeDesignator evaluation not implemented".to_string()))
    }
}