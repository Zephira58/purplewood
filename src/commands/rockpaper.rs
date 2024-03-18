// Stolen from https://github.com/MercuryMer/fartcentral/blob/rust/src/commands/rock.rs to test

use rand::Rng;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

pub fn register() -> CreateCommand {
    CreateCommand::new("rockpaper")
        .description("Play rock paper scissors with purplewood!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "choice",
                "choices: rock, paper, scissors",
            )
            .required(true),
        )
}

pub fn run(player: i32, cpu: Option<i32>) -> String {
    let choices = ["rock", "paper", "scissors"];
    let player_choice = choices[player as usize % 3];
    let cpu_choice = match cpu {
        Some(cpu) => choices[cpu as usize % 3],
        None => {
            let mut rng = rand::thread_rng();
            choices[rng.gen_range(0..3)]
        }
    };

    match (player_choice, cpu_choice) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
            format!(
                "You won with {} against (CPU chose {})",
                player_choice, cpu_choice
            )
        }
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => {
            format!(
                "You lost with {} against (CPU chose {})",
                player_choice, cpu_choice
            )
        }
        _ => {
            format!("You got a tie, You both chose {}", player_choice)
        }
    }
}
