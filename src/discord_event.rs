use serenity::model::guild::ScheduledEvent;
use serenity::model::id::GuildId;
use serenity::builder::CreateScheduledEvent;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};

use crate::Error;
use crate::Context;
use crate::tournament_event::TournamentEvent;

pub async fn post_event(ctx: Context<'_>, builder: CreateScheduledEvent<'_>) -> Result<ScheduledEvent, Error> {
    
    let guild_id: GuildId = ctx.guild_id().ok_or("called outside guild.")?;
    let ready_event: ScheduledEvent = guild_id.create_scheduled_event(ctx, builder).await?;
    Ok(ready_event)
}

fn should_skip(event: &TournamentEvent, active_events: &Vec<ScheduledEvent>) -> bool {
    if event.is_already_imported(active_events) {
            warn!("skipping {}; already scheduled", event);
            false
        } else if event.is_past() {
            warn!("skipping {}; event is in the past.", event);
            false
        } else {
            info!("importing {}", event);
            true
        }
}

///returns the number of events scheduled
pub async fn schedule_events(ctx: Context<'_>, fab_events: &Vec<TournamentEvent>) -> Result<usize, Error> {

    let guild_id: GuildId = ctx.guild_id().ok_or("called outside guild.")?;
    let active_events = guild_id.scheduled_events(ctx, false).await?;

    debug!("{:?}", active_events);

    let mut event_stream = tokio_stream::iter(fab_events);
    
    let mut i = 0;
    while let Some(fab_event) = event_stream.next().await {

        if should_skip(fab_event, &active_events) { continue; }

        if let Err(schedule_error) = post_event(ctx, fab_event.into()).await {
            error!(schedule_error);
            continue;
        }
            
        info!("scheduled event: {}", fab_event);
        i += 1;
    }
    Ok(i)
}