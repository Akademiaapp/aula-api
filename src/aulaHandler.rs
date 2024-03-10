use reqwest::Client;
use crate::unilogin;
use crate::unilogin::unilogin;

pub struct AulaSession {
    client: Client,
    id: Option<i64>
}
impl AulaSession {
    pub async fn new(username: &str, password: &str) -> Self {
        let client = unilogin(username, password).await.unwrap();
        Self {client, id: None}
    }

    pub fn method1(&self) {
        // method body
    }
}