use self_update::cargo_crate_version;
use serenity::builder::CreateCommand;

pub fn run() -> String {
    format!("Heya my name is Zephira and i made this small discord bot to help out my friends on their discord server as a better solution to their ingame recruitment system.\nYou can find more of my projects here https://zephira.uk/ alongside my socials if you wish to get in contact ♥\n ```yaml\nBotInfo:\n  Version: {:?}\n  Owner: Zephira\n  Email: Zephira58@protonmail.com\n  License: GNU General Public License v3.0\n  Repo: https://github.com/Zephira58/purplewood\n  ```", cargo_crate_version!()).to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("credits").description("Who made the bot and what is it about?")
}
