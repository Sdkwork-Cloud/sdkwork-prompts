pub fn json_error_response(code: &str, message: &str, status: u16) -> Vec<u8> {
    format!(
        r#"{{"type":"about:blank","title":"{}","status":{},"detail":"{}"}}"#,
        escape_json(code),
        status,
        escape_json(message)
    ).into_bytes()
}

pub fn problem_content_type() -> &'static str {
    "application/problem+json"
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
