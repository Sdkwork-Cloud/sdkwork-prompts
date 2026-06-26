pub fn json_response(_status: u16, body: &[u8]) -> Vec<u8> {
    body.to_vec()
}

pub fn json_error_response(code: &str, message: &str, status: u16) -> Vec<u8> {
    format!(
        r#"{{"type":"about:blank","title":"{}","status":{},"detail":"{}"}}"#,
        escape_json(code),
        status,
        escape_json(message)
    ).into_bytes()
}

pub fn empty_success() -> Vec<u8> {
    r#"{"success":true}"#.as_bytes().to_vec()
}

pub fn success_with_id(id: i64, uuid: &str) -> Vec<u8> {
    format!(r#"{{"success":true,"id":{},"uuid":"{}"}}"#, id, escape_json(uuid)).into_bytes()
}

pub fn paginated_response(items_json: &str, next_cursor: Option<&str>, has_more: bool) -> Vec<u8> {
    format!(
        r#"{{"items":[{}],"nextCursor":{},"hasMore":{}}}"#,
        items_json,
        next_cursor.map(|c| format!("\"{}\"", escape_json(c))).unwrap_or_else(|| "null".to_string()),
        has_more
    ).into_bytes()
}

pub fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
