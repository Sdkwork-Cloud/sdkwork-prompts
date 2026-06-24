use std::sync::Arc;

use sdkwork_intelligence_prm_repository_sqlx::SqlxPromptsRepository;
use sdkwork_prm_database_host::bootstrap_prompts_database_from_env;
use sqlx::PgPool;
use sdkwork_intelligence_prm_service::PromptsService;
use sdkwork_intelligence_prm_service::value_objects::PromptsRequestContext;
use sdkwork_database_ops::DatabaseOpsService;
use sdkwork_database_spi::{DefaultDatabaseModule, LocaleTag, SeedProfile};
use sdkwork_database_sqlx::DatabasePool;
use tracing;

mod ports;

pub struct PromptsServiceHost {
    service: PromptsService<SqlxPromptsRepository>,
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
            .expect("Failed to bootstrap forum database");

        let pool = database_host.pool().clone();
        let database_module = database_host.module();

        let pg_pool = pool
            .as_postgres()
            .expect("Expected PostgreSQL pool for forum service")
            .clone();

        let iam_pool = if iam_enabled_from_env() {
            Some(load_iam_pool(&pg_pool).await)
        } else {
            None
        };

        tracing::info!("Database connected successfully");

        let repository = SqlxPromptsRepository::new(pg_pool.clone());
        let service = PromptsService::new_with_ports(
            repository,
            ports::build_drive_port(),
            ports::build_search_port(),
            ports::build_notification_port(),
        );

        tracing::info!("Prompts service initialized");

        Self {
            service,
            pool,
            pg_pool,
            iam_pool,
            database_module,
        }
    }

    pub fn service(&self) -> &PromptsService<SqlxPromptsRepository> {
        &self.service
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

    pub fn build_request_context(
        &self,
        tenant_id: i64,
        organization_id: i64,
        user_id: i64,
    ) -> PromptsRequestContext {
        PromptsRequestContext::new(tenant_id, organization_id, user_id)
    }
}

fn iam_enabled_from_env() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_IAM_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

async fn load_iam_pool(prm_pool: &PgPool) -> PgPool {
    if let Ok(url) = std::env::var("SDKWORK_PROMPTS_IAM_DATABASE_URL") {
        if !url.trim().is_empty() {
            return PgPool::connect(&url)
                .await
                .expect("Failed to connect SDKWORK_PROMPTS_IAM_DATABASE_URL");
        }
    }
    prm_pool.clone()
}

pub fn build_prm_service() -> PromptsService<SqlxPromptsRepository> {
    PromptsService::new(SqlxPromptsRepository::new_placeholder())
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
