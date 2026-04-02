use std::{env, str::FromStr};
use tokio_stream::StreamExt;

use poise::serenity_prelude::ScheduledEvent;
use poise::serenity_prelude::{GatewayIntents, ClientBuilder};

use tracing::{info, error};
use tracing_subscriber;

use crate::fab_event::{City, format_fab_events, get_fab_events};

mod fab_event;
mod discord_event;

#[derive(Debug)]
struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_SCHEDULED_EVENTS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![event(), post()],
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

/// import events from event locator.
#[poise::command(slash_command, prefix_command, check = "check", on_error = "error_hander")]
async fn post(ctx: Context<'_>, #[description = "city:"] city: String) -> Result<(), Error> {

    let city: City = City::from_str(&city)?;
    let fab_events = get_fab_events(&city).await?.results;

    let intro = format!("preparing {} events...", fab_events.len());
    info!(intro);
    ctx.say(intro).await?;

    //do the thing
    let i = discord_event::schedule_events(ctx, fab_events).await?;

    let final_words = format!("Successfully scheduled {} events", i);
    info!(final_words);
    ctx.say(final_words).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, subcommands("oslo", "stavanger", "drammen"), on_error = "error_hander")]
async fn event(ctx: Context<'_>) -> Result<(), Error> {

    let response = format!("Select a city");
    ctx.say(response).await?;
    Ok(())
}

/// A list of Tournaments in Oslo.
#[poise::command(slash_command, prefix_command, aliases("Oslo"), on_error = "error_hander")]
async fn oslo(ctx: Context<'_>) -> Result<(), Error> {

    generic_city_cmd(ctx, &City::Oslo).await
}

/// A list of Tournaments in Stavanger.
#[poise::command(slash_command, prefix_command, aliases("Stavanger"), on_error = "error_hander")]
async fn stavanger(ctx: Context<'_>) -> Result<(), Error> {

    generic_city_cmd(ctx, &City::Stavanger).await
}

/// A list of Tournaments in Drammen.
#[poise::command(slash_command, prefix_command, aliases("Drammen"), on_error = "error_hander")]
async fn drammen(ctx: Context<'_>) -> Result<(), Error> {

    generic_city_cmd(ctx, &City::Drammen).await
}

async fn generic_city_cmd(ctx: Context<'_>, city: &City) -> Result<(), Error> {

    let response = format_fab_events(city).await?;

    let temp = response.join("\r\n\r\n");
    ctx.say(temp).await?;
    Ok(())
}

async fn check(ctx: Context<'_>) -> Result<bool, Error> {

    // We discriminate against users starting with an X
    Ok(!ctx.author().name.starts_with('X'))
}

async fn error_hander(error: poise::FrameworkError<'_, Data, Error>) {
    error!("{}", error);
}