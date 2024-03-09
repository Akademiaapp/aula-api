use std::fs::File;
use std::io::Read;
use serde_json::Value;

mod unilogin;

#[tokio::main]
async fn main() {

    // Read the JSON file
    let mut file = File::open("./user.json").expect("Failed to open JSON file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read JSON file");


    // Parse the JSON contents


    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // Get the username and password from the JSON
    let username = json["username"].as_str().expect("Failed to get username from JSON");
    let password = json["password"].as_str().expect("Failed to get password from JSON");

    // Call the unilogin function
    let client = unilogin::unilogin(username, password).await;
}

// Add the missing dependency 'unilogin' to your Cargo.toml file
// cargo.toml
// [dependencies]
// unilogin = "0.1.0"
