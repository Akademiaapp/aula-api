use std::collections::HashMap;

use reqwest;
use reqwest::{Client, Response, Url};
use reqwest::cookie::CookieStore;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::responseStructs::getProfilesByLoginStruct::GetProfilesByLogin;

fn find_form_action(prev_r: &String, name: Option<&String>) -> String {
    // implementation here
    let document = Html::parse_document(&prev_r);


    let selector = if name.is_some() {
        
        Selector::parse(&format!("form[name=\"{}\"]", name.unwrap())).unwrap()
    } else {
        Selector::parse("form").unwrap()
    };
    let form = document.select(&selector).next().unwrap();
    let action = form.value().attr("action").unwrap();
    action.to_string()
    }

async fn post_form(prev_r: Response, data: String, client: &Client) -> Result<Response, reqwest::Error> {
    let body = prev_r.text().await?;
    let action = find_form_action(&body, None);

    println!("action: {}", action);
    
    let res: Response = client.post(action)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data) // Convert &str to String
        .send()
        .await?;
    Ok(res)
}

pub async fn unilogin(username: &str, password: &str) -> Result<Client, reqwest::Error> {
    let cookie_store  = reqwest_cookie_store::CookieStore::default();
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);
    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        // .cookie_store(true)
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()?;
    

    let resp = client.get("https://www.aula.dk/auth/login.php?type=unilogin")
        .send()
        .await?;
    println!("{}", resp.status());
    println!("{:?}", resp.cookies().next());
    
    let mut r = post_form(resp, "selectedIdp=uni_idp".to_string(), &client).await?;
    r = post_form(r, format!("username={}", username), &client).await?;
    r = post_form(r, format!("password={}", password), &client).await?;


    let payload = get_payload(&r.text().await?);

    println!("{:?}", &payload);

    r = client.post("https://broker.unilogin.dk/auth/realms/broker/broker/uni_idp/endpoint")
        .form(&payload.unwrap())
        .send()
        .await?;

    r = post_form(r, "".to_string(), &client).await?;

    let text = r.text().await?;
    let action = find_form_action(&text, Some(&"saml-post-binding".to_string()));
    println!("action: {}", action);

    r = client.post(action)
        .form(&get_payload(&text).unwrap())
        .send()
        .await.unwrap();
    
    println!("cookies");
    


    for i in cookie_store.cookies( &Url::parse("https://www.aula.dk").unwrap() ) {
        println!("cookie: {:?}", i);
    }

    //https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin

    r = client.get("https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin")
        .send()
        .await?;

    println!("profile cookies: {:?}", r.headers().get("Set-Cookie"));
    
    println!("after google.com GET");
     
    

    let csfrp_token = {
        let store = cookie_store.lock().unwrap();
        let x = store.iter_any().find(|c| c.name() == "Csrfp-Token").unwrap().value().to_string(); x
    };

    let instProfileIds = r.json::<GetProfilesByLogin>().await.unwrap();


    let id = instProfileIds.data.profiles[0].institution_profiles[0].id;


    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct data {
        pub inst_profile_ids: Vec<i64>,
        pub resource_ids: Vec<Value>,
        pub start: String,
        pub end: String,
    }

    let data = data {
        inst_profile_ids: vec![instProfileIds.data.profiles[0].institution_profiles[0].id],
        resource_ids: vec![],
        start: "2024-03-10 00:00:00.0000+01:00".to_string(),
        end: "2024-04-10 23:59:59.9990+02:00".to_string()
    };

    // println!("{}", r.text().await?);
    r = client
        .post("https://www.aula.dk/api/v18/?method=calendar.getEventsByProfileIdsAndResourceIds")
        .json(&data)
        .header("Csrfp-Token", csfrp_token)
        .send()
        .await?;
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

fn get_payload(html_text: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut payload = HashMap::new();
    let fragment = Html::parse_document(html_text);

    let saml_response_selector = Selector::parse(r#"input[name="SAMLResponse"]"#).unwrap();
    let relay_state_selector = Selector::parse(r#"input[name="RelayState"]"#).unwrap();

    if let Some(saml_response_element) = fragment.select(&saml_response_selector).next() {
        let saml_response_value = saml_response_element.value().attr("value").unwrap().to_string();
        payload.insert("SAMLResponse".to_string(), saml_response_value);
    }

    if let Some(relay_state_element) = fragment.select(&relay_state_selector).next() {
        let relay_state_value = relay_state_element.value().attr("value").unwrap().to_string();
        payload.insert("RelayState".to_string(), relay_state_value);
    }

    Ok(payload)
}