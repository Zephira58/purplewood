use self_update::cargo_crate_version;
use serde_yaml::Value;
use serenity::builder::CreateCommand;
use std::collections::BTreeMap;

pub fn run() -> String {
    let mut info = BTreeMap::new();
    info.insert("Version", Value::String(cargo_crate_version!().to_string()));
    info.insert("Owner", Value::String("Zephira".to_string()));
    info.insert("Email", Value::String("business@zephira.uk".to_string()));
    info.insert(
        "License",
        Value::String("GNU General Public License v3.0".to_string()),
    );
    info.insert(
        "Repo",
        Value::String("https://github.com/Zephira58/purplewood".to_string()),
    );

    let yaml = serde_yaml::to_string(&info).unwrap();

    format!("Heya my name is Zephira and i made this small discord bot to help out my friends on their discord server as a better solution to their ingame recruitment system.\n\nYou can find more of my projects here https://zephira.uk/ alongside my socials if you wish to get in contact ♥```yaml\n{}\n```", yaml)
}

pub fn register() -> CreateCommand {
    colour::magenta!(" credits,");
    CreateCommand::new("credits").description("Who made the bot and what is it about?")
}
