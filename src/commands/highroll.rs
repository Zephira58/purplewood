use rand::{self, Rng};
use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("highroll").description("Highroll the discord bot and see who wins")

}

pub fn run() -> String {
    let cpu = rand::thread_rng().gen_range(0..100);
    let player: i32 = rand::thread_rng().gen_range(0..100);

    if cpu > player {
        return format!("You lost with value: {:?} vs {:?}", player, cpu);
    } else {
        return format!("You won with value: {:?} vs {:?}", player, cpu);
    }
}