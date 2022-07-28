use serenity::async_trait;
use serenity::model::application::interaction::{Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod commands;
mod handlers;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler 
{
    async fn ready(&self, ctx: Context, ready: Ready) { handlers::ready(ctx, ready).await;}
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {handlers::interaction_handler(ctx, interaction).await;}
}

#[tokio::main]
async fn main() {

    let mut client = Client::builder("MTAwMTI4MDEwMjUwOTgzODQyNw.Goz-oz.PCTutRNkKgaQuscdOQXqzaD1HMKhdH807hLcPE", GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await 
    {
        println!("Client error: {:?}", why);
    }
}