use crate::utils::*;
use crate::xacml::structs::*;


pub fn get_policy_from_context(context: &str) -> Result<PolicyType, XacmlError> {
    let policy = parse_xml_file::<PolicyType>(context)
        .expect(format!("Failed to parse policy file for context: {}", context).as_str());
    Ok(policy)
}