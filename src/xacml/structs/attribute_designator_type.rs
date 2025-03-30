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
    pub fn evaluate(&self, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
        let attributes = request.attributes.iter()
            .map(|attr| attr.get_attribute_value_by_designator(self))
            .collect::<Result<Vec<&AttributeValueType>, XacmlError>>()?;
        let values = attributes.iter()
            .flat_map(|attr| attr.get_value())
            .collect::<Vec<Value>>();
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