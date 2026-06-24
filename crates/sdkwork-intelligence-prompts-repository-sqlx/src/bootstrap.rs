pub use sdkwork_prm_database_host::{
    bootstrap_prompts_database, bootstrap_prompts_database_from_env, PromptsDatabaseHost,
};

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

pub async fn connect_prm_database_pool_from_env() -> Result<DatabasePool, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("PROMPTS")
        .map_err(|error| format!("read forum database config failed: {error}"))?;
    create_pool_from_config(config)
        .await
        .map_err(|error| format!("create forum database pool failed: {error}"))
}

pub async fn connect_and_bootstrap_prompts_database_from_env() -> Result<PromptsDatabaseHost, String> {
    let pool = connect_prm_database_pool_from_env().await?;
    bootstrap_prompts_database(pool).await
}
