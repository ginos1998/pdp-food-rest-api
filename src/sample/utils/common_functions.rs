pub fn get_limit_or_default(limit: Option<i64>, default: i64) -> i64 {
    match limit {
        Some(limit) => limit,
        None => default,
    }
}