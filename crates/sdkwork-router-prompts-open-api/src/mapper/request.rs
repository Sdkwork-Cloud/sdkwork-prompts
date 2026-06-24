pub fn parse_query_params(query: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();
    for param in query.split('&') {
        let mut parts = param.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.push((key.to_string(), value.to_string()));
        }
    }
    params
}

pub fn extract_path_param(path: &str, segment_index: usize) -> Option<String> {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    segments.get(segment_index).map(|s| s.to_string())
}

pub fn extract_site_slug_from_path(path: &str) -> Option<String> {
    extract_path_param(path, 4)
}

pub fn extract_board_id_from_path(path: &str) -> Option<i64> {
    extract_path_param(path, 6).and_then(|s| s.parse().ok())
}

pub fn extract_topic_id_from_path(path: &str) -> Option<i64> {
    extract_path_param(path, 6).and_then(|s| s.parse().ok())
}

pub fn extract_topic_slug_from_path(path: &str) -> Option<String> {
    extract_path_param(path, 7)
}

pub fn get_param(params: &[(String, String)], key: &str) -> Option<String> {
    params.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

pub fn get_param_u16(params: &[(String, String)], key: &str, default: u16) -> u16 {
    get_param(params, key).and_then(|v| v.parse().ok()).unwrap_or(default)
}
