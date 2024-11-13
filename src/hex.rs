/// # Panics
///
/// Panics if the input string is not a valid hex string.
#[must_use]
pub fn hex(str: &str) -> Vec<u8> {
    str.trim()
        .replace([' ', '\n'], "")
        .chars()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|byte_chars| {
            u8::from_str_radix(&byte_chars.iter().collect::<String>(), 16)
                .ok()
                .unwrap()
        })
        .collect()
}
