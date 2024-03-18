//Template nicked from https://github.com/serenity-rs/serenity/tree/current/examples/e14_slash_commands

mod commands;
pub mod tests;

use colour::*;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
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
            //Returns the full json body
            //println!("Received command interaction: {command:#?}");

            // Get the current time as a Duration since the Unix epoch
            let duration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards!");

            // Convert the Duration to Unix time (seconds since the Unix epoch)
            let unix_time = duration.as_secs();
            green!("\n\nNew command executed:\n");
            prnt!(
                "    User: {:?}\n    Command: {:?}\n    Unix Timestamp: {:?}",
                &command.member.clone().unwrap().user.name.as_str(),
                &command.data.name.as_str(),
                unix_time
            );
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
        green!("\n{} is connected!\n", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id
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

        print!("I now have the following commands: ");
        magenta!("credits, recruit, highroll, rockpaper");
        prnt!("");
    }
}

#[tokio::main]
async fn main() {
    let _ = env_file_maker();
    dotenv::from_path("./data/.env").unwrap();
    dotenv().ok();

    println!("Hello and welcome to purplewood, this is a small discord bot I, Zephira made to help my friends out");
    println!("And to also learn the serenity framework within rust! ♥");
    print!("Feel free to run the ");
    magenta!("credits");
    prnt!(" command to find out more if your interested\n");
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
            yellow!("Data directory found, reading data...\n");
            prnt!("");
        }
        Err(_) => {
            yellow!("Data directory couldnt be found, creating....\n");
            fs::create_dir_all("./data")?;
            File::create("./data/purplewood-data.db").unwrap();
            let mut file = File::create("./data/.env")?;
            file.write_all(b"# Please add your discord bot token and guildID here\n# -Zephira\nDISCORD_TOKEN = ''\nGUILD_ID = ''")?;

            red!("Your .env file didn't exist, we made one for you but to use the bot you'll have to populate the data yourself manually. It should be located at: ./data/.env");
            process::exit(0);
        }
    }

    Ok(())
}
