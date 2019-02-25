pub fn flag_is_enabled(v: Option<&bool>) -> bool {
    match v {
        Some(flag) => *flag,
        None => false,
    }
}
