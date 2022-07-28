use serenity::model::id::GuildId;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::gateway::Ready;

use serenity::prelude::*;

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let guild_id = GuildId(100128&6661877022800);

    let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                command.name("ping").description("A ping command")
            })
            .create_application_command(|command| {
            command.name("id").description("Get a user id").create_option(|option| {
                    option
                        .name("id")
                        .description("The user to lookup")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
            })
            .create_application_command(|command| {
                command
                    .name("attachmentinput")
                    .description("Test command for attachment input")
                    .create_option(|option| {
                        option
                            .name("attachment")
                            .description("A file")
                            .kind(CommandOptionType::Attachment)
                            .required(true)
                    })
            })
    })
    .await;

    println!("I now have the following guild slash commands: {:#?}", commands);

    let guild_command = Command::create_global_application_command(&ctx.http, |command| {
        command.name("wonderful_command").description("An amazing command")
    })
    .await;

    println!("I created the following global slash command: {:#?}", guild_command);
}