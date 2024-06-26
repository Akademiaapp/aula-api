use std::sync::Arc;

use reqwest::{Client, Url};
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::request_structs::get_events_by_profile_ids_and_resource_ids::GetEventsByProfileIdsAndResourceIdsReq;
use crate::response_structs::get_events_by_profile_ids_and_resource_ids::{
    Daum, GetEventsByProfileIdsAndResourceIdsRes,
};
use crate::response_structs::get_profiles_by_login::{Data, GetProfilesByLoginRes};
use crate::unilogin::{Session, unilogin};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    token: String,
    php_session: String,
    id: i64,
    institution_code: String,
}

pub struct AulaSession {
    session: Session,
    pub token: String,
    pub php_session: String,
    id: i64,
    pub institution_code: String,
}

impl AulaSession {
    pub async fn from_credentials(username: &str, password: &str) -> Self {
        let session = unilogin(username, password).await.unwrap();

        Self::new(session).await
    }

    async fn new(session: Session) -> Self {
        //https://www.aula.dk/api/v19/?method=profiles.getProfilesByLogin
        let profile_info = Self::request_profile_info(&session.client).await;

        let token = Self::get_cookie(&session, "Csrfp-Token".to_string());
        let php_session = Self::get_cookie(&session, "PHPSESSID".to_string());
        let profile = &profile_info.profiles[0].institution_profiles[0];
        let id = profile.id;
        let institution_code = profile.institution_code.clone();

        println!("id: {}", id);

        Self {
            session,
            token,
            php_session,
            id,
            institution_code,
        }
    }

    pub(crate) fn get_login_info(&self) -> LoginInfo {
        LoginInfo {
            token: self.token.clone(),
            php_session: self.php_session.clone(),
            id: self.id,
            institution_code: self.institution_code.clone(),
        }
    }

    pub async fn from_login_info(info: &LoginInfo) -> Self {
        let session =
            Self::client_from_cred_cookies(info.token.clone(), info.php_session.clone()).await;

        Self {
            session,
            token: info.token.clone(),
            php_session: info.php_session.clone(),
            id: info.id,
            institution_code: info.institution_code.clone(),
        }
    }

    async fn client_from_cred_cookies(token: String, php_session: String) -> Session {
        let cookie_store = reqwest_cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        // create the HTTP client
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            // .cookie_store(true)
            .build()
            .unwrap();

        let token_str = format!("Csrfp-Token={}; Domain=www.aula.dk", token);
        let token_cookie = HeaderValue::from_str(&token_str).unwrap();

        let php_str = format!("PHPSESSID={}; Domain=aula.dk", php_session);
        let php_cookie = HeaderValue::from_str(&php_str).unwrap();

        cookie_store.set_cookies(
            &mut [&token_cookie].into_iter(),
            &Url::parse("https://www.aula.dk").unwrap(),
        );
        cookie_store.set_cookies(
            &mut [&php_cookie].into_iter(),
            &Url::parse("https://www.aula.dk").unwrap(),
        );

        Session {
            client,
            cookie_store,
        }
    }

    pub async fn from_cookies(token: String, php_session: String) -> Self {
        println!("hi");
        // Create a cookie jar we can share with the HTTP client
        let cookie_store = reqwest_cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        // create the HTTP client
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            // .cookie_store(true)
            .build()
            .unwrap();

        let token_str = format!("Csrfp-Token={}; Domain=www.aula.dk", token);
        let token_cookie = HeaderValue::from_str(&token_str).unwrap();

        let php_str = format!("PHPSESSID={}; Domain=aula.dk", php_session);
        let php_cookie = HeaderValue::from_str(&php_str).unwrap();

        cookie_store.set_cookies(
            &mut [&token_cookie].into_iter(),
            &Url::parse("https://www.aula.dk").unwrap(),
        );
        cookie_store.set_cookies(
            &mut [&php_cookie].into_iter(),
            &Url::parse("https://www.aula.dk").unwrap(),
        );

        Self::new(Session {
            client,
            cookie_store,
        })
        .await
    }

    fn get_cookie(session: &Session, name: String) -> String {
        let store = session.cookie_store.lock().unwrap();
        let x = store
            .iter_any()
            .find(|c| c.name() == name)
            .unwrap()
            .value()
            .to_string();
        x
    }

    async fn request_profile_info(client: &Client) -> Data {
        let r = client
            .get("https://www.aula.dk/api/v19/?method=profiles.getProfilesByLogin")
            .send()
            .await
            .unwrap();

        let text = r.text().await.unwrap_or("".to_string());
        println!("{:?}", &text);

        let profile_info = serde_json::from_str::<GetProfilesByLoginRes>(&text).unwrap();

        profile_info.data
    }

    pub async fn request_events(
        &self,
        start: String,
        end: String,
    ) -> Result<Vec<Daum>, reqwest::Error> {
        let data = GetEventsByProfileIdsAndResourceIdsReq {
            inst_profile_ids: vec![self.id],
            resource_ids: vec![],
            start,
            end,
        };

        // println!("{}", r.text().await?);
        let r = self
            .session
            .client
            .post(
                "https://www.aula.dk/api/v19/?method=calendar.getEventsByProfileIdsAndResourceIds",
            )
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
            .get(format!("https://www.aula.dk/api/v19/?method=messaging.getThreads&sortOn=date&orderDirection=desc&page={}", page).as_str())
            .header("Csrfp-Token", &self.token)
            .send()
            .await?;

        Ok(r.text().await?)
    }

    pub async fn request_get(&self, url: String) -> Result<Value, reqwest::Error> {
        let r = self
            .session
            .client
            .get(url)
            // .header("Csrfp-Token", &self.token)
            .send()
            .await?;

        let text = r.text().await?;
        let v: Value = serde_json::from_str(&text).unwrap();
        Ok(v)
    }
}
