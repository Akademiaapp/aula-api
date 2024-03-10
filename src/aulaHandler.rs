use std::fmt::Error;
use reqwest::Client;
use crate::request_structs::get_events_by_profile_ids_and_resource_ids::GetEventsByProfileIdsAndResourceIdsReq;
use crate::response_structs::get_events_by_profile_ids_and_resource_ids::{Daum, GetEventsByProfileIdsAndResourceIdsRes};
use crate::response_structs::get_profiles_by_login::{Data, GetProfilesByLoginRes};
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
        let profile_info = Self::request_profile_info(&session).await;

        let token = Self::get_cookie(&session, "Csrfp-Token".to_string());
        let php_session = Self::get_cookie(&session, "PHPSESSID".to_string());

        let id = profile_info.profiles[0].institution_profiles[0].id;

        println!("id: {}", id);

        Self {session, profile_info, token, php_session, id }
    }

    fn get_cookie(session: &Session, name: String) -> String {
        let store = session.cookie_store.lock().unwrap();
        let x = store.iter_any().find(|c| c.name() == name).unwrap().value().to_string();
        x
    }


    async fn request_profile_info(session: &Session) -> Data {
        let r = session.client.get("https://www.aula.dk/api/v18/?method=profiles.getProfilesByLogin")
            .send()
            .await;

        let profile_info = r.unwrap().json::<GetProfilesByLoginRes>().await.unwrap();

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

        let result = response_data.data;
        Ok(result)
    }
}