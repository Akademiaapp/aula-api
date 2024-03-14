#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;
    use serde_json::Value;

    use crate::response_structs::get_events_by_profile_ids_and_resource_ids::Daum;
    use crate::unilogin;

    use crate::aulaHandler;
    use crate::util::compress_events;
    use crate::util::get_current_time_in_js_format;
    use crate::LoginInfo;

    use super::*;

    async fn test_login() -> aulaHandler::AulaSession {
        let mut file = File::open("./user.json").expect("Failed to open JSON file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read JSON file");
    
    
        // Parse the JSON contents
    
    
        let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    
        // Get the username and password from the JSON
        let username = json["username"].as_str().expect("Failed to get username from JSON");
        let password = json["password"].as_str().expect("Failed to get password from JSON");
    
        // Call the unilogin function
        

        let aula_session = aulaHandler::AulaSession::new(username, password).await;
        aula_session
    }

    #[tokio::test]
    async fn test_get_events() {
        println!("edcrfv");

        let aula_session = test_login().await;        
        
        let evensts = aula_session.request_events(
            "2024-03-09T08:35:11+00:00".to_string(), 
            "2024-03-11T08:35:11+00:00".to_string()
        ).await.unwrap();

        println!("{:?}", evensts);
    }

    #[tokio::test]
    async fn test_reuse_client() {
        let aula_session = test_login().await;        
        let login_info = LoginInfo { token: aula_session.token, php_session: aula_session.php_session };
        let aula_session = aulaHandler::AulaSession::from_cookies(login_info.token, login_info.php_session).await;
        let mut events = aula_session.request_events(
            "2024-03-09T08:35:11+00:00".to_string(), 
            "2024-03-11T08:35:11+00:00".to_string()
        ).await.unwrap();

        println!("events len before compression{:?}", events.len());
        
        println!("events len after compression{:?}", compress_events(&mut events).len());

        println!("{:?}", events);
    }

    #[test]
    fn js_time() {
        let time = get_current_time_in_js_format(0);
        println!("Time without timezone: {}", time);
        let time = get_current_time_in_js_format(1);
        println!("Time with timezone: {}", time);
    }

}