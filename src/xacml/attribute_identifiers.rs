use std::{borrow::Cow, fmt, str::FromStr};
use super::*;


/// This enum contains the standardized identifiers used for attributes
/// see XACML 3.0 10.2.6 and B.2
/// Ass these Identifiers are standardized but not the use is not restricted to them,
/// a general Identifier holding arbitraty IDs is provided.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeIdentifiers{
    DnsName,
    IpAddress,
    AuthenticationMethod,
    AuthenticationTime,
    KeyInfo,
    RequestTime,
    SessionStartTime,
    SubjectId,
    SubjectIdQualifier,
    ResourceLocation,
    ResourceId,
    ResourceSimpleFileName,
    ActionId,
    ImpliedAction,
    // Custom IDs here..
    Other(String),
    TrusteeBelief
}

fn create_mapping() -> Vec<(AttributeIdentifiers, String)> {
    vec![
        (AttributeIdentifiers::DnsName, "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:dns-name".to_string()),
        (AttributeIdentifiers::IpAddress, "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:ip-address".to_string()),
        (AttributeIdentifiers::AuthenticationMethod, "urn:oasis:names:tc:xacml:1.0:subject:authentication-method".to_string()),
        (AttributeIdentifiers::AuthenticationTime, "urn:oasis:names:tc:xacml:1.0:subject:authentication-time".to_string()),
        (AttributeIdentifiers::KeyInfo, "urn:oasis:names:tc:xacml:1.0:subject:key-info".to_string()),
        (AttributeIdentifiers::RequestTime, "urn:oasis:names:tc:xacml:1.0:subject:request-time".to_string()),
        (AttributeIdentifiers::SessionStartTime, "urn:oasis:names:tc:xacml:1.0:subject:session-start-time".to_string()),
        (AttributeIdentifiers::SubjectId, "urn:oasis:names:tc:xacml:1.0:subject:subject-id".to_string()),
        (AttributeIdentifiers::SubjectIdQualifier, "urn:oasis:names:tc:xacml:1.0:subject:subject-id-qualifier".to_string()),
        (AttributeIdentifiers::ResourceLocation, "urn:oasis:names:tc:xacml:1.0:resource:resource-location".to_string()),
        (AttributeIdentifiers::ResourceId, "urn:oasis:names:tc:xacml:1.0:resource:resource-id".to_string()),
        (AttributeIdentifiers::ResourceSimpleFileName, "urn:oasis:names:tc:xacml:1.0:resource:simple-file-name".to_string()),
        (AttributeIdentifiers::ActionId, "urn:oasis:names:tc:xacml:1.0:action:action-id".to_string()),
        (AttributeIdentifiers::ImpliedAction, "urn:oasis:names:tc:xacml:1.0:action:implied-action".to_string()),
        (AttributeIdentifiers::TrusteeBelief, format!("urn:{}:xacml:attribute:trustee:belief", URN_NAMESPACE))
    ]
}


impl FromStr for AttributeIdentifiers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        create_mapping()
            .iter()
            .find(|(_, v)| *v == s)
            .map(|(k, _)| k.clone())
            .or_else(|| Some(AttributeIdentifiers::Other(s.to_string())))
            .ok_or(())
    }
}

impl AttributeIdentifiers {
    pub fn to_xacml_id(&self) -> String {
    if let Some((_, value)) = create_mapping().into_iter().find(|(k, _)| *k == *self) {
        return value;
    }

    match self {
        AttributeIdentifiers::Other(s) => s.clone(),
        _ => "".to_string(), // fallback: return empty string if nothing matches
    }
    }
}

impl Serialize for AttributeIdentifiers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_xacml_id().as_str())
    }
}

impl<'de> Deserialize<'de> for AttributeIdentifiers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        AttributeIdentifiers::from_str(&s).map_err( |_| serde::de::Error::custom("Invalid category?"))
    }
}

impl fmt::Display for AttributeIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


mod attribute_identifiers_test { use quick_xml::de::from_str;

//TODO
    use super::*;
    use quick_xml::{de, se};

    #[test]
    fn test_attribute_identifier_serialize() {
        let categories = create_attribute_identifiers_vec();
        let id_str = categories.iter()
            .map(|c| c.to_xacml_id())
            .collect::<Vec<String>>();
        assert_eq!(id_str, create_id_vec());

    }

    #[test]
    fn test_attribute_identifier_deserialization() {
        let id_str = create_id_vec();
        let categories = id_str.iter()
            .map(|s| AttributeIdentifiers::from_str(s).unwrap())
            .collect::<Vec<AttributeIdentifiers>>();
        assert_eq!(categories, create_attribute_identifiers_vec());
    }

    #[test]
    fn test_attribute_identifier_xml_deserialization() {
        let deserialized_elements = create_id_vec().iter()
            .map(|s| de::from_str::<AttributeIdentifiers>(s).unwrap())
            .collect::<Vec<AttributeIdentifiers>>();
        assert_eq!(deserialized_elements, create_attribute_identifiers_vec());
    }

    #[test]
    fn test_attribute_identifier_xml_serialization() {
        let serialized_elements = create_attribute_identifiers_vec().iter()
            .map(|id| se::to_string_with_root("id", &Some(id)).unwrap())
            .collect::<Vec<String>>();
        assert_eq!(serialized_elements, create_id_vec().iter().map(|s| format!("<id>{}</id>", s)).collect::<Vec<String>>());
    }

    fn create_attribute_identifiers_vec() -> Vec<AttributeIdentifiers> {
        vec![
            AttributeIdentifiers::DnsName,
            AttributeIdentifiers::IpAddress,
            AttributeIdentifiers::AuthenticationMethod,
            AttributeIdentifiers::AuthenticationTime,
            AttributeIdentifiers::KeyInfo,
            AttributeIdentifiers::RequestTime,
            AttributeIdentifiers::SessionStartTime,
            AttributeIdentifiers::SubjectId,
            AttributeIdentifiers::SubjectIdQualifier,
            AttributeIdentifiers::ResourceLocation,
            AttributeIdentifiers::ResourceId,
            AttributeIdentifiers::ResourceSimpleFileName,
            AttributeIdentifiers::ActionId,
            AttributeIdentifiers::ImpliedAction,
            AttributeIdentifiers::Other("Other Identifier".to_string()),
            AttributeIdentifiers::TrusteeBelief
            ]
    }

    fn create_id_vec() -> Vec<String> {
        vec![
            "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:dns-name".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:ip-address".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:authentication-method".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:authentication-time".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:key-info".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:request-time".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:session-start-time".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:subject-id".to_string(),
            "urn:oasis:names:tc:xacml:1.0:subject:subject-id-qualifier".to_string(),
            "urn:oasis:names:tc:xacml:1.0:resource:resource-location".to_string(),
            "urn:oasis:names:tc:xacml:1.0:resource:resource-id".to_string(),
            "urn:oasis:names:tc:xacml:1.0:resource:simple-file-name".to_string(),
            "urn:oasis:names:tc:xacml:1.0:action:action-id".to_string(),
            "urn:oasis:names:tc:xacml:1.0:action:implied-action".to_string(),
            "Other Identifier".to_string(),
            format!("urn:{}:xacml:attribute:trustee:belief", URN_NAMESPACE)
            ]
    }

}