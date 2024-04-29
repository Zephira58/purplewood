use rand::{self, Rng};
use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    colour::magenta!(" highroll,");
    CreateCommand::new("highroll").description("Highroll the discord bot and see who wins")
}

pub fn run(mut player: i32, mut cpu: i32) -> String {
    if player <= 0 || cpu <= 0 {
        cpu = rand::thread_rng().gen_range(0..100);
        player = rand::thread_rng().gen_range(0..100);
    }

    match cpu.cmp(&player) {
        std::cmp::Ordering::Greater => {
            format!("You lost with value: {:?} vs {:?}", player, cpu)
        }
        std::cmp::Ordering::Equal => {
            format!("You tied with value: {:?} vs {:?}", player, cpu)
        }
        std::cmp::Ordering::Less => {
            format!("You won with value: {:?} vs {:?}", player, cpu)
        }
    }
}
