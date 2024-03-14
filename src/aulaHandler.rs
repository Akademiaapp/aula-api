use std::fmt::Error;
use std::sync::Arc;
use reqwest::{Client, Url};
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::HeaderValue;
use crate::request_structs::get_events_by_profile_ids_and_resource_ids::GetEventsByProfileIdsAndResourceIdsReq;
use crate::response_structs::get_events_by_profile_ids_and_resource_ids::{Daum, GetEventsByProfileIdsAndResourceIdsRes};
use crate::response_structs::get_profiles_by_login::{Data, GetProfilesByLoginRes};
use crate::response_structs::messaging_get_threads::MessagingGetThreadsRes;
use crate::unilogin;
use crate::unilogin::{Session, unilogin};

pub struct AulaSession {
    session: Session,
    profile_info: Data,
    pub token: String,
    pub php_session: String,
    id: i64
}
impl AulaSession {
    pub async fn new(username: &str, password: &str) -> Self {
        let session = unilogin(username, password).await.unwrap();

        //https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin
        let profile_info = Self::request_profile_info(&session.client).await;

        let token = Self::get_cookie(&session, "Csrfp-Token".to_string());
        let php_session = Self::get_cookie(&session, "PHPSESSID".to_string());

        let id = profile_info.profiles[0].institution_profiles[0].id;

        println!("id: {}", id);

        Self {session, profile_info, token, php_session, id }
    }

    pub async fn from_cookies(token: String, php_session: String) -> Self {
        println!("hi");
        // Create a cookie jar we can share with the HTTP client
        let cookie_store  = reqwest_cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        // create the HTTP client
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            // .cookie_store(true)
            .build()
            .unwrap();


        // Add some cookies
//         let php_cookie =HeaderValue::from_static (
//             format!("PHPSESSID={}; Domain=.aula.dk", php_session).as_str().clone()
//         );
//
        let token_str = format!("Csrfp-Token={}; Domain=www.aula.dk", token);
        let token_cookie = HeaderValue::from_str(&token_str).unwrap();

        let php_str = format!("PHPSESSID={}; Domain=aula.dk", php_session);
        let php_cookie = HeaderValue::from_str(&php_str).unwrap();




        cookie_store.set_cookies(&mut [&token_cookie].into_iter(), &Url::parse("https://www.aula.dk").unwrap());
        cookie_store.set_cookies(&mut [&php_cookie].into_iter(), &Url::parse("https://www.aula.dk").unwrap());
//
//
//
//         let url = "https//www.aula.dk".parse::<Url>().unwrap();
//         jar.set_cookies(&mut [&token_cookie].into_iter(), &url);
//         jar.set_cookies(&mut [&php_cookie].into_iter(), &url);

        let profile_info = Self::request_profile_info(&client).await;

        println!("{:?}", profile_info);

        let id = profile_info.profiles[0].institution_profiles[0].id;

        Self {session: Session {client, cookie_store }, profile_info, token, php_session, id}
    }

    fn get_cookie(session: &Session, name: String) -> String {
        let store = session.cookie_store.lock().unwrap();
        let x = store.iter_any().find(|c| c.name() == name).unwrap().value().to_string();
        x
    }


    async fn request_profile_info(client: &Client) -> Data {
        let r = client.get("https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin")
            .send()
            .await
            .unwrap();

        let text = r.text().await.unwrap_or("".to_string());
        println!("{:?}",  &text);

        let profile_info = serde_json::from_str::<GetProfilesByLoginRes>(&text).unwrap();

        profile_info.data
    }



    pub async fn request_events(&self, start: String, end: String) -> Result<Vec<Daum>, reqwest::Error> {
        let data = GetEventsByProfileIdsAndResourceIdsReq {
            inst_profile_ids: vec![self.id],
            resource_ids: vec![],
            start,
            end
        };

        // println!("{}", r.text().await?);
        let r = self.session.client
            .post("https://www.aula.dk/api/v18/?method=calendar.getEventsByProfileIdsAndResourceIds")
            .json(&data)
            .header("Csrfp-Token", &self.token)
            .send()
            .await?;
        // println!("{}", r.text().await?);



        let response_data = r.json::<GetEventsByProfileIdsAndResourceIdsRes>().await?;

        println!("{:?}", response_data);

        let result: Vec<Daum> = response_data.data;

        Ok(result)
    }

    pub async fn request_all_messages(&self, page: String) -> Result<String, reqwest::Error> {
        
        

        // println!("{}", r.text().await?);
        let r = self.session.client
            .get(format!("https://www.aula.dk/api/v18/?method=messaging.getThreads&sortOn=date&orderDirection=desc&page={}", page).as_str())
            .header("Csrfp-Token", &self.token)
            .send()
            .await?;


        

        Ok(r.text().await?)
    }
}