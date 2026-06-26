//! Gateway assembly for sdkwork-prompts (prompts-only HTTP plane).

mod bootstrap;

pub use bootstrap::{assemble_application_router, ApplicationAssembly};

pub fn assembly_route_count() -> usize {
    18
}
