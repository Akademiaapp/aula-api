use std::collections::HashMap;
use std::sync::Arc;

use reqwest;
use reqwest::{Client, Response};
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::request_structs::get_events_by_profile_ids_and_resource_ids::GetEventsByProfileIdsAndResourceIdsReq;
use crate::response_structs::get_events_by_profile_ids_and_resource_ids::GetEventsByProfileIdsAndResourceIdsRes;
use crate::response_structs::get_profiles_by_login::GetProfilesByLoginRes;

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

pub struct Session {
    pub client: Client,
    pub cookie_store: Arc<CookieStoreMutex>
}
pub async fn unilogin(username: &str, password: &str) -> Result<Session, reqwest::Error> {
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

    Ok(Session { client, cookie_store })
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