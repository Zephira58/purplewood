#[test]
fn highroll() {
    let win = crate::commands::highroll::run(50, 32);
    let lose = crate::commands::highroll::run(50, 93);
    let tie = crate::commands::highroll::run(50, 50);

    assert_eq!(win, "You won with value: 50 vs 32");
    assert_eq!(lose, "You lost with value: 50 vs 93");
    assert_eq!(tie, "You tied with value: 50 vs 50");
}

#[test]
fn discord_wrapper() {
    let id = crate::commands::recruit::discord_id_wrapper("292971545956188160".to_owned());
    assert_eq!(id, "<@292971545956188160>")
}

//Todo: Figure out how tf to make recruit.rs/id.rs/rockpaper.rs testable