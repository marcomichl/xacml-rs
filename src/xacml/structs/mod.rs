mod test_types;

use core::str;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use derive_builder::Builder;

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


/// Shall appear in the policy set or the policy elements, may be contained in a rule element
/// Shall contain a conjunctive sequence of <AnyOf> elements, to be applicable one of these has to match the decision request.
/// Each AnyOf element contains a disjunctive AllOf element, that all have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct TargetType {
    #[serde(rename = "AnyOf", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    any_of: Option<Vec<AnyOfType>>                  // Data type for elements, of which one must match the context to be applicable; if empty, the target is always applicable; might be changed to a simple vec, that can also be of length 0
}

/// 5.7 AnyOf element
/// Shall contain a disjunctive sequence of <AllOf> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AnyOfType {
    #[serde(rename = "AllOf")]
    all_of: Vec<AllOfType>                  // Data type for elements, of which all must match the context to be applicable; if empty, the anyOf is always applicable; might be changed to a simple vec, that can also be of length 0
}

/// 5.8 AllOf element
/// Shall contain a conjunctive sequence of <Match> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AllOfType {
    #[serde(rename = "Match")]
    _match: Vec<MatchType>                           // One or many, all of which must match the context to be applicable
}

/// 5.9 Match element
/// Shall contain a condition that must be fulfilled by the context to be applicable
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct MatchType {
    #[serde(rename = "@MatchId")]
    match_id: String,                        // More specific of URI type
    #[serde(rename = "AttributeValue")]
    attribute_value: AttributeValueType,
    #[serde(rename = "AttributeDesignator", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_designator: Option<AttributeDesignatorType>,   // Either this or the attributeSelector must be present, not both and not none
    #[serde(rename = "AttributeSelector", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute_selector: Option<AttributeSelectorType>
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

/// 5.14 Policy element
/// Describes a policy as smallest unit useable by a PDP
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
#[serde(rename = "Policy")]
pub struct PolicyType {
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns: Option<String>,
    #[serde(rename = "@xmlns:xsi", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns_xsi: Option<String>,
    #[serde(rename = "@PolicyId")]
    policy_id: String,                          // More specific of URI type
    #[serde(rename = "@Version")]
    version: VersionType,
    #[serde(rename = "@RuleCombiningAlgId")]
    rule_combining_alg_id: RuleCombiningAlgorithms,              // Combining algorithm, as of now string, might later be an enum
    #[serde(rename = "@MaxDelegationDepth", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    max_delegation_depth: Option<i32>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,                
    #[serde(rename = "PolicyIssuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_issuer: Option<PolicyIssuerType>,
    #[serde(rename = "PolicyDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_defaults: Option<DefaultsType>,
    #[serde(rename = "CombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    combiner_parameters: Option<Vec<CombinerParametersType>>,       // own type?
    #[serde(rename = "RuleCombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    rule_combiner_parameters: Option<Vec<String>>,  // own type?
    #[serde(rename = "Target")]
    target: Vec<TargetType>,
    #[serde(rename = "VariableDefinition", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    variable_definition: Option<Vec<String>>,       // own type?
    #[serde(rename = "Rule")]
    rule: Vec<RuleType>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligation_expressions: Option<Vec<ObligationExpressionsType>>,
    #[serde(rename = "AdvideExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    advice_expressions: Option<Vec<AdviceExpressionsType>>,
    
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
pub struct VersionType (String); //Newtype wrapper as this is a simple string restricted to numbers separated by dots, should be implemented later

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

/// 5.21 RuleType
/// Defines a rule in a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RuleType {
    #[serde(rename = "@RuleId")]
    rule_id: String,
    #[serde(rename = "@Effect")]
    effect: EffectType,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    target: Option<TargetType>,
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    condition: Option<ConditionType>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligation_expressions: Option<ObligationExpressionsType>,
    #[serde(rename = "AdviceExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    advice_expressions: Option<AdviceExpressionsType>
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

/// 5.25 Expression Substitution Group definition
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
//#[serde(untagged)]
pub enum ExpressionType {
    #[serde(rename = "Apply")]
    Apply(ApplyType),
    #[serde(rename = "AttributeDesignator")]
    AttributeDesignator(AttributeDesignatorType),
    #[serde(rename = "AttributeSelector")]
    AttributeSelector(AttributeSelectorType),
    #[serde(rename = "VariableReference")]
    VariableReference(VariableReferenceType),
    #[serde(rename = "Function")]
    Function(FunctionType),
    #[serde(rename = "AttributeValue")]
    AttributeValue(AttributeValueType)
}

/// 5.26 ConditionType definition
/// Boolean function over attributes or functions of attributes
/// Not clear if a vector is correct, as a xs:sequence is defined, but description says one expression
/// Might also be flattened?
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ConditionType {
    #[serde(rename = "$value")]
    expression: Vec<ExpressionType>
}

/// 5.27 ApplyType definition
/// Describes the application of a function to its arguments
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ApplyType {
    #[serde(rename = "@FunctionId")]
    function_id: String,        // More specific of URI type
    #[serde(rename = "@Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "$value",skip_serializing_if = "Option::is_none")]
    #[builder(default)] 
    expression: Option<Vec<ExpressionType>>
}

/// 5.28 FunctionType definition
/// Used to name a function in the ApplyType
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct FunctionType {
    #[serde(rename = "@FunctionId")]
    function_id: function::Function,   
}     

/// 5.29 AttributeDesignatorType definition
/// Used to retrieve a bag of attributes from the request context
/// The attribute id must match the id of an attribute in the request context
/// In case it is not contained, an error is raised according to the MustBePresent attribute
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AttributeDesignatorType{
    #[serde(rename = "@AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "@DataType")]
    data_type: DataType,          // More specific of URI type
    #[serde(rename = "@Category")]
    category: String,           // More specific of URI type
    #[serde(rename = "@MustBePresent")]
    must_be_present: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    issuer: Option<String>
}

/// 5.30 AttributeSelectorType definition
/// Used to retrieve a bag of unnamed and uncategorized attributes from the request context
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
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

/// 5.31 AttributeValueType definition
/// Contains a literal attribute value
/// Is kind of a special case as the data type of the value is described by the DataType attribute
#[derive(Serialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AttributeValueType {
    #[serde(rename = "@DataType")]
    data_type: DataType,          // More specific of URI type
    #[serde(rename = "$value")]
    value: Value
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Boolean(bool),
    Integer(i64),
    Double(EqF64)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct EqF64(f64);

impl Eq for EqF64 {}

impl PartialEq for EqF64 {
    fn eq(&self, other: &EqF64) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl FromStr for EqF64 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(EqF64).map_err(|_| ())
    }
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

///5.42 RequestType
/// Contains the request for a decision
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RequestType {
    #[serde(rename = "@ReturnPolicyIdList")]
    return_policy_id_list: bool,
    #[serde(rename = "@CombinedDecision")]
    combined_decision: bool,
    #[serde(rename = "RequestDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    request_defaults: Option<RequestDefaultsType>,
    #[serde(rename = "Attributes")]
    attributes: Vec<AttributesType>,
    #[serde(rename = "MultiRequests", skip_serializing_if = "UnimplementedField::is_none")]
    multi_requests: UnimplementedField // Is not yet implemented and optional, will fail if present
}

#[derive(Serialize, PartialEq, Eq, Debug)]
struct UnimplementedField(Option<String>);
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

/// 5.43 RequestDefaultsType
/// Contains the XPath Version for the request
/// Optional
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct RequestDefaultsType {
    #[serde(rename = "XPathVersion")]
    x_path_version: String,     // More specific of URI type
}

/// 5.44 AttributesType
/// Contains a set of attributes
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AttributesType {
    #[serde(rename = "@Category")]
    category: String,           //Specifies for what type of entity this attributes are defined
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xml_id: Option<String>,     // Unique identifier for the attributes
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    content: Option<Content>,        // Type 5.45, defined as sequence with 0 or 1 occurance
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attribute: Option<Vec<AttributeType>>    // Type 5.46, defined as sequence with ANY number
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

/// 5.46 AttributeType
/// Contains a single attribute metadata and value
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AttributeType {
    #[serde(rename = "@AttributeId")]
    attribute_id: String,       // Pre-defined URIs in the Annex B, but contain only commonly used; might be implemented as enum
    #[serde(rename = "@IncludeInResult", default = "default_false")]
    include_in_result: bool,
    #[serde(rename = "@Issuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    issuer: Option<String>,    
    #[serde(rename = "AttributeValue")]
    attribute_value: Vec<AttributeValueType>
}

fn default_false() -> bool {
    false
}   

/// 5.47 ResponseType
/// Standard return type for a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ResponseType {
    #[serde(rename = "Result")]
    result: Vec<ResultType>
}

/// 5.48 ResultType
/// Contains the result of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ResultType {
    #[serde(rename = "Decision")]
    decision: DecisionType,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status: Option<StatusType>,
    #[serde(rename = "Obligations", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligations: Option<ObligationsType>,
    #[serde(rename = "AssociatedAdvice", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    associated_advice: Option<AssociatedAdviceType>,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    attributes: Option<Vec<AttributesType>>,
    #[serde(rename = "PolicyIdentifierList", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_identifier_list: Option<PolicyIdentifierListType> // If set the return_policy_id_list true, this list contains policies that are fully-applicable
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

/// 5.50 - 5.52 are optional and skipped
/// 5.53 DecisionType
/// Enumeration to indicate the decision of a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum DecisionType {
    Permit,
    Deny,
    Indeterminate,
    NotApplicable
}

/// 5.54 StatusType
/// Contains the status of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct StatusType {
    #[serde(rename = "StatusCode")]
    status_code: StatusCodeType,
    #[serde(rename = "StatusMessage", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_message: Option<StatusMessageType>,
    #[serde(rename = "StatusDetail", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_detail: Option<StatusDetailType>
}

/// 5.55 StatusCodeType
/// Contains the status code of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct StatusCodeType {
    #[serde(rename = "@Value")]
    value: String,          // see Annex B.8 for values / implementation as enum 
    #[serde(rename = "StatusCode", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    status_code: Option<Vec<StatusCodeType>>     //Minor codes
}

/// 5.56 StatusMessageType
/// Contains the status message of a decision request
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct StatusMessageType {
    #[serde(rename = "$value")]
    value: String
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