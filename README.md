# xacml-rs
Rust-based XACML engine

This is a scientific protoype. As such, it is not intended to be used as policy engine in productive and / or security relevant use-cases. Be aware that there might be plenty of bugs and vulnerabilities in this code.
It is intended to investigate the creation and evaluation of XACML policies. As such, the main effort was to create a set of structs and enums representing the XACML 3.0 standard described [here](https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.html)

Although it is not intended to be used as productive XACML engine, this code can be used to create one - especially if it focuses on specific use-cases.

As of now, mainly the policy creation and evaluation was implemented and tested. Everything apart from this (especially everything outside the PDP) was not tested and mostly not implemented.

## Subjective Logic Integration

The integration of subjective-logic based automotive trust management systems was the main reason why this project was created. The evaluation of a paper currently under review for the CSCS'25 is described here. The conducted evaluation was either based on unit / integration tests for functional requirements, or on benchmarks for performance evaluation.

### Functional Unit Tests

Unit tests (using the rust / cargo test framework) are included in nearly all structs to test the specific functionality.
The subjective logic specific test cases including the code to create the example structures in the paper are included in `src/ls_policies/`. Basic tests and shared functions are in the `mod.rs` file, whereas `trust_discounting.rs` includes evaluation of the trust discounting mechanism.

All tests can be evaluated using `cargo test`.

### Benchmark / Performance Tests

Benchmark tests are created using the nightly benching feature. The necessary code is contained in the `benches/` directory.
To run the benchmark tests, the nightly relese channel has to be used:

```bash
rustup override set nighlty
cargo bench
# As alternative the following command temporarily uses the nightly toolchain:
cargo +nightly bench
```

Two types of benchmark sets are created: one to evaluate the basic subjective-logic based policy (`benchmark_subjective_logic_policy_evaluation`) and a set of tests to measure the time necessary to evaluate policies with varying numbers of rules (`bench_multiple_rule_policy_X`). With the above command, all benchmark tests are executed and the results are printed out.