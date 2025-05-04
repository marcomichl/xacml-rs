mod test_types;
mod attributes_type;
mod match_type;
mod target_type;
mod any_of_type;
mod all_of_type;
mod policy_type;
mod response_type;
mod result_type;
mod status_type;
mod status_code_type;
mod status_message_type;
mod request_type;
mod attribute_type;
mod rule_type;
mod condition_type;
mod expression_type;
mod apply_type;
mod function_type;
mod function_implementation;
mod attribute_value_type;
mod attribute_designator_type;
mod decision_type;
mod rule_result;
mod target_results;

use core::str;
use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use derive_builder::Builder;

use crate::utils::*;

pub use any_of_type::*;
pub use match_type::*;
pub use attributes_type::*;
pub use target_type::*;
pub use all_of_type::*;
pub use policy_type::*; 
pub use response_type::*;
pub use result_type::*;
pub use status_type::*;
pub use status_code_type::*;
pub use status_message_type::*;
pub use request_type::*;
pub use attribute_type::*;
pub use rule_type::*;
pub use condition_type::*;
pub use expression_type::*;
pub use apply_type::*;
pub use function_type::*;
pub use attribute_value_type::*;
pub use attribute_designator_type::*;   
pub use decision_type::*;
pub use rule_result::*;
pub use target_results::*;

use super::enums::{combining_algorithms::{PolicyCombiningAlgorithms, RuleCombiningAlgorithms}, data_types::DataType, *};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
#[serde(rename = "PolicySet")]
pub struct PolicySetType {
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns: Option<String>,
    #[serde(rename = "@xmlns:xsi", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns_xsi: Option<String>,
    #[serde(rename = "@PolicySetId")]
    policy_set_id: String,
    #[serde(rename = "@Version")]
    version: VersionType,
    #[serde(rename = "@PolicyCombiningAlgId")]
    policy_combining_alg_id: PolicyCombiningAlgorithms, // Combining algorithm, as of now string, might later be an enum
    #[serde(rename = "@MaxDelegationDepth", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    max_delegation_depth: Option<i32>,
    // 5.2 Description element
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,        // checked data type
    // 5.3 PolicyIssuer element
    #[serde(rename = "PolicyIssuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_issuer: Option<PolicyIssuerType>,
    // 5.4 PolicyDefaults element
    #[serde(rename = "PolicySetDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_set_defaults: Option<DefaultsType>,
    // 5.6 Target element defining the applicability of the policyset, policy or (optional) a rule
    #[serde(rename = "Target")]
    target: TargetType,
    #[serde(rename = "PolicySet", skip_serializing_if = "Option::is_none")]
    #[builder(default)]  // Might be any number (including 0) -> might be necessary to make it an option, so that complete missing fields are accepted (not only empty vecs)
    policy_set: Option<Vec<PolicySetType>>,
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]
    #[builder(default)]  // Might be any number (including 0)
    policy: Option<Vec<PolicyType>>,
    // 5.10 PolicySetIdReference element
    #[serde(rename = "PolicySetIdReference", skip_serializing_if = "Option::is_none")]
    #[builder(default)]  // Reference to a policy that must be included in the PolicySet
    policy_set_id_reference: Option<Vec<IdReferenceType>>,
    // 5.11 PolicyIdReference element
    #[serde(rename = "PolicyIdReference", skip_serializing_if = "Option::is_none")]
    #[builder(default)]  // Reference to a policy that must be included in the Policy
    policy_id_reference: Option<Vec<IdReferenceType>>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligation_expressions: Option<Vec<ObligationExpressionsType>>,
    #[serde(rename = "AdvideExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    advice_expressions: Option<Vec<AdviceExpressionsType>>,
    #[serde(rename = "CombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    combiner_parameters: Option<Vec<CombinerParametersType>>,
    #[serde(rename = "PolicyCombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_combiner_parameters: Option<Vec<PolicySetCombinerParametersType>>,
    #[serde(rename = "PolicySetCombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    poicy_set_combiner_parameters: Option<Vec<PolicySetCombinerParametersType>>
}

/// 5.3 PolicyIssuerType
/// Type describing the issuer of a Policy or Policy set
/// Defined in XACMLAdmin specification, if the PDP does not implement this, it should raise an error if it is contained
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicyIssuerType {
    // Free form XML content
    #[serde(rename = "Content")]
    content: String,        // Type 5.45
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute: Option<Vec<String>>    // Type 5.46  
}







/// 5.10 PolicySetIdReferenceType
/// Reference a PolicySet by the ID
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicySetIdReferenceType {
    #[serde(flatten)]
    id: IdReferenceType
}

/// IDReferenceType used in PolicySetIdReferenceType and PolicyIdReferenceType
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct IdReferenceType {
    #[serde(rename = "@Version", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    version: Option<VersionMatchType>,
    #[serde(rename = "@EarliestVersion", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    earliest_version: Option<VersionMatchType>,                // Earliest version
    #[serde(rename = "@LatestVersion", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    latest_version: Option<VersionMatchType>,                  // Latest version
    #[serde(rename = "$value")]
    id: String                               // More specific an URI, IdReferenceType extends URI
}

/// 5.11 PolicyIdReferenceType
/// Reference a Policy by the ID
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicyIdReferenceType {
    #[serde(flatten)]
    id: IdReferenceType
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct DefaultsType {
    x_path_version: Option<Vec<String>>        // more specific URI type
}

/// 5.13 VersionMatchType
/// Type for the version attribute of the policy set or policy
/// Consists of a string, that is restricted to numbers separated by dots
/// The two wildcards * for any and + for any higher version are allowed
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct VersionMatchType{
    version: String
}                           

/// 5.12 VersionType
/// Type for the version attribute of the policy set or policy
/// Consists of a string, that is restricted to numbers separated by dots
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct VersionType (pub String); //Newtype wrapper as this is a simple string restricted to numbers separated by dots, should be implemented later

/// 5.16 CombinerParameterType
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct CombinerParametersType {
    #[serde(rename = "ParameterName")]
    combiner_parameters: Vec<CombinerParameterType>
}

/// 5.17 CombinerParameter
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct CombinerParameterType {
    #[serde(rename = "ParameterName")]
    parameter_name: String,
    #[serde(rename = "ParameterValue")]
    parameter_value: String
}


/// 5.18 RuleCombinerParametersType
/// Extends the CombinerParametersType with a rule id that must reference a rule in the policy
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RuleCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "RuleIdRef")]
    rule_id_ref: String
}

/// 5.19 PolicyCombinerParametersType
/// Extends the CombinerParametersType with a policy id that must reference a policy in the policy set
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicyCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "PolicyIdRef")]
    policy_id_ref: String           // More specific of URI type
}

/// 5.20 PolicySetCombinerParametersType
/// Extends the CombinerParametersType with a policy set id that must reference a policy set in the policy set
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicySetCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "PolicySetIdRef")]
    policy_set_id_ref: String           // More specific of URI type
}




/// 5.22 EffectType
/// Enumeration to indicate the effect of a rule
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum EffectType {
    Permit,
    Deny
}

/// 5.23 VariableDefinitionType
/// Defines a variable in a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct VariableDefinitionType {
    #[serde(rename = "Expression")]
    expression: Vec<ExpressionType>,
    #[serde(rename = "VariableId")]
    variable_id: String
}

/// 5.24 VariableReferenceType
/// References a variable in a policy
/// Is allowed anywhere in the policy where an ExpressionType is allowed
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct VariableReferenceType {
    #[serde(rename = "VariableId")]
    variable_id: String
}


/// 5.30 AttributeSelectorType definition
/// Used to retrieve a bag of unnamed and uncategorized attributes from the request context
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AttributeSelectorType {
    #[serde(rename = "@Category")]
    category: String,           // More specific of URI type
    #[serde(rename = "@ContextSelectorId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    context_selector_id: Option<String>,       // More specific of URI type
    #[serde(rename = "@Path")]
    path: String,               
    #[serde(rename = "@DataType")]
    data_type: String,          // More specific of URI type
    #[serde(rename = "@MustBePresent")]
    must_be_present: bool
}



#[derive(Serialize, Deserialize, PartialOrd, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Double(f64),
    String(String),
    Date(String),
    Time(String),
    DateTime(String),
    AnyURI(String),
    Indeterminate,
}

impl Eq for Value {}

/// Custom implementation of PartialEq so that f64 can be matched (interpreted bitwise)
impl PartialEq for Value{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Boolean(val) => {
                if let Value::Boolean(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::Integer(val) => {
                if let Value::Integer(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::Double(val) => {
                if let Value::Double(other_val) = other {
                    val.to_bits() == other_val.to_bits()
                } else {
                    false
                }
            }
            Value::String(val) => {
                if let Value::String(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::Date(val) => {
                if let Value::Date(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::Time(val) => {
                if let Value::Time(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::DateTime(val) => {
                if let Value::DateTime(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::AnyURI(val) => {
                if let Value::AnyURI(other_val) = other {
                    val == other_val
                } else {
                    false
                }
            }
            Value::Indeterminate => matches!(other, Value::Indeterminate),
        }
    }
}

/// 5.32 OblicationsType definition
/// Contains a set of oblication elements
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ObligationsType {
    #[serde(rename = "Obligation")]
    obligations: Vec<ObligationType>
}

/// 5.33 AssociatedAdviceType definition
/// Contains a set of advice elements
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AssociatedAdviceType {
    #[serde(rename = "Advice")]
    advice: Vec<AdviceType>
}

/// 5.34 ObligationType definition
/// Contains an obligation element
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ObligationType {
    #[serde(rename = "@ObligationId")]
    obligation_id: String,      // More specific of URI type
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}

/// 5.35 AdviceType definition
/// Containts an identifier for the advice and a set of attributes as supplemantal information
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AdviceType {
    #[serde(rename = "@AdviceId")]
    advice_id: String,          // More specific of URI type
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}

/// 5.36 AttributeAssignmentType definition
/// Used to include arguments in obligations and advices
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
#[serde(rename = "AttributeAssignment")]
pub struct AttributeAssignmentType {
    // TODO: This incorporates the AttributeValueType, but is not yet implemented
    //#[serde(flatten)]
    //value: AttributeValueType,
    #[serde(rename = "$value")]
    attribute: String,
    #[serde(rename = "@AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "@Category", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    category: Option<String>,          // More specific of URI type
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    issuer: Option<String>
}

/// 5.37 ObligationExpressionsType definition
/// Contains a set of obligation expressions
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ObligationExpressionsType {
    #[serde(rename = "ObligationExpression")]
    obligation_expressions: Vec<ObligationExpressionType>
}

/// 5.38 AdviceExpressionsType definition
/// Contains a set of advice expressions
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AdviceExpressionsType {
    #[serde(rename = "AdviceExpression")]
    advice_expressions: Vec<AdviceExpressionType>
}

/// 5.39 ObligationExpressionType definition
/// Element that evaluates to an obligation
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ObligationExpressionType {
    #[serde(rename = "@ObligationId")]
    obligation_id: String,      // More specific of URI type
    #[serde(rename = "@FulfillOn")]
    fulfill_on: EffectType,
    #[serde(rename = "AttributeAssignmentExpression", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_assignment: Option<Vec<AttributeAssignmentExpressionType>>
}


/// 5.40 AdviceExpressionType definition
/// Element that evaluates to an advice
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AdviceExpressionType {
    #[serde(rename = "@AdviceId")]
    advice_id: String,          // More specific of URI type
    #[serde(rename = "@AppliesTo")]
    applies_to: EffectType,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_assignment: Option<Vec<AttributeAssignmentExpressionType>>
}

/// 5.41 AttributeAssignmentExpressionType definition
/// Used to include arguments in obligations and advices
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
#[serde(rename = "AttributeAssignmentExpression")]
pub struct AttributeAssignmentExpressionType {
    #[serde(rename = "@AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "@Category", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    category: Option<String>,          // More specific of URI type
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    issuer: Option<String>,
    #[serde(rename = "$value")]
    expression: Vec<ExpressionType>
}



/// 5.43 RequestDefaultsType
/// Contains the XPath Version for the request
/// Optional
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RequestDefaultsType {
    #[serde(rename = "XPathVersion")]
    x_path_version: String,     // More specific of URI type
}



/// 5.45 ContentType
/// Placeholder for additional attributes, typically content of the resource
/// Optional, not that reasoned implemented
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct Content {
    //#[serde(rename = "$value")]
    // TODO: Did not yet find a way to deserialize arbitratory XML content
    #[serde(skip_deserializing)]
    any: String        // Any XML content
}





/// 5.49 PolicyIdentifierListType
/// Contains a list of policy identifiers
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct PolicyIdentifierListType {
    #[serde(rename = "PolicyIdReference", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_id_reference: Option<Vec<PolicyIdReferenceType>>,
    #[serde(rename = "PolicySetIdReference", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_set_id_reference: Option<Vec<PolicySetIdReferenceType>>
}

/// 5.57 StatusDetailType
/// Contains the status detail of a decision request
/// Optional, therefore not that reasoned implemented
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct StatusDetailType {
    #[serde(rename = "Any")]
    any: String
}

/// 5.58 MissingAttributeDetailType
/// Contains the status detail of a missing attribute
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct MissingAttributeDetailType {
    #[serde(rename = "@Category")]
    category: String,           // More specific of URI type
    #[serde(rename = "@AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "@DataType")]
    data_type: DataType,          // Not direcly specified as DataTypeEnum?
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    issuer: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_value: Option<Vec<AttributeValueType>>        
}