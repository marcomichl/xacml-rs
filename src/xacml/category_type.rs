use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Categories {
    Resource,
    Action,
    Environment,
    Other(String)
}

const CATEGORY_MAPPING : &[(Categories, &str)] = &[
    (Categories::Resource, "urn:oasis:names:tc:xacml:3.0:attribute-category:resource"),
    (Categories::Action, "urn:oasis:names:tc:xacml:3.0:attribute-category:action"),
    (Categories::Environment, "urn:oasis:names:tc:xacml:3.0:attribute-category:environment"),
];

impl FromStr for Categories {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        CATEGORY_MAPPING
            .iter()
            .find(|(_, v)| *v == s)
            .map(|(k, _)| k.clone())
            .or_else(|| Some(Categories::Other(s.to_string())))
            .ok_or(())
    }
}

impl Categories {
    pub fn to_xacml_id(&self) -> &str {
        CATEGORY_MAPPING
            .iter()
            .find(|(k, _)| *k == *self)
            .map(|(_, v)| *v)
            .or_else(|| match self {
                Categories::Other(s) => Some(s),
                _ => Some("")
            })
            .unwrap()
    }
}

impl Serialize for Categories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_xacml_id())
    }
}

impl<'de> Deserialize<'de> for Categories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Categories::from_str(&s).map_err( |_| serde::de::Error::custom("Invalid category?"))
    }
}

impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


mod category_type_test {
    use super::*;

    #[test]
    fn test_category_serialize() {
        let categories = create_categories_vec();
        let id_str = categories.iter()
            .map(|c| c.to_xacml_id())
            .collect::<Vec<&str>>();
        assert_eq!(id_str, create_id_vec());

    }

    #[test]
    fn test_category_deserialization() {
        let id_str = create_id_vec();
        let categories = id_str.iter()
            .map(|s| Categories::from_str(s).unwrap())
            .collect::<Vec<Categories>>();
        assert_eq!(categories, create_categories_vec());
    }

    fn create_categories_vec() -> Vec<Categories> {
        vec![
            Categories::Resource,
            Categories::Action,
            Categories::Environment,
            Categories::Other("Other Category".to_string())
            ]
    }

    fn create_id_vec() -> Vec<String> {
        vec![
            "urn:oasis:names:tc:xacml:3.0:attribute-category:resource".to_string(),
            "urn:oasis:names:tc:xacml:3.0:attribute-category:action".to_string(),
            "urn:oasis:names:tc:xacml:3.0:attribute-category:environment".to_string(),
            "Other Category".to_string()
            ]
    }

}
/*
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:dns-name"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:authn-locality:ip-address"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:authentication-method"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:authentication-time"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:key-info"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:request-time"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:session-start-time"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:subject-id"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject:subject-id-qualifier"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject-category:access-subject"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject-category:codebase"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject-category:intermediary-subject"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject-category:recipient-subject"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:subject-category:requesting-machine"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:resource:resource-location"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:resource:resource-id"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:resource:simple-file-name"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:action:action-id"),
    (Identifiers::, "urn:oasis:names:tc:xacml:1.0:action:implied-action"), */
