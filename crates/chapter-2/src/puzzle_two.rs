#![allow(dead_code)]

use utils::device::Device;

fn run(input: &str) -> String {
    let (left, right, operation, formatting) = Device::parse(input);
    let result = Device::run(left, right, operation);
    Device::format(result, formatting)
}

pub fn solve() -> String {
    let first_result = run("YXYXXXYY; YYXXXXYX; G; E");
    let second_result = run("YXXYYXYX; YYXXXXYY; W; E");

    format!("{}{}", first_result, second_result)
}

