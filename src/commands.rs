use tracing::{info, error};

use crate::api_types::City;
use crate::tournament_event::TournamentEvent;
use crate::{Context, Error, discord_event, tournament_event};
use crate::lss_api::ApiResponse;

/// import / schedule events from fabtcg event locator at the choosen city.
#[poise::command(slash_command, prefix_command, required_permissions = "CREATE_EVENTS", on_error = "error_hander")]
pub async fn post(ctx: Context<'_>, #[description = "city:"] city: City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_response(&city).await?;
    let fab_events: Vec<TournamentEvent> = response.get_tournaments();

    let intro = format!("preparing {} events...", fab_events.len());
    info!("{}", intro);
    ctx.say(intro).await?;

    //do the thing
    let i = discord_event::schedule_events(ctx, &fab_events).await?;

    let final_words = format!("scheduled {} events, skipped {}", i, fab_events.len());
    info!("{final_words}");
    ctx.say(final_words).await?;
    Ok(())
}

/// Get a list of Flesh and Blood events in the choosen city.
#[poise::command(slash_command, prefix_command, on_error = "error_hander")]
pub async fn events(ctx: Context<'_>, #[description = "city:"] city: City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_response(&city).await?;
    let fab_events: Vec<TournamentEvent> = response.get_tournaments();
    let lines: Vec<String> = tournament_event::format_fab_events(fab_events);

    let message: String = lines.join("\n");
    ctx.say(message).await?;
    Ok(())
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

async fn error_hander(error: poise::FrameworkError<'_, (), Error>) {
    println!("{}", error);
    error!("{}", error);
}