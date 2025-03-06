use crate::utils::*;
use crate::xacml::structs::*;
use crate::pap::get_policy_from_context;


pub fn decide_request(request: RequestType, context: &str) -> Result<ResponseType, XacmlError> {
    let policy: PolicyType = get_policy_from_context(context)?;

    ResponseTypeBuilder::default().build().map_err(|e| XacmlError::new(XacmlErrorType::FormatError, format!("Error building response: {}", e)))
}