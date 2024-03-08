use scraper::{Html, Selector};
use reqwest;
use std::collections::HashMap;
use reqwest::{Response, Client};

async fn post_form(prev_r: Response, data: HashMap<&str, &str>, client: Client) -> Result<Response, reqwest::Error> {
    let body = prev_r.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("form").unwrap();
    let form = document.select(&selector).next().unwrap();
    let action = form.value().attr("action").unwrap();

    let res: Response = client.post(action)
        .send().await?;
    Ok(res)
}

async fn unilogin(username: &str, password: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()?;

    let resp = client.get("https://www.aula.dk/auth/login.php?type=unilogin")
        .send()
        .await?;

    let resp = client.get(resp.headers().get("location").unwrap().to_str().unwrap())
        .send()
        .await?;

    let href = resp.headers().get("location").unwrap().to_str().unwrap();
    Ok(())
}