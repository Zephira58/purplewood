// Stolen from https://github.com/MercuryMer/fartcentral/blob/rust/src/commands/rock.rs to test

use rand::Rng;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

// now before you look, i think i fucked some things up
// gimme a break rust hates me and its scary

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(choice),
        ..
    }) = options.first()
    {
        let mut rng = rand::thread_rng();
        let computer_choice = match rng.gen_range(0..2) {
            0 => "rock",
            1 => "paper",
            2 => "scissors",
            _ => "error",
        };
        match *choice {
            "rock" => match computer_choice {
                "rock" => "It's a tie! [rock]",
                "paper" => "You lose! [paper]",
                "scissors" => "You win! [scissors!]",
                _ => "error",
            },
            "paper" => match computer_choice {
                "rock" => "You win! [rock!]",
                "paper" => "It's a tie! [paper!]",
                "scissors" => "You lose! [scissors]",
                _ => "error",
            },
            "scissors" => match computer_choice {
                "rock" => "You lose! [rock]",
                "paper" => "You win! [paper]",
                "scissors" => "It's a tie! [scissors]",
                _ => "error",
            },
            _ => "Please provide a valid choice",
        }
        .to_string()
    } else {
        "Please provide a valid choice".to_string()
    }
}

pub fn register() -> CreateCommand {
    colour::magenta!(" rockpaper,");
    CreateCommand::new("rockpaper")
        .description("Play rock paper scissors with purplewood!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice",
                "choices: rock, paper, scissors",
            )
            .required(true),
        )
}
