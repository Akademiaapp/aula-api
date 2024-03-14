use chrono::{DateTime, FixedOffset, Utc};

use crate::response_structs::get_events_by_profile_ids_and_resource_ids::Daum;

pub fn get_current_time_in_js_format(time_zone_offset: i32) -> String {
    let now: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(time_zone_offset * 3600).unwrap(); // Change the offset value according to your desired timezone
    let local_time = now.with_timezone(&offset);
    local_time.to_rfc3339().replace("+", "%2B")
}

pub fn compress_events(events: &mut Vec<Daum>) -> Vec<Daum> {
    let mut newVec = Vec::<Daum>::new();

    for a in events.iter() {
        let m = true;
        while m {
            for b in events.iter() {
                if a.end_date_time == b.start_date_time {
                    let mut new = a.clone();
                    new.end_date_time = b.end_date_time.clone();
                    newVec.push(new);
                    println!("compressed");
                    break;
                } else {
                    if a.start_date_time == b.end_date_time {
                        break;
                    }
                    newVec.push(a.clone());
                    break;
                }
            }
        }
    }

    newVec
}
