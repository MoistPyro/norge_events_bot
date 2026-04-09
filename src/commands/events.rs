use poise::CreateReply;
use tracing::info;

use crate::tournament_event::{TournamentEvent, format_fab_events};
use crate::{Context, Error};
use crate::lss_api::ApiResponse;
use crate::structs::{City, DenmarkCity, NorwayCity, SwedenCity};
use super::error_hander;

/// Get a list of Flesh and Blood events in the choosen city. cooldown 10 minutes.
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("nor_events", "swe_events", "den_events"),
    on_error = "error_hander",
    subcommand_required,
)]
pub async fn events(_ctx: Context<'_>) -> Result<(), Error> {

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "norge",
    guild_cooldown = 600,
    on_error = "error_hander",
)]
async fn nor_events(ctx: Context<'_>, #[description = "city:"] city: NorwayCity) -> Result<(), Error> {

    generic_events(ctx, city).await
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "sverige",
    guild_cooldown = 600,
    on_error = "error_hander"
)]
async fn swe_events(ctx: Context<'_>, #[description = "city:"] city: SwedenCity) -> Result<(), Error> {

    generic_events(ctx, city).await
}

#[poise::command(
    slash_command,
    prefix_command,
    rename = "danmark",
    guild_cooldown = 600,
    on_error = "error_hander"
)]
async fn den_events(ctx: Context<'_>, #[description = "city:"] city: DenmarkCity) -> Result<(), Error> {

    generic_events(ctx, city).await
}

async fn generic_events(ctx: Context<'_>, city: impl City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_from_city(&city).await?;
    let fab_events: Vec<TournamentEvent> = response.get_tournaments();
    let message: CreateReply = format_fab_events(fab_events);

    let length_of_message = message.content.as_ref().unwrap().len();
    info!(length_of_message);

    ctx.send(message).await?;
    Ok(())
}