/// Convert str to bytes.
pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 && s.len() >= 2 {
        let startposition;
        if s[..2].to_lowercase() == "0x" {
            startposition = 2;
        } else { startposition = 0 }

        (startposition..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            )
            .collect()
    } else {
        None
    }
}

/// Return the type of variables.
pub fn TypeOf<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

/// extract Non-Null bytes to String.
pub fn extract_string(input: &[u8]) -> String {
    let null_end = input
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(input.len());
    String::from_utf8_lossy(&input[0..null_end]).to_string()
}