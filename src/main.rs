use std::process::Command;

use response::Response;

mod response;

fn main() {
    let output = Command::new("speedtest")
        .arg("--format=json")
        .arg("--accept-license")
        .arg("--accept-gdpr")
        .output()
        .expect("Failed to execute 'speedtest'");

    let response: Response = serde_json::from_slice(&output.stdout).unwrap();

    println!("{response:?}");
}
