use std::collections::HashMap;

pub fn validate_http_version(p0: &str) -> Result<(), &str> {
    match p0.splitn(2, "/").last() {
        Some(v) => match v.parse::<f32>() {
            Ok(version) => {
                if version < 1.1 {
                    Err("Invalid HTTP version")
                } else {
                    Ok(())
                }
            }
            Err(_) => Err("Invalid HTTP version"),
        },
        None => Err("Invalid Request"),
    }
}

pub fn parse_headers(h_lines: &[String]) -> HashMap<String, String> {
    h_lines
        .iter()
        .skip(1)
        .map(|d| {
            let mut header = d.splitn(2, ":");
            (
                header.next().unwrap().trim().to_ascii_lowercase(),
                header.next().unwrap().trim().to_string(),
            )
        })
        .filter(|(k, v)| !k.is_empty() && !v.is_empty())
        .collect()
}
