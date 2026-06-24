use axum::http::HeaderMap;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessClaims {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
}

pub fn parse_access_token_header(headers: &HeaderMap) -> Option<AccessClaims> {
    let raw = headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())?;

    parse_access_token(raw)
}

pub fn parse_access_token(raw: &str) -> Option<AccessClaims> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.contains('=') && !trimmed.contains('.') {
        return parse_kv_access_token(trimmed);
    }

    parse_jwt_access_token(trimmed)
}

fn parse_kv_access_token(raw: &str) -> Option<AccessClaims> {
    let mut tenant_id = None;
    let mut organization_id = None;
    let mut user_id = None;

    for segment in raw.split(';') {
        let mut parts = segment.splitn(2, '=');
        let key = parts.next()?.trim();
        let value = parts.next()?.trim();
        match key {
            "tenant_id" => tenant_id = value.parse().ok(),
            "organization_id" => organization_id = value.parse().ok(),
            "user_id" | "sub" => user_id = value.parse().ok(),
            _ => {}
        }
    }

    Some(AccessClaims {
        tenant_id: tenant_id?,
        organization_id: organization_id.unwrap_or(0),
        user_id: user_id?,
    })
}

fn parse_jwt_access_token(raw: &str) -> Option<AccessClaims> {
    let payload = raw.split('.').nth(1)?;
    let decoded = base64_decode(payload)?;
    let claims: Value = serde_json::from_slice(&decoded).ok()?;

    let tenant_id = claim_i64(&claims, &["tenant_id", "tenantId"])?;
    let organization_id = claim_i64(&claims, &["organization_id", "organizationId"]).unwrap_or(0);
    let user_id = claim_i64(&claims, &["user_id", "userId", "sub"])?;

    Some(AccessClaims {
        tenant_id,
        organization_id,
        user_id,
    })
}

fn claim_i64(value: &Value, keys: &[&str]) -> Option<i64> {
    for key in keys {
        if let Some(number) = value.get(*key).and_then(Value::as_i64) {
            return Some(number);
        }
        if let Some(text) = value.get(*key).and_then(Value::as_i64) {
            return Some(text);
        }
        if let Some(text) = value.get(*key).and_then(Value::as_str) {
            if let Ok(number) = text.parse::<i64>() {
                return Some(number);
            }
        }
    }
    None
}

fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let normalized: String = input
        .chars()
        .map(|ch| match ch {
            '-' => '+',
            '_' => '/',
            other => other,
        })
        .collect();
    let padding = (4 - normalized.len() % 4) % 4;
    let padded = format!("{normalized}{}", "=".repeat(padding));
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, padded).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kv_access_token_values() {
        let claims = parse_access_token(
            "tenant_id=42;organization_id=7;user_id=99;session_id=abc",
        )
        .expect("claims");
        assert_eq!(claims.tenant_id, 42);
        assert_eq!(claims.organization_id, 7);
        assert_eq!(claims.user_id, 99);
    }
}
