# Notes to XACML-RS

## Enums

The enums describe possible content of various fields in the structs. E.g. the function table describes function IDs (in this case URIs) that are mapped to specific functions.

## Test cases

Additional test cases to simple ones or the ones in the specification can be the conformance test elements, e.g. contained here (AuhtForze Project that adapted the 2.0 conformance test cases to the 3.0 specification).
Target (as of now) would be, that all the test cases can be (de-) serialized, but only the functions and methods necessary for the diss are implemented.
https://github.com/authzforce/core/tree/develop/pdp-testutils/src/test/resources/conformance/xacml-3.0-from-2.0-ct

Therefore, the xml-files would be serialized and deserialized in the test cases, but the Policy + Request would not be evaluated to match the Result.

## ToDo

Adapt function implementations so that only Vec<Value> is given to them (retrieving the correct values is not handled in the function implementation, as this signature is necessary in the match_type)
And think about how the Vec<Value> can be created in the match_type..
Then the result of the applied function has to be verified to be boolean -> result of match can be returned