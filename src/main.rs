use std::env;

use poise::serenity_prelude::{GatewayIntents, ClientBuilder};

use tracing::info;
use tracing_subscriber::{self, fmt::format};

use crate::commands::*;

mod fab_event;
mod discord_event;
mod commands;

#[derive(Debug)]
struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().expect("Failed to load .env file");

    let fmt_event = format().with_line_number(true);
    tracing_subscriber::fmt().event_format(fmt_event).init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![events(), post(), help()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    info!("starting bot");
    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();

    client.start().await.unwrap();
}