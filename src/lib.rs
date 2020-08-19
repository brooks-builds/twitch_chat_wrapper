use anyhow::Context as _;
use dotenv::dotenv;
use std::env;
use twitchchat::UserConfig;

pub fn run() -> anyhow::Result<()> {
    dotenv().ok();
    let user_config = get_user_config()?;
    let channel = get_channel()?;
    Ok(())
}

fn get_user_config() -> anyhow::Result<UserConfig> {
    let twitch_name = get_environment_variable("TWITCH_NICKNAME")?;
    let twitch_password = get_environment_variable("TWITCH_PASSWORD")?;
    let config = UserConfig::builder()
        .name(twitch_name)
        .token(twitch_password)
        .enable_all_capabilities()
        .build()?;
    Ok(config)
}

fn get_environment_variable(name: &str) -> anyhow::Result<String> {
    env::var(name).with_context(|| format!("Environment variable {} doesn't exist", name))
}

fn get_channel() -> anyhow::Result<String> {
    get_environment_variable("TWITCH_CHANNEL")
}
