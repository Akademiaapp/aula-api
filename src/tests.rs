#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::io::{self, Write};
    use std::path::Path;

    use serde_json::Value;

    use crate::aula_handler;
    use crate::response_structs::get_new_threads::GetNewThreadsRes;
    use crate::response_structs::messaging_get_threads::MessagingGetThreadsRes;
    use crate::util::get_current_time_in_js_format;
    use crate::LoginInfo;

    async fn test_login() -> aula_handler::AulaSession {
        let mut file = File::open("./user.json").expect("Failed to open JSON file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read JSON file");

        // Parse the JSON contents

        let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

        // Get the username and password from the JSON
        let username = json["username"]
            .as_str()
            .expect("Failed to get username from JSON");
        let password = json["password"]
            .as_str()
            .expect("Failed to get password from JSON");

        // Call the unilogin function

        let aula_session = aula_handler::AulaSession::from_credentials(username, password).await;
        aula_session
    }

    #[tokio::test]
    async fn test_get_events() {
       if !Path::new("./user.json").exists() {
            println!("File does not exist");
            return
        }
        let aula_session = test_login().await;

        let evensts = aula_session
            .request_events(
                "2024-03-09T08:35:11+00:00".to_string(),
                "2024-03-11T08:35:11+00:00".to_string(),
            )
            .await
            .unwrap();

        println!("{:?}", evensts);
    }

    #[tokio::test]
    async fn test_reuse_client() {
        if !Path::new("./user.json").exists() {
            println!("File does not exist");
            return
        }
        let aula_session = test_login().await;
        let aula_session =
            aula_handler::AulaSession::from_login_info(&aula_session.get_login_info()).await;
        let events = aula_session
            .request_events(
                "2024-03-09T08:35:11+00:00".to_string(),
                "2024-03-11T08:35:11+00:00".to_string(),
            )
            .await
            .unwrap();

        println!("{:?}", events);
    }

    #[test]
    fn js_time() {
        let time = get_current_time_in_js_format(0);
        println!("Time without timezone: {}", time);
        let time = get_current_time_in_js_format(1);
        println!("Time with timezone: {}", time);
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_msg() {
        println!("hi");

        let aula_session = test_login().await;

        let text: String = aula_session
            .request_all_messages('0'.to_string())
            .await
            .unwrap();

        println!("{:?}", text);

        let mut data: MessagingGetThreadsRes =
            serde_json::from_str(&text).expect("Failed to parse JSON");

        println!("{:?}", data);

        let mut i = 0;

        while data.data.more_messages_exist {
            i += 1;

            let text = aula_session
                .request_all_messages(i.to_string())
                .await
                .unwrap();

            data = serde_json::from_str(&text).expect("Failed to parse JSON");
        }

        println!("{:?}", i);
    }

    #[tokio::test]
    #[ignore]
    async fn test_msg_pulling() {
        println!("hi");

        let aula_session = test_login().await;

        // let text = aula_session.request_all_messages('0'.to_string()).await.unwrap();
        let mut time = get_current_time_in_js_format(1);

        for i in 0..300 {
            // https://www.aula.dk/api/v19/?method=messaging.getNewThreads&lastPollingTimestamp=2024-03-14T12:25:18%2B01:00&page=0
            let url = format!("https://www.aula.dk/api/v19/?method=messaging.getNewThreads&lastPollingTimestamp={}&page=0", time);
            let json = aula_session.request_get(url).await.unwrap();

            let info: GetNewThreadsRes = serde_json::from_value(json).unwrap();

            if info.data.more_messages_exist {
                println!();
                println!("{:?}", info.data.threads)
            } else {
                print!(".");
                io::stdout().flush().unwrap();
            }
            time = get_current_time_in_js_format(1).replace("+", "%2B");

            // tokio::time::sleep(Duration::from_secs(5)).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    #[test]
    fn hi() {
        #[derive(Clone, Debug)]
        struct Test {
            a: i32,
            b: i32,
        }

        let mut events = Vec::<Test>::new();
        events.push(Test { a: 1, b: 2 });
        // events.push(Test { a: 1, b: 2 });
        events.push(Test { a: 2, b: 3 });
        events.push(Test { a: 3, b: 4 });
        events.push(Test { a: 5, b: 6 });

        let mut newVec = Vec::<Test>::new();

        //     for a in events.iter() {
        //         let mut last = a.b.clone();
        //         for b in events.iter() {
        //             if a.a == b.b {
        //                 last = -1;
        //                 break
        //             } else { if a.b == b.a {

        //                 last = b.b.clone();
        //                 continue;
        //             } }

        //         }

        //         if last == -1 {continue;}
        //         let mut new = a.clone();
        //         new.b = last;
        //         newVec.push(new);

        //     }

        for a in events.iter() {
            let mut m = true;

            for b in events.iter() {
                if a.b == b.a {
                    let mut new = a.clone();
                    new.b = b.b.clone();
                    newVec.push(new);
                    println!("compressed");
                    m = false;
                    break;
                } else {
                    if a.a == b.b {
                        m = false;
                        break;
                    }
                }
            }
            if m {
                newVec.push(a.clone());
            }
        }

        println!("Length of newVec: {}", newVec.len());
        println!("{:?}", newVec);
    }
}
