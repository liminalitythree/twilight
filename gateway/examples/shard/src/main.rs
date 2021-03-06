use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::Shard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let mut shard = Shard::new(env::var("DISCORD_TOKEN")?);
    let mut events = shard.events().await;

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
