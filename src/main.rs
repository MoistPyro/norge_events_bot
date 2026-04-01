use std::env;
use poise::serenity_prelude::{GatewayIntents, ClientBuilder};

use crate::fab_event::{City, format_fab_events};

mod fab_event;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![event()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();

    client.start().await.unwrap();
}

#[poise::command(slash_command, prefix_command, subcommands("oslo", "stavanger", "drammen"))]
async fn event(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("Select a city");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, aliases("Oslo"))]
async fn oslo(ctx: Context<'_>) -> Result<(), Error> {
    generic_city_cmd(ctx, &City::Oslo).await
}

#[poise::command(slash_command, prefix_command, aliases("Stavanger"))]
async fn stavanger(ctx: Context<'_>) -> Result<(), Error> {
    generic_city_cmd(ctx, &City::Stavanger).await
}

#[poise::command(slash_command, prefix_command, aliases("Drammen"))]
async fn drammen(ctx: Context<'_>) -> Result<(), Error> {
    generic_city_cmd(ctx, &City::Drammen).await
}

async fn generic_city_cmd(ctx: Context<'_>, city: &City) -> Result<(), Error> {
    let response = format_fab_events(city).await?;

    let temp = response.join("\r\n\r\n");
    ctx.say(temp).await?;
    Ok(())
}