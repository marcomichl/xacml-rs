use crate::utils::*;
use crate::xacml::structs::*;
use crate::pap::get_policy_from_context;


pub fn decide_request(request: RequestType, context: &str) -> Result<ResponseType, XacmlError> {
    let policy: PolicyType = get_policy_from_context(context)?;
    // verify that the policy is applicable
    if !policy.match_request(&request)? {
        return ResponseTypeBuilder::default()
            .result(
                vec![ResultTypeBuilder::default()
                    .decision(DecisionType::NotApplicable)
                    .status(StatusTypeBuilder::default()
                        .status_message(StatusMessageType::new("Policy is not applicable".to_string()))
                        .build()
                        .unwrap())
                    .build()
                    .unwrap()
                ]
            )
            .build()
            .map_err(|e| XacmlError::new(XacmlErrorType::FormatError, format!("Error building response: {}", e)));
    }
    // First verify that the target of the policy matches the request
    

    ResponseTypeBuilder::default().build().map_err(|e| XacmlError::new(XacmlErrorType::FormatError, format!("Error building response: {}", e)))
}