use std::env;

use poise::serenity_prelude::{GatewayIntents, ClientBuilder};

use tracing::{Level, info};
use tracing_subscriber::{self, fmt::format};

use crate::commands::*;

mod fab_event;
mod discord_event;
mod commands;
mod command_options;

const LOG_LVL: Level = Level::INFO;

#[derive(Debug)]
struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().expect("Failed to load .env file");

    let fmt_event = format().with_line_number(true);
    
    tracing_subscriber::fmt()
        .with_max_level(LOG_LVL)
        .event_format(fmt_event)
        .init();

    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents: GatewayIntents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    info!("enabling intents {:?}", intents);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![events(), post(), help()],
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