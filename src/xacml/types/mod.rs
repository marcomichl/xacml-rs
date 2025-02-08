mod test_types;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PolicySet {
    #[serde(rename = "PolicySetId")]
    policy_set_id: String,
    #[serde(rename = "Version")]
    version: VersionType,
    #[serde(rename = "PolicyCombiningAlgId")]
    policy_combining_alg_id: String, // Combining algorithm, as of now string, might later be an enum
    #[serde(rename = "MaxDelegationDepth", skip_serializing_if = "Option::is_none")]
    max_delegation_depth: Option<i32>,
    // 5.2 Description element
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,        // checked data type
    // 5.3 PolicyIssuer element
    #[serde(rename = "PolicyIssuer", skip_serializing_if = "Option::is_none")]
    policy_issuer: Option<PolicyIssuerType>,
    // 5.4 PolicyDefaults element
    #[serde(rename = "PolicySetDefaults", skip_serializing_if = "Option::is_none")]
    policy_set_defaults: Option<DefaultsType>,
    // 5.6 Target element defining the applicability of the policyset, policy or (optional) a rule
    #[serde(rename = "Target")]
    target: TargetType,
    #[serde(rename = "PolicySet", skip_serializing_if = "Option::is_none")]  // Might be any number (including 0) -> might be necessary to make it an option, so that complete missing fields are accepted (not only empty vecs)
    policy_set: Option<Vec<PolicySet>>,
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]  // Might be any number (including 0)
    policy: Option<Vec<Policy>>,
    // 5.10 PolicySetIdReference element
    #[serde(rename = "PolicySetIdReference", skip_serializing_if = "Option::is_none")]  // Reference to a policy that must be included in the PolicySet
    policy_set_id_reference: Option<Vec<IdReferenceType>>,
    // 5.11 PolicyIdReference element
    #[serde(rename = "PolicyIdReference", skip_serializing_if = "Option::is_none")]  // Reference to a policy that must be included in the Policy
    policy_id_reference: Option<Vec<IdReferenceType>>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    obligation_expressions: Option<Vec<String>>,
    #[serde(rename = "AdvideExpressions", skip_serializing_if = "Option::is_none")]
    advice_expressions: Option<Vec<String>>,
    #[serde(rename = "CombinerParameters", skip_serializing_if = "Option::is_none")]
    combiner_parameters: Option<Vec<String>>,
    #[serde(rename = "PolicyCombinerParameters", skip_serializing_if = "Option::is_none")]
    policy_combiner_parameters: Option<Vec<String>>,
    #[serde(rename = "PolicySetCombinerParameters", skip_serializing_if = "Option::is_none")]
    poicy_set_combiner_parameters: Option<Vec<String>>
}

/// Type describing the issuer of a Policy or Policy set
/// Defined in XACMLAdmin specification, if the PDP does not implement this, it should raise an error if it is contained
/// As of now this is not implemented
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PolicyIssuerType {
    // Free form XML content
    #[serde(rename = "Content")]
    content: String,        // Type 5.45
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    attribute: Option<Vec<String>>    // Type 5.46  
}


/// Shall appear in the policy set or the policy elements, may be contained in a rule element
/// Shall contain a conjunctive sequence of <AnyOf> elements, to be applicable one of these has to match the decision request.
/// Each AnyOf element contains a disjunctive AllOf element, that all have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TargetType {
    #[serde(rename = "AnyOf", skip_serializing_if = "Option::is_none")]
    any_of: Option<Vec<AnyOfType>>                  // Data type for elements, of which one must match the context to be applicable; if empty, the target is always applicable; might be changed to a simple vec, that can also be of length 0
}

/// 5.7 AnyOf element
/// Shall contain a disjunctive sequence of <AllOf> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AnyOfType {
    #[serde(rename = "AllOf")]
    all_of: Vec<AllOfType>                  // Data type for elements, of which all must match the context to be applicable; if empty, the anyOf is always applicable; might be changed to a simple vec, that can also be of length 0
}

/// 5.8 AllOf element
/// Shall contain a conjunctive sequence of <Match> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AllOfType {
    #[serde(rename = "Match")]
    _match: Vec<MatchType>                           // One or many, all of which must match the context to be applicable
}

/// 5.9 Match element
/// Shall contain a condition that must be fulfilled by the context to be applicable
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MatchType {
    #[serde(rename = "MatchId")]
    match_id: String,                        // More specific of URI type
    #[serde(rename = "AttributeValue")]
    attribute_value: String,
    #[serde(rename = "AttributeDesignator", skip_serializing_if = "Option::is_none")]
    attribute_designator: Option<AttributeDesignator>,   // Either this or the attributeSelector must be present, not both and not none
    #[serde(rename = "AttributeSelector", skip_serializing_if = "Option::is_none")]
    attribute_selector: Option<AttributeSelector>
}

/// 5.14 Policy element
/// Describes a policy as smallest unit useable by a PDP
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Policy {
    #[serde(rename = "@PolicyId")]
    policy_id: String,                          // More specific of URI type
    #[serde(rename = "@Version")]
    version: VersionType,
    #[serde(rename = "@RuleCombiningAlgId")]
    rule_combining_alg_id: String,              // Combining algorithm, as of now string, might later be an enum
    #[serde(rename = "@MaxDelegationDepth", skip_serializing_if = "Option::is_none")]
    max_delegation_depth: Option<i32>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,                
    #[serde(rename = "PolicyIssuer", skip_serializing_if = "Option::is_none")]
    policy_issuer: Option<PolicyIssuerType>,
    #[serde(rename = "PolicyDefaults", skip_serializing_if = "Option::is_none")]
    policy_defaults: Option<DefaultsType>,
    #[serde(rename = "CombinerParameters", skip_serializing_if = "Option::is_none")]
    combiner_parameters: Option<Vec<CombinerParametersType>>,       // own type?
    #[serde(rename = "RuleCombinerParameters", skip_serializing_if = "Option::is_none")]
    rule_combiner_parameters: Option<Vec<String>>,  // own type?
    #[serde(rename = "Target")]
    target: Vec<TargetType>,
    #[serde(rename = "VariableDefinition", skip_serializing_if = "Option::is_none")]
    variable_definition: Option<Vec<String>>,       // own type?
    #[serde(rename = "Rule")]
    rule: Vec<Rule>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    obligation_expressions: Option<Vec<String>>,
    #[serde(rename = "AdvideExpressions", skip_serializing_if = "Option::is_none")]
    advice_expressions: Option<Vec<String>>,
    
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Rule {

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DefaultsType {
    x_path_version: Option<Vec<String>>        // more specific URI type
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AttributeDesignator {
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AttributeSelector {
}

/// Extends the xs:anyURI type
/// The referenced policy set with the id has to match the remaining attributes
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct IdReferenceType {
    id: String,                                 // More specific of URI type
    version: Option<VersionMatchType>,                        // Exact version
    earliest_version: Option<VersionMatchType>,                // Earliest version
    latest_version: Option<VersionMatchType>,                  // Latest version
}

/// 5.13 VersionMatchType
/// Type for the version attribute of the policy set or policy
/// Consists of a string, that is restricted to numbers separated by dots
/// The two wildcards * for any and + for any higher version are allowed
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct VersionMatchType{
    version: String
}                           

/// 5.12 VersionType
/// Type for the version attribute of the policy set or policy
/// Consists of a string, that is restricted to numbers separated by dots
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct VersionType (String); //Newtype wrapper as this is a simple string restricted to numbers separated by dots, should be implemented later

/// 5.16 CombinerParameterType
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CombinerParametersType {
    #[serde(rename = "ParameterName")]
    combiner_parameters: Vec<CombinerParameterType>
}

/// 5.17 CombinerParameter
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CombinerParameterType {
    #[serde(rename = "ParameterName")]
    parameter_name: String,
    #[serde(rename = "ParameterValue")]
    parameter_value: String
}


/// 5.18 RuleCombinerParametersType
/// Extends the CombinerParametersType with a rule id that must reference a rule in the policy
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct RuleCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "RuleIdRef")]
    rule_id_ref: String
}

/// 5.19 PolicyCombinerParametersType
/// Extends the CombinerParametersType with a policy id that must reference a policy in the policy set
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PolicyCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "PolicyIdRef")]
    policy_id_ref: String           // More specific of URI type
}

/// 5.20 PolicySetCombinerParametersType
/// Extends the CombinerParametersType with a policy set id that must reference a policy set in the policy set
/// None of the Combination Methods in XACML 3.0 use this type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PolicySetCombinerParametersType {
    #[serde(flatten)]
    combiner_parameters: Vec<CombinerParametersType>,
    #[serde(rename = "PolicySetIdRef")]
    policy_set_id_ref: String           // More specific of URI type
}

/// 5.21 RuleType
/// Defines a rule in a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct RuleType {
    #[serde(rename = "RuleId")]
    rule_id: String,
    #[serde(rename = "Effect")]
    effect: EffectType,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    target: Option<TargetType>,
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    condition: Option<ConditionType>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    obligation_expressions: Option<Vec<String>>,
    #[serde(rename = "AdviceExpressions", skip_serializing_if = "Option::is_none")]
    advice_expressions: Option<Vec<String>>
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
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct VariableDefinitionType {
    #[serde(rename = "Expression")]
    expression: Vec<ExpressionType>,
    #[serde(rename = "VariableId")]
    variable_id: String
}

/// 5.24 VariableReferenceType
/// References a variable in a policy
/// Is allowed anywhere in the policy where an ExpressionType is allowed
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct VariableReferenceType {
    #[serde(rename = "VariableId")]
    variable_id: String
}

/// 5.25 Expression Substitution Group definition
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConditionType {
    #[serde(rename = "Expression")]
    expression: Vec<ExpressionType>
}

/// 5.27 ApplyType definition
/// Describes the application of a function to its arguments
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ApplyType {
    #[serde(rename = "FunctionId")]
    function_id: String,        // More specific of URI type
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Expression", skip_serializing_if = "Option::is_none")]
    expression: Option<Vec<ExpressionType>>
}

/// 5.28 FunctionType definition
/// Used to name a function in the ApplyType
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct FunctionType {
    #[serde(rename = "FunctionId")]
    function_id: String,        // More specific of URI type
}

/// 5.29 AttributeDesignatorType definition
/// Used to retrieve a bag of attributes from the request context
/// The attribute id must match the id of an attribute in the request context
/// In case it is not contained, an error is raised according to the MustBePresent attribute
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AttributeDesignatorType{
    #[serde(rename = "AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "DataType")]
    data_type: String,          // More specific of URI type
    #[serde(rename = "Category")]
    category: String,           // More specific of URI type
    #[serde(rename = "MustBePresent")]
    must_be_present: bool,
    #[serde(rename = "Issuer", skip_serializing_if = "Option::is_none")]
    issuer: Option<String>     
}

/// 5.30 AttributeSelectorType definition
/// Used to retrieve a bag of unnamed and uncategorized attributes from the request context
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AttributeSelectorType {
    #[serde(rename = "Category")]
    category: String,           // More specific of URI type
    #[serde(rename = "ContextSelectorId")]
    context_selector_id: String,       // More specific of URI type
    #[serde(rename = "Path")]
    path: String,               
    #[serde(rename = "DataType")]
    data_type: String,          // More specific of URI type
    #[serde(rename = "MustBePresent")]
    must_be_present: bool
}

/// 5.31 AttributeValueType definition
/// Contains a literal attribute value
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AttributeValueType {
    #[serde(rename = "DataType")]
    data_type: String,          // More specific of URI type
    #[serde(rename = "$value")]
    value: String
}

/// 5.32 OblicationsType definition
/// Contains a set of oblication elements
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ObligationsType {
    #[serde(rename = "Obligation")]
    obligations: Vec<ObligationType>
}

/// 5.33 AssociatedAdviceType definition
/// Contains a set of advice elements
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AssociatedAdviceType {
    #[serde(rename = "Advice")]
    advice: Vec<AdviceType>
}

/// 5.34 ObligationType definition
/// Contains an obligation element
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ObligationType {
    #[serde(rename = "ObligationId")]
    obligation_id: String,      // More specific of URI type
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}

/// 5.35 AdviceType definition
/// Containts an identifier for the advice and a set of attributes as supplemantal information
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AdviceType {
    #[serde(rename = "AdviceId")]
    advice_id: String,          // More specific of URI type
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}

/// 5.36 AttributeAssignmentType definition
/// Used to include arguments in obligations and advices
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AttributeAssignmentType {
    #[serde(flatten)]
    attribute: AttributeValueType,
    #[serde(rename = "AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    category: Option<String>,          // More specific of URI type
    #[serde(rename = "Issuer", skip_serializing_if = "Option::is_none")]
    issuer: Option<String>
}

/// 5.37 ObligationExpressionsType definition
/// Contains a set of obligation expressions
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ObligationExpressionsType {
    #[serde(rename = "ObligationExpression")]
    obligation_expressions: Vec<ObligationExpressionType>
}

/// 5.38 AdviceExpressionsType definition
/// Contains a set of advice expressions
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AdviceExpressionsType {
    #[serde(rename = "AdviceExpression")]
    advice_expressions: Vec<AdviceExpressionType>
}

/// 5.39 ObligationExpressionType definition
/// Element that evaluates to an obligation
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ObligationExpressionType {
    #[serde(rename = "ObligationId")]
    obligation_id: String,      // More specific of URI type
    #[serde(rename = "FulfillOn")]
    fulfill_on: EffectType,
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}


/// 5.40 AdviceExpressionType definition
/// Element that evaluates to an advice
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AdviceExpressionType {
    #[serde(rename = "AdviceId")]
    advice_id: String,          // More specific of URI type
    #[serde(rename = "AppliesTo")]
    applies_to: EffectType,
    #[serde(rename = "AttributeAssignment", skip_serializing_if = "Option::is_none")]
    attribute_assignment: Option<Vec<AttributeAssignmentType>>
}

/// 5.41 AttributeAssignmentExpressionType definition
/// Used to include arguments in obligations and advices
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AttributeAssignmentExpressionType {
    #[serde(rename = "AttributeId")]
    attribute_id: String,       // More specific of URI type
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    category: Option<String>,          // More specific of URI type
    #[serde(rename = "Issuer", skip_serializing_if = "Option::is_none")]
    issuer: Option<String>,
    #[serde(rename = "Expression")]
    expression: Vec<ExpressionType>
}