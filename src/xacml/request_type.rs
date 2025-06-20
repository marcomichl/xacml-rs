use super::*;


///5.42 RequestType
/// Contains the request for a decision
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RequestType {
    #[serde(rename = "@ReturnPolicyIdList")]
    pub(super) return_policy_id_list: bool,
    #[serde(rename = "@CombinedDecision")]
    pub(super) combined_decision: bool,
    #[serde(rename = "RequestDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(super) request_defaults: Option<RequestDefaultsType>,
    #[serde(rename = "Attributes")]
    pub(super) attributes: Vec<AttributesType>,
    #[serde(rename = "MultiRequests", skip_serializing_if = "UnimplementedField::is_none")]
    #[builder(default = "UnimplementedField(None)")]
    pub(super) multi_requests: UnimplementedField // Is not yet implemented and optional, will fail if present
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct UnimplementedField(pub Option<String>);
impl<'de> Deserialize<'de> for UnimplementedField {
    fn deserialize<D>(deserializer: D) -> Result<UnimplementedField, D::Error>
    where
        D: Deserializer<'de>,
    {
        let x: Option<String> = Deserialize::deserialize(deserializer)?;
        if x.is_some() {
            Err(serde::de::Error::custom(format!("Field must not be present: {}", x.unwrap())))
        }
        else {
            Ok(Self(None))
        }
    
    }
}
impl UnimplementedField {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

#[cfg(test)]
mod request_type_test {
    use super::*;

    #[test]
    fn request_type_builder_test() {
        let request = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![])
            .build()
            .unwrap();
        assert_eq!(request.return_policy_id_list, false);
        assert_eq!(request.combined_decision, false);
        assert_eq!(request.attributes, vec![]);
    }

    #[test]
    fn request_type_serialization_test(){
        let request = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category(Categories::Resource)
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other("Test-ID".to_string()))
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(23))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        assert_eq!(quick_xml::se::to_string(&request).unwrap(), r#"<RequestType ReturnPolicyIdList="false" CombinedDecision="false"><Attributes Category="urn:oasis:names:tc:xacml:3.0:attribute-category:resource"><Attribute AttributeId="Test-ID" IncludeInResult="false"><AttributeValue DataType="http://www.w3.org/2001/XMLSchema#integer">23</AttributeValue></Attribute></Attributes></RequestType>"#)
    }

    #[test]
    fn request_type_deserialization_test() {
        let serialized_request = r#"<RequestType ReturnPolicyIdList="false" CombinedDecision="false"><Attributes Category="urn:oasis:names:tc:xacml:3.0:attribute-category:resource"><Attribute AttributeId="Test-ID" IncludeInResult="false"><AttributeValue DataType="http://www.w3.org/2001/XMLSchema#integer">23</AttributeValue></Attribute></Attributes></RequestType>"#;
        let request: RequestType = quick_xml::de::from_str(&serialized_request).unwrap();
        let built_request = RequestTypeBuilder::default()
            .return_policy_id_list(false)
            .combined_decision(false)
            .attributes(vec![AttributesTypeBuilder::default()
                .category(Categories::Resource)
                .attribute(vec![AttributeTypeBuilder::default()
                    .attribute_id(AttributeIdentifiers::Other("Test-ID".to_string()))
                    .include_in_result(false)
                    .attribute_value(vec![AttributeValueTypeBuilder::default()
                        .data_type(DataType::Integer)
                        .value(Value::Integer(23))
                        .build()
                        .unwrap()    
                        ])
                    .build()
                    .unwrap()
                ])
                .build().unwrap()
            ])
            .build()
            .unwrap();
        assert_eq!(request, built_request);
    }

    #[test]
    #[should_panic(expected = "Field must not be present: Test String")]
    fn request_type_deserialization_failed_multirequest_test () {
        let serialized_request = r#"<RequestType ReturnPolicyIdList="false" CombinedDecision="false"><Attributes Category="TestCategory"><Attribute AttributeId="Test-ID" IncludeInResult="false"><AttributeValue DataType="http://www.w3.org/2001/XMLSchema#integer">23</AttributeValue></Attribute></Attributes><MultiRequests>Test String</MultiRequests></RequestType>"#;
        let _request = quick_xml::de::from_str::<RequestType>(&serialized_request).unwrap();
    }

}