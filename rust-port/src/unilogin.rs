use scraper::{Html, Selector};
use reqwest;
use std::collections::HashMap;
use reqwest::{Response, Client};

async fn post_form(prev_r: Response, data: &HashMap<&str, &str>, client: &Client) -> Result<Response, reqwest::Error> {
    let body = prev_r.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("form").unwrap();
    let form = document.select(&selector).next().unwrap();
    let action = form.value().attr("action").unwrap();

    println!("action: {}", action);

    let res: Response = client.post(action).json(&data).send().await?;
    Ok(res)
}

pub async fn unilogin(username: &str, password: &str) -> Result<Client, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .cookie_store(true)
        .build()?;
    

    let resp = client.get("https://www.aula.dk/auth/login.php?type=unilogin")
        .send()
        .await?;
    println!("{}", resp.status());
    print!("{:?}", resp.cookies().next());
    let mut r: Response = post_form(resp, &HashMap::from([("selectedIdp", "uni_idp")]), &client).await?;
    // r = post_form(r, &HashMap::from([("username", username), ("password", password)]), &client).await?;

    let body = r.text().await?;
    println!("{}", body);

    Ok(client)
}