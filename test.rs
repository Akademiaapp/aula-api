use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://httpbin.org/get").await?;

    println!("{}", response.status());

    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}