use tracing::{info, error};

use crate::{Context, Data, Error, discord_event, fab_event::{City, format_fab_events, get_fab_events}};

/// import (schedule) events from event locator.
#[poise::command(slash_command, prefix_command, check = "check", on_error = "error_hander")]
pub async fn post(ctx: Context<'_>, #[description = "city:"] city: City) -> Result<(), Error> {

    let fab_events = get_fab_events(&city).await?.results;

    let intro = format!("preparing {} events...", fab_events.len());
    info!("{}", intro);
    ctx.say(intro).await?;

    //do the thing
    let i = discord_event::schedule_events(ctx, fab_events).await?;

    let final_words = format!("Successfully scheduled {} events", i);
    info!("{final_words}");
    ctx.say(final_words).await?;
    Ok(())
}

/// Get a list of events in the choosen city.
#[poise::command(slash_command, prefix_command, on_error = "error_hander")]
pub async fn events(ctx: Context<'_>, #[description = "city:"] city: City) -> Result<(), Error> {

    let response = get_fab_events(&city).await?;
    let lines = format_fab_events(response)?;

    let message = lines.join("\r\n");
    ctx.say(message).await?;
    Ok(())
}

async fn check(ctx: Context<'_>) -> Result<bool, Error> {

    // We discriminate against users starting with an X
    Ok(!ctx.author().name.starts_with('X'))
}

async fn error_hander(error: poise::FrameworkError<'_, Data, Error>) {
    println!("{}", error);
    error!("{}", error);
}

#[poise::command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type ?help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}