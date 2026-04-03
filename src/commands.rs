use serenity::all::ScheduledEventId;
use tracing::{info, error};

use crate::api_types::{City, DenmarkCity, NorwayCity, SwedenCity};
use crate::tournament_event::TournamentEvent;
use crate::{Context, Error, discord_event, tournament_event};
use crate::lss_api::ApiResponse;

/// import / schedule events from fabtcg event locator at the choosen city.
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "CREATE_EVENTS",
    guild_only,
    subcommands("nor_post", "swe_post", "den_post"),
    on_error = "error_hander"
)]
pub async fn post(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "norge",
    required_permissions = "CREATE_EVENTS",
    guild_only,
    on_error = "error_hander"
)]
pub async fn nor_post(ctx: Context<'_>, #[description = "city:"] city: NorwayCity) -> Result<(), Error> {
    
    generic_post(ctx, city.into()).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "sverige",
    required_permissions = "CREATE_EVENTS",
    guild_only,
    on_error = "error_hander"
)]
pub async fn swe_post(ctx: Context<'_>, #[description = "city:"] city: SwedenCity) -> Result<(), Error> {
    
    generic_post(ctx, city.into()).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "danmark",
    required_permissions = "CREATE_EVENTS",
    guild_only,
    on_error = "error_hander"
)]
pub async fn den_post(ctx: Context<'_>, #[description = "city:"] city: DenmarkCity) -> Result<(), Error> {
    
    generic_post(ctx, city.into()).await?;
    Ok(())
}

/// Get a list of Flesh and Blood events in the choosen city.
#[poise::command(
    slash_command,
    prefix_command,
    on_error = "error_hander"
)]
pub async fn events(ctx: Context<'_>, #[description = "city:"] city: City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_response(&city).await?;
    let fab_events: Vec<TournamentEvent> = response.get_tournaments();
    let lines: Vec<String> = tournament_event::format_fab_events(fab_events);

    let message: String = lines.join("\n");
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
        Type `/help <command>` for more info on a command.
        You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// delete all events.
#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_EVENTS",
    on_error = "error_hander"
)]
pub async fn purge(ctx: Context<'_>) -> Result<(), Error> {

    let guild = ctx.guild_id().ok_or("failed to find guild id.")?;
    let active_events = guild.scheduled_events(ctx, false).await?;
    let event_ids: Vec<ScheduledEventId> = active_events
        .iter()
        .map(|event| event.id)
        .collect();

    for id in event_ids {
        guild.delete_scheduled_event(ctx, id).await?;
    }
    
    Ok(())
}

async fn error_hander(error: poise::FrameworkError<'_, (), Error>) {
    println!("{}", error);
    error!("{}", error);
}

async fn generic_post(ctx: Context<'_>, city: City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_response(&city).await?;
    let fab_events: Vec<TournamentEvent> = response.get_tournaments();

    let intro = format!("preparing {} events...", fab_events.len());
    info!("{}", intro);
    ctx.say(intro).await?;

    //do the thing
    let i = discord_event::schedule_events(ctx, &fab_events).await?;

    let final_words = format!("scheduled {} events, skipped {}", i, fab_events.len() - i);
    info!("{final_words}");
    ctx.say(final_words).await?;
    Ok(())
}