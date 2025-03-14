use crate::utils::XacmlError;

use super::*;

/// 5.47 ResponseType
/// Standard return type for a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ResponseType {
    #[serde(rename = "Result")]
    result: Vec<ResultType>
}

impl ResponseType {
    pub (crate) fn get_permit_response() -> Result<ResponseType, XacmlError> {
        Ok(ResponseTypeBuilder::default()
            .result(
                vec![ResultTypeBuilder::default()
                    .decision(DecisionType::Permit)
                    .build()?,
                ]
            )
            .build()?)
    }

    pub (crate) fn get_deny_response() -> Result<ResponseType, XacmlError> {
        Ok(ResponseTypeBuilder::default()
            .result(
                vec![ResultTypeBuilder::default()
                    .decision(DecisionType::Deny)
                    .build()?,
                ]
            )
            .build()?)
    }
}

