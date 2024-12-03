pub struct PolicySet {
    policy_set_id: String,
    version: i32,
    policy_combining_alg_id: String, // Combining algorithm, as of now string, might later be an enum
    max_delegation_depth: Option<i32>,
    description: Option<String>,        // checked data type
    policy_issuer: Option<String>,      // Complex data type, defined in the XACMLAdmin standard; PDP not implementing this should report an error if it is contained -> shall raise an error
    policy_set_defaults: Option<DefaultsType>, // Policy defaults, comlpex data type
    target: Target,
    policy_sets: Vec<PolicySet>,
    policy: Vec<Policy>,
    policy_set_id_reference: Vec<IdReferenceType>,
    policy_id_reference: Vec<IdReferenceType>,
    obligation_expressions: Option<Vec<String>>,
    advice_expressions: Option<Vec<String>>,
    combiner_parameters: Option<Vec<String>>,
    policy_combiner_parameters: Option<Vec<String>>,
    poicy_set_combiner_parameters: Option<Vec<String>>
}

/// Shall appear in the policy set or the policy elements, may be contained in a rule element
/// Shall contain a conjunctive sequence of <AnyOf> elements, to be applicable one of these has to match the decision request.
/// Each AnyOf element contains a disjunctive AllOf element, that all have to match the decision request.
pub struct Target {
    any_of: Option<Vec<AnyOf>>                  // Data type for elements, of which one must match the context to be applicable; if empty, the target is always applicable; might be changed to a simple vec, that can also be of length 0
}

pub struct AnyOf {
    all_of: Option<Vec<AllOf>>                  // Data type for elements, of which all must match the context to be applicable; if empty, the anyOf is always applicable; might be changed to a simple vec, that can also be of length 0
}

pub struct AllOf {
    _match: Vec<Match>                           // One or many, all of which must match the context to be applicable
}

pub struct Match {
    match_id: String,                        // More specific of URI type
    attribute_value: String,
    attribute_designator: Option<AttributeDesignator>,   // Either this or the attributeSelector must be present, not both and not none
    attribute_selector: Option<AttributeSelector>
}

pub struct Policy {

}

pub struct DefaultsType {
    x_path_version: Option<Vec<String>>        // more specific URI type
}

pub struct AttributeDesignator {
}

pub struct AttributeSelector {
}

/// Extends the xs:anyURI type
/// The referenced policy set with the id has to match the remaining attributes
pub struct IdReferenceType {
    id: String,                                 // More specific of URI type
    version: Option<VersionMatchType>,                        // Exact version
    earliest_version: Option<VersionMatchType>,                // Earliest version
    latest_version: Option<VersionMatchType>,                  // Latest version
}


pub struct VersionMatchType{
    
}