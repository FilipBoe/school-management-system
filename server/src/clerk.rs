use clerk_rs::{
    clerk::Clerk,
    validators::{ jwks::MemoryCacheJwksProvider, rocket::ClerkGuardConfig },
    ClerkConfiguration,
};

use crate::config::Config;

pub fn load(setup_config: &Config) -> ClerkGuardConfig<MemoryCacheJwksProvider> {
    let clerk_config = ClerkConfiguration::new(
        None,
        None,
        Some(setup_config.clerk_secret_key.to_string()),
        None
    );
    let clerk = Clerk::new(clerk_config);

    ClerkGuardConfig::new(MemoryCacheJwksProvider::new(clerk), None, true)
}
