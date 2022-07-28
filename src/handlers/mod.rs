use serenity::model::application::interaction::{Interaction};
use serenity::prelude::*;
use serenity::model::gateway::Ready;

mod ready;
mod interaction;

pub async fn interaction_handler(ctx: Context, interaction: Interaction) {
    interaction::interaction_create(ctx, interaction).await;
}

pub async fn ready(ctx: Context, ready: Ready) 
{
    ready::ready(ctx, ready).await;
}