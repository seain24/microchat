use std::time::UNIX_EPOCH;

pub fn current_timestamp() -> u128 {
    std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}
