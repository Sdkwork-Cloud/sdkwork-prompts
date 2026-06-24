use sha2::{Digest, Sha256};

pub(crate) fn stable_uuid(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    for part in parts {
        hasher.update(b":");
        hasher.update(part.as_bytes());
    }
    let digest = format!("{:x}", hasher.finalize());
    format!("{prefix}-{}", &digest[..40])
}
