use super::*;

/// 5.31 AttributeValueType definition
/// Contains a literal attribute value
/// Is kind of a special case as the data type of the value is described by the DataType attribute
#[derive(Serialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeValueType {
    #[serde(rename = "@DataType")]
    pub (super) data_type: DataType,          // More specific of URI type
    #[serde(rename = "$value")]
    pub (super) value: Value
}


impl<'de> Deserialize<'de> for AttributeValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {       
        #[derive(Deserialize)]
        struct ValueHelper {
            #[serde(rename = "@DataType")]
            data_type: DataType,
            #[serde(rename = "$value")]
            value: String
        }
        /* 
        let str: String = Deserialize::deserialize(deserializer)?;
        print!("{}", str);
        Err(serde::de::Error::custom("DEBUG"))
        */ // /*
        let helper = ValueHelper::deserialize(deserializer).map_err (|_| serde::de::Error::custom("Helper deserialization failed"))?;

        match helper.data_type {
            DataType::String => Ok(AttributeValueType{data_type: helper.data_type, value: Value::String(helper.value)}),
            DataType::Boolean => Ok(AttributeValueType{data_type: helper.data_type, value: Value::Boolean(helper.value.parse().map_err( |_| serde::de::Error::custom("Invalid boolean"))?)}),
            DataType::Integer => Ok(AttributeValueType{data_type: helper.data_type, value: Value::Integer(helper.value.parse().map_err( |_| serde::de::Error::custom("Invalid integer"))?)}),
            DataType::AnyURI => Ok(AttributeValueType{data_type: helper.data_type, value: Value::String(helper.value)}),
            DataType::Double => Ok(AttributeValueType{data_type: helper.data_type, value: Value::Double(helper.value.parse().map_err( |_| serde::de::Error::custom("Invalid double"))?)}),
            _ => Err(serde::de::Error::custom("Unimplemented data type"))
        }
        //*/
    }
}

impl AttributeValueType {
    /// Evaluate the attribute value
    pub fn evaluate(&self, _request: &RequestType) -> Result<Vec<Value>, XacmlError> {
        Ok([self.value.clone()].to_vec())      // Could be relevant for a revision to change the type to a reference, but could be a problem with the lifetime of other return values (e.g. function results)
    }
}