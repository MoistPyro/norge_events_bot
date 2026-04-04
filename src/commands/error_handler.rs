use crate::Error;
use tracing::error;

pub async fn error_hander(error: poise::FrameworkError<'_, (), Error>) {
    println!("{}", error);
    error!("{}", error);
}