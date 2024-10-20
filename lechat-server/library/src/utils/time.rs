#[inline]
pub fn now_timestamp_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

#[inline]
pub fn now_timestamp_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[inline]
pub fn now_naive_datetime() -> chrono::NaiveDateTime {
    let utc_now = chrono::Utc::now();
    utc_now.naive_utc()
}
