use tracing::info;

use crate::structs::{City, DenmarkCity, NorwayCity, SwedenCity};
use crate::Context;
use crate::Error;
use crate::lss_api::ApiResponse;
use crate::tournament_event::TournamentEvent;
use crate::discord_event;
use super::error_hander;

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
    
    generic_post(ctx, city).await?;
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
    
    generic_post(ctx, city).await?;
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
    
    generic_post(ctx, city).await?;
    Ok(())
}

async fn generic_post(ctx: Context<'_>, city: impl City) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_from_city(&city).await?;
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