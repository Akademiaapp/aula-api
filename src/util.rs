use chrono::{DateTime, Utc, FixedOffset};

pub fn get_current_time_in_js_format(time_zone_offset: i32) -> String {
    let now: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(time_zone_offset * 3600).unwrap(); // Change the offset value according to your desired timezone
    let local_time = now.with_timezone(&offset);
    local_time.to_rfc3339()
}