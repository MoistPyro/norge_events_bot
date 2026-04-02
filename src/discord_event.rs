use chrono::Duration;
use serenity::all::ScheduledEvent;
use serenity::all::ScheduledEventType;
use serenity::model::id::GuildId;
use serenity::builder::CreateScheduledEvent;
use tokio_stream::StreamExt;
use tracing::{debug, info};

use crate::Error;
use crate::Context;
use crate::fab_event::FabEvent;

async fn create_event(ctx: Context<'_>, fab_event: &FabEvent) -> Result<ScheduledEvent, Error> {
    let name = &fab_event.nickname;
    let end_time = fab_event.start_time + Duration::hours(2);

    let discord_event = CreateScheduledEvent::new(ScheduledEventType::External, name, fab_event.start_time)
        .end_time(end_time)
        .location(&fab_event.address)
        .description(&fab_event.description);

    let guild_id: GuildId = ctx.guild_id().ok_or("failed to find guild id.")?;

    let ready_event = guild_id.create_scheduled_event(ctx, discord_event).await?;
    Ok(ready_event)
}

pub async fn schedule_events(ctx: Context<'_>, fab_events: Vec<FabEvent>) -> Result<usize, Error> {
    let mut event_stream = tokio_stream::iter(fab_events);
    
    let mut i = 0;
    while let Some(e) = event_stream.next().await {
        if check_sameness(ctx, &e).await? {
            info!("skipping {}", e);
            continue;
        }

        let _ = create_event(ctx, &e).await?;
        info!("scheduled {}", e);
        i += 1;
    }
    Ok(i)
}

pub async fn check_sameness(ctx: Context<'_> , fab_event: &FabEvent) -> Result<bool, Error> {

    let guild = ctx.guild_id().ok_or("failed to find guild id.")?;
    let active_events = guild.scheduled_events(ctx, false).await?;

    let mut already_scheduled = false;

    for e in active_events {
        let same_name = e.name == fab_event.nickname;
        let same_day = e.start_time.naive_utc() == fab_event.start_time.naive_utc();

        debug!(e.name, same_name, same_day);
        if same_name && same_day { already_scheduled = true };
    }

    Ok(already_scheduled)
}