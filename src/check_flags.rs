#[must_use]
pub fn check_flags(bytes: &[u8]) -> bool {
    bytes.iter().all(|&byte| byte == 0 || byte == 1)
}
