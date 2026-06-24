pub const ROUTE_MANIFEST_PATH: &str = "sdks/_route-manifests/app-api/sdkwork-router-prompts-app-api.route-manifest.json";
pub const SCHEMA_VERSION: u32 = 1;
pub const MANIFEST_KIND: &str = "sdkwork.route.manifest";
pub const PACKAGE_NAME: &str = "sdkwork-router-prompts-app-api";
pub const SURFACE: &str = "app-api";
pub const OWNER: &str = "sdkwork-prompts";
pub const DOMAIN: &str = "intelligence";
pub const CAPABILITY: &str = "prompts";
pub const API_AUTHORITY: &str = "sdkwork-intelligence-prompts-app-api";
pub const SDK_FAMILY: &str = "sdkwork-intelligence-prompts-app-sdk";
pub const PREFIX: &str = "/app/v3/api";

pub fn manifest_path() -> &'static str {
    ROUTE_MANIFEST_PATH
}

pub fn manifest_metadata() -> ManifestMetadata {
    ManifestMetadata {
        schema_version: SCHEMA_VERSION,
        kind: MANIFEST_KIND,
        package_name: PACKAGE_NAME,
        surface: SURFACE,
        owner: OWNER,
        domain: DOMAIN,
        capability: CAPABILITY,
        api_authority: API_AUTHORITY,
        sdk_family: SDK_FAMILY,
        prefix: PREFIX,
    }
}

#[derive(Debug, Clone)]
pub struct ManifestMetadata {
    pub schema_version: u32,
    pub kind: &'static str,
    pub package_name: &'static str,
    pub surface: &'static str,
    pub owner: &'static str,
    pub domain: &'static str,
    pub capability: &'static str,
    pub api_authority: &'static str,
    pub sdk_family: &'static str,
    pub prefix: &'static str,
}
