use reqwest;
use reqwest::{Client, Response};
use scraper::{Html, Selector};

async fn post_form(prev_r: Response, data: String, client: &Client) -> Result<Response, reqwest::Error> {
    let body = prev_r.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("form").unwrap();
    let form = document.select(&selector).next().unwrap();
    let action = form.value().attr("action").unwrap();

    println!("action: {}", action);
    
    let res: Response = client.post(action)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data) // Convert &str to String
        .send()
        .await?;
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
    println!("{:?}", resp.cookies().next());
    
    let mut r = post_form(resp, "selectedIdp=uni_idp".to_string(), &client).await?;
    r = post_form(r, format!("username={}", username), &client).await?;
    r = post_form(r, format!("password={}", password), &client).await?;



    println!("{}", r.text().await?);
    // ...

    // let mut file = File::open("/path/to/credentials.json")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // let json: serde_json::Value = serde_json::from_str(&contents)?;

    // let username = json["username"].as_str().unwrap();
    // let password = json["password"].as_str().unwrap();

    // ...

    // r = post_form(r, &format!("username={}&password={}", username, password), &client).await?;
    // let mut r: Response = post_form(resp, "selectedIdp=uni_idp", &client).await?;

    // r = post_form(r, &HashMap::from([("username", username), ("password", password)]), &client).await?;

    // let body = r.text().await?;
    // println!("{}", body);

    Ok(client)
}