//Template nicked from https://github.com/serenity-rs/serenity/tree/current/examples/e14_slash_commands

mod commands;
pub mod tests;

use std::fs::File;
use std::io::Write;
use std::{env, fs, process};

use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "rockpaper" => Some(commands::rockpaper::run(&command.data.options())),
                "recruit" => {
                    commands::recruit::run(&ctx, &command).await.unwrap();
                    None
                }
                "credits" => Some(commands::credits::run()),
                "highroll" => Some(commands::highroll::run(0, 0)),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("\n{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::credits::register(),
                    commands::recruit::register(),
                    commands::highroll::register(),
                    commands::rockpaper::register(),
                ],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }
}

#[tokio::main]
async fn main() {
    let _ = env_file_maker();
    dotenv::from_path("./data/.env").unwrap();
    dotenv().ok();

    println!("Hello and welcome to purplewood, this is a small discord bot I, Zephira made to help my friends out");
    println!("And to also learn the serenity framework within rust! ♥");
    println!("Feel free to run the credits command to find out more if your interested");
    for _x in 0..100 {
        print!("-")
    }

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    let _ = update();
}

use self_update::cargo_crate_version;

fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Zephira58")
        .repo_name("purplewood")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

fn env_file_maker() -> std::io::Result<()> {
    let x = fs::read_dir("./data");

    match x {
        Ok(_) => {
            println!("Directory exists");
        }
        Err(_) => {
            fs::create_dir_all("./data")?;
            let mut file = File::create("./data/.env")?;
            file.write_all(b"# Please add your discord bot token and guildID here\n# -Zephira\nDISCORD_TOKEN = ''\nGUILD_ID = ''")?;

            println!("Your .env file didn't exist, we made one for you but to use the bot you'll have to populate the data yourself manually. It should be located at: ./data/.env");
            process::exit(0);
        }
    }

    Ok(())
}
