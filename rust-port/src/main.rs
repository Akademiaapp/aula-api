mod unilogin;
#[tokio::main]
async fn main() {
    // Create a new tokio runtime

    // Call the unilogin function
    let client = unilogin::unilogin("elli0509", "password1234").await;

}

// Add the missing dependency 'unilogin' to your Cargo.toml file
// cargo.toml
// [dependencies]
// unilogin = "0.1.0"
