use std::env;

use poise::serenity_prelude::{GatewayIntents, ClientBuilder};

use tracing::{Level, info};
use tracing_subscriber::{self, fmt::format};

use crate::commands::{events, post, help};

mod lss_api;
mod discord_event;
mod tournament_event;
mod commands;
mod structs;

const LOG_LVL: Level = Level::INFO;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().expect("expected a .env file in my folder");
    
    let fmt_event = format().with_line_number(false);
    
    tracing_subscriber::fmt()
    .with_max_level(LOG_LVL)
    .event_format(fmt_event)
    .init();

let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
let intents: GatewayIntents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_SCHEDULED_EVENTS;

let framework = poise::Framework::builder()
.options(poise::FrameworkOptions {
    commands: vec![post(), events(), help()],
    ..Default::default()
})
.setup(|ctx, ready, framework| {
    Box::pin(async move {
        info!("Logged in as {}", ready.user.name);
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(())
    })
})
.build();

info!("starting bot");
let mut client = ClientBuilder::new(token, intents)
.framework(framework)
.await.unwrap();

client.start().await.unwrap();
}

#[cfg(test)]
mod tests;