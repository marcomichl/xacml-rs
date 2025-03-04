# xacml-rs
Rust-based XACML engine

## Known limitation

### (De-) Serialization

- ContentType cannot parse substructure (did not yet find a way to parse arbitrary XML into string)
- AttributeAssignmentType extends AttributeValue, did not yet implement this (would need to reuse the specific dataType extraction)
- Only some dataTypes implemented

