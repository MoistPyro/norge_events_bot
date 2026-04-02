use chrono::{Duration, Local};
use serenity::all::{ScheduledEvent, ScheduledEventType};
use serenity::model::id::GuildId;
use serenity::builder::CreateScheduledEvent;
use tokio_stream::StreamExt;
use tracing::{error, info};
use crate::Error;
use crate::Context;
use crate::fab_event::FabEvent;

async fn create_event(ctx: Context<'_>, fab_event: &FabEvent) -> Result<ScheduledEvent, Error> {
    let name = &fab_event.nickname;
    let start_time = fab_event.get_start_time_local();
    let end_time = fab_event.get_start_time_local() + Duration::hours(2);
    let now = Local::now();

    if start_time < now || end_time < now {
        error!("{} is in the past; {} is now.", start_time, now);
        return Err("time is in the past".into());
    }

    let discord_event = CreateScheduledEvent::new(ScheduledEventType::External, name, start_time)
        .end_time(end_time)
        .location(&fab_event.address)
        .description(&fab_event.description);

    let guild_id: GuildId = ctx.guild_id().ok_or("failed to find guild id.")?;

    let ready_event = guild_id.create_scheduled_event(ctx, discord_event).await?;
    Ok(ready_event)
}

pub async fn schedule_events(ctx: Context<'_>, fab_events: Vec<FabEvent>) -> Result<usize, Error> {

    let guild = ctx.guild_id().ok_or("failed to find guild id.")?;
    let active_events = guild.scheduled_events(ctx, false).await?;

    let mut event_stream = tokio_stream::iter(fab_events);
    
    let mut i = 0;
    while let Some(e) = event_stream.next().await {
        if check_sameness(&e, &active_events)? {
            info!("skipping {}", e);
            continue;
        }

        let _ = create_event(ctx, &e).await?;
        info!("scheduled {}", e);
        i += 1;
    }
    Ok(i)
}

fn check_sameness(fab_event: &FabEvent, active_events: &Vec<ScheduledEvent>) -> Result<bool, Error> {

    let exact_matches = active_events.iter()
        .filter(|e| e.start_time.naive_utc() == fab_event.get_start_time_local().naive_utc() && e.name == fab_event.nickname)
        .count();

    Ok(exact_matches != 0)
}