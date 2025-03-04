mod xacml;
mod utils;
mod create_files;

use std::env;
use utils::{parse_xml_file, serialize_to_xml_file};
use xacml::structs::*;

fn main() {
 
    println!("XACML 3.0 Rust implementation");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<args>]", args[0]);
        return;
    }

    match args[1].as_str() {
        "decideRequest" => {
            if args.len() != 4 {
                eprintln!("Usage: {} evaluate <policy> <context>", args[0]);
                return;
            }
            let policy = &args[2];
            let context_id = &args[3];
            decide_request(policy, context_id);
        }
        "evaluatePolicy" => {
            if args.len() != 4 {
                eprintln!("Usage: {} evaluate <policy> <context>", args[0]);
                return;
            }
            let policy = &args[2];
            let context_id = &args[3];
            evaluate_policy(policy, context_id);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

    fn evaluate_policy(policy_file: &str, context: &str) {
        // Placeholder for actual policy evaluation logic
        let policy = parse_xml_file::<PolicyType>(policy_file)
        .expect("Failed to parse policy file");
        
    }

    fn decide_request(request_file: &str, context_id: &str) {
        let request = parse_xml_file::<RequestType>(request_file)
            .expect("Failed to parse request file");
    }
