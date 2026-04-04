use chrono::{DateTime, Local};
use serenity::model::guild::{ScheduledEvent, ScheduledEventType};
use serenity::model::id::GuildId;
use serenity::builder::CreateScheduledEvent;
use tokio_stream::StreamExt;
use tracing::{error, info, debug, warn};

use crate::Error;
use crate::Context;
use crate::tournament_event::TournamentEvent;

async fn create_event(ctx: Context<'_>, fab_event: &TournamentEvent) -> Result<ScheduledEvent, Error> {

    let name: &str = &fab_event.event_name;
    let start_time: DateTime<Local> = fab_event.start_time;
    let end_time: DateTime<Local> = fab_event.start_time + fab_event.calculate_duration();
    let now: DateTime<Local> = Local::now();

    if start_time < now || end_time < now {
        warn!("{} is in the past. skipping event", start_time);
        return Err("time is in the past".into());
    }

    let discord_event = CreateScheduledEvent::new(ScheduledEventType::External, name, start_time)
        .end_time(end_time)
        .location(&fab_event.address)
        .description(&fab_event.description);

    let guild_id: GuildId = ctx.guild_id().ok_or("failed to find guild id.")?;

    let ready_event: ScheduledEvent = guild_id.create_scheduled_event(ctx, discord_event).await?;
    Ok(ready_event)
}

pub async fn schedule_events(ctx: Context<'_>, fab_events: &Vec<TournamentEvent>) -> Result<usize, Error> {

    let guild = ctx.guild_id().ok_or("failed to find guild id.")?;
    let active_events = guild.scheduled_events(ctx, false).await?;

    active_events.iter().for_each(|e| debug!("{}", e.name));

    let mut event_stream = tokio_stream::iter(fab_events);
    
    let mut i = 0;
    while let Some(fab_event) = event_stream.next().await {

        if fab_event.is_already_imported(&active_events) {
            info!("skipping {}; already scheduled", fab_event);
            continue;
        } else {
            info!("importing {}", fab_event);
        }

        if let Err(schedule_error) = create_event(ctx, fab_event).await {
            error!(schedule_error);
            continue;
        }
            
        info!("scheduled event: {}", fab_event);
        i += 1;
    }
    Ok(i)
}