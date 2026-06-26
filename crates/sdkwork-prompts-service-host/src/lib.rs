use std::sync::Arc;

use sdkwork_intelligence_prompts_ai_repository_sqlx::SqlxPromptAiRepository;
use sdkwork_prompts_database_host::bootstrap_prompts_database_from_env;
use sqlx::PgPool;
use sdkwork_database_ops::DatabaseOpsService;
use sdkwork_database_spi::{DefaultDatabaseModule, LocaleTag, SeedProfile};
use sdkwork_database_sqlx::DatabasePool;
use tracing;

pub struct PromptsServiceHost {
    ai_repository: SqlxPromptAiRepository,
    pool: DatabasePool,
    pg_pool: PgPool,
    iam_pool: Option<PgPool>,
    database_module: Arc<DefaultDatabaseModule>,
}

impl PromptsServiceHost {
    pub async fn new() -> Self {
        let _ = dotenvy::dotenv();

        tracing::info!("Connecting to database...");

        let database_host = bootstrap_prompts_database_from_env()
            .await
            .expect("Failed to bootstrap prompts database");

        let pool = database_host.pool().clone();
        let database_module = database_host.module();

        let pg_pool = pool
            .as_postgres()
            .expect("Expected PostgreSQL pool for prompts service")
            .clone();

        let iam_pool = if iam_enabled_from_env() {
            Some(load_iam_pool(&pg_pool).await)
        } else {
            None
        };

        tracing::info!("Database connected successfully");

        let ai_repository = SqlxPromptAiRepository::new(pg_pool.clone());

        tracing::info!("Prompts service initialized");

        Self {
            ai_repository,
            pool,
            pg_pool,
            iam_pool,
            database_module,
        }
    }

    pub fn ai_repository(&self) -> &SqlxPromptAiRepository {
        &self.ai_repository
    }

    pub fn database_pool(&self) -> DatabasePool {
        self.pool.clone()
    }

    pub fn postgres_pool(&self) -> &PgPool {
        &self.pg_pool
    }

    pub fn iam_pool(&self) -> Option<&PgPool> {
        self.iam_pool.as_ref()
    }

    pub fn database_module(&self) -> Arc<DefaultDatabaseModule> {
        self.database_module.clone()
    }

    pub fn database_ops_service(&self) -> DatabaseOpsService {
        DatabaseOpsService::new(self.pool.clone(), self.database_module.clone())
    }
}

fn iam_enabled_from_env() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_IAM_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

async fn load_iam_pool(prompts_pool: &PgPool) -> PgPool {
    if let Ok(url) = std::env::var("SDKWORK_PROMPTS_IAM_DATABASE_URL") {
        if !url.trim().is_empty() {
            return PgPool::connect(&url)
                .await
                .expect("Failed to connect SDKWORK_PROMPTS_IAM_DATABASE_URL");
        }
    }
    prompts_pool.clone()
}

pub fn default_seed_locale() -> LocaleTag {
    LocaleTag(
        std::env::var("SDKWORK_PROMPTS_DATABASE_SEED_LOCALE").unwrap_or_else(|_| "zh-CN".to_string()),
    )
}

pub fn default_seed_profile() -> SeedProfile {
    SeedProfile(
        std::env::var("SDKWORK_PROMPTS_DATABASE_SEED_PROFILE")
            .unwrap_or_else(|_| "standard".to_string()),
    )
}
