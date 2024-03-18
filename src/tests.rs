#![allow(unused_imports)]

use self_update::cargo_crate_version;

use crate::commands;
use crate::commands::credits;
use crate::commands::highroll;
use crate::commands::recruit::discord_id_wrapper;
use crate::commands::rockpaper;
use crate::env_file_maker;

#[test]
fn highroll_test_win() {
    let result = commands::highroll::run(50, 32);
    assert_eq!(result, "You won with value: 50 vs 32")
}

#[test]
fn highroll_test_lose() {
    let result = commands::highroll::run(50, 93);
    assert_eq!(result, "You lost with value: 50 vs 93")
}

#[test]
fn highroll_test_tied() {
    let result = commands::highroll::run(50, 50);
    assert_eq!(result, "You tied with value: 50 vs 50")
}

#[test]
fn credits_test() {
    let credits = format!("Heya my name is Zephira and i made this small discord bot to help out my friends on their discord server as a better solution to their ingame recruitment system.\nYou can find more of my projects here https://zephira.uk/ alongside my socials if you wish to get in contact ♥\n ```yaml\nBotInfo:\n  Version: {:?}\n  Owner: Zephira\n  Email: Zephira58@protonmail.com\n  License: GNU General Public License v3.0\n  Repo: https://github.com/Zephira58/purplewood\n  ```", cargo_crate_version!()).to_string();
    let result = commands::credits::run();
    assert_eq!(credits, result)
}

#[test]
fn discord_wrapper_test() {
    let id = discord_id_wrapper("292971545956188160".to_string());
    assert_eq!(id, "<@292971545956188160>")
}

//Todo: Figure out how tf to make the recruit.rs module testable
/*#[test]
fn recruit_test() {
    todo!()
}*/

#[test]
fn rockpaper_test_win() {
    let result = rockpaper::run(0, Some(2));
    assert_eq!(result, "You won with rock against (CPU chose scissors)")
}

#[test]
fn rockpaper_test_lose() {
    let result = rockpaper::run(0, Some(1));
    assert_eq!(result, "You lost with rock against (CPU chose paper)")
}

#[test]
fn rockpaper_test_tie() {
    let result = rockpaper::run(0, Some(0));
    assert_eq!(result, "You got a tie, You both chose rock")
}
