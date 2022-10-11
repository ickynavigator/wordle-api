use chrono::{DateTime, NaiveDateTime, Utc};

pub fn check_if_same_day(timestamp: i64) -> bool {
    // get date from timestamp
    let today_datetime = Utc::now();
    let timestamp_to_datetime =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

    let today_date = today_datetime.date();
    let timestamp_date = timestamp_to_datetime.date();

    // check if the dates are the same
    if today_date == timestamp_date {
        return true;
    } else {
        return false;
    }
}
