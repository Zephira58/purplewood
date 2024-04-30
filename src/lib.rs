use colour::*;
use dotenv::dotenv;
use std::{
    env,
    fs::{self, File},
    io::Write,
    process,
};

pub fn env_file_maker() -> std::io::Result<()> {
    let x = fs::read_dir("./data");

    match x {
        Ok(_) => {
            if check_debug() {
                yellow!("Data directory found, reading data...\n");
                prnt!("");
            }
        }
        Err(_) => {
            red!("Data directory couldn't be found, creating....\n");
            fs::create_dir_all("./data")?;
            File::create("./data/purplewood-data.db").unwrap();
            let mut file = File::create("./data/.env")?;
            file.write_all(b"# Please add your discord bot token and guildID here\n# -Zephira\nDISCORD_TOKEN = ''\nGUILD_ID = ''\nDEBUG = 'false'")?;

            red!("Your .env file didn't exist, we made one for you but to use the bot you'll have to populate the data yourself manually. It should be located at: ./data/.env");
            process::exit(0);
        }
    }

    Ok(())
}

pub fn check_debug() -> bool {
    // Load the environment variables from the .env file
    dotenv::from_path("./data/.env").unwrap();
    dotenv().ok();

    // Get the DEBUG variable's value
    match env::var("DEBUG") {
        Ok(val) => {
            // Check if the value is "true"
            val.to_lowercase() == "true"
        }
        Err(_) => {
            // If the DEBUG variable does not exist, return false
            false
        }
    }
}

pub fn discord_id_wrapper(id: String) -> String {
    format!("<@{}>", id)
}
