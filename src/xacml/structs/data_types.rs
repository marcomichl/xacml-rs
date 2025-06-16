use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DataType {
    String,
    Boolean,
    Integer,
    Double,
    Time,
    Date,
    DateTime,
    AnyURI,
    HexBinary,
    Base64Binary,
    DayTimeDuration,
    YearMonthDuration,
    X500Name,
    RFC822Name,
    IP,
    DNSName,
    XPathExpression,
}

const DATA_TYPE_MAPPING: &[(DataType, &str)] = &[
    (DataType::String, "http://www.w3.org/2001/XMLSchema#string"),
    (DataType::Boolean, "http://www.w3.org/2001/XMLSchema#boolean"),
    (DataType::Integer, "http://www.w3.org/2001/XMLSchema#integer"),
    (DataType::Double, "http://www.w3.org/2001/XMLSchema#double"),
    (DataType::Time, "http://www.w3.org/2001/XMLSchema#time"),
    (DataType::Date, "http://www.w3.org/2001/XMLSchema#date"),
    (DataType::DateTime, "http://www.w3.org/2001/XMLSchema#dateTime"),
    (DataType::AnyURI, "http://www.w3.org/2001/XMLSchema#anyURI"),
    (DataType::HexBinary, "http://www.w3.org/2001/XMLSchema#hexBinary"),
    (DataType::Base64Binary, "http://www.w3.org/2001/XMLSchema#base64Binary"),
    (DataType::DayTimeDuration, "http://www.w3.org/2001/XMLSchema#dayTimeDuration"),
    (DataType::YearMonthDuration, "http://www.w3.org/2001/XMLSchema#yearMonthDuration"),
    (DataType::X500Name, "urn:oasis:names:tc:xacml:1.0:data-type:x500Name"),
    (DataType::RFC822Name, "urn:oasis:names:tc:xacml:1.0:data-type:rfc822Name"),
    (DataType::IP, "urn:oasis:names:tc:xacml:2.0:data-type:ipAddress"),
    (DataType::DNSName, "urn:oasis:names:tc:xacml:2.0:data-type:dnsName"),
    (DataType::XPathExpression, "urn:oasis:names:tc:xacml:3.0:data-type:xpathExpression"),
];

impl DataType {
    pub fn to_xacml_id (&self) -> &str {
        DATA_TYPE_MAPPING.iter()
            .find(|(dt, _)| *dt == *self)
            .map(|(_, id)| *id)
            .unwrap()   // This should never happen because we have a complete mapping
    }
}

impl FromStr for DataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DATA_TYPE_MAPPING.iter()
            .find(|(_, id)| *id == s)
            .map(|(dt, _)| *dt)
            .ok_or(())
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_xacml_id())
    }
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        DataType::from_str(&s).map_err( |_| serde::de::Error::custom("Invalid data type?"))
    }
}

#[cfg(test)]
mod test_data_type{
    #[cfg(test)]
    use super::*;
    use serde::{Serialize, Deserialize};
    #[allow(unused_imports)]
    use quick_xml::{se::to_string, de::from_str};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData{
        #[serde(rename = "dataType")]
        data_type: DataType
    }

    #[test]
    fn test_data_type_string(){
        let test_data = TestData{data_type: DataType::String};
        let xml = r#"<TestData><dataType>http://www.w3.org/2001/XMLSchema#string</dataType></TestData>"#;
        assert_eq!(to_string(&test_data).unwrap(), xml);
        assert_eq!(from_str::<TestData>(xml).unwrap().data_type, DataType::String);
    }

    #[test]
    fn test_data_type_double(){
        let test_data = TestData{data_type: DataType::Double};
        let xml = r#"<TestData><dataType>http://www.w3.org/2001/XMLSchema#double</dataType></TestData>"#;
        assert_eq!(to_string(&test_data).unwrap(), xml);
        assert_eq!(from_str::<TestData>(xml).unwrap().data_type, DataType::Double);
    }
}