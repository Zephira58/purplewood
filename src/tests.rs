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
fn test_discord_id_wrapper() {
    let id = "123456789".to_string();
    let result = purplewood::discord_id_wrapper(id.clone());
    assert_eq!(result, format!("<@{}>", id));
}

#[test]
fn test_check_debug() {
    // Set the DEBUG environment variable to "true"
    std::env::set_var("DEBUG", "true");
    // Test that check_debug returns true when DEBUG is set to "true"
    assert_eq!(purplewood::check_debug(true), true);

    // Set the DEBUG environment variable to "false"
    std::env::set_var("DEBUG", "false");
    // Test that check_debug returns false when DEBUG is set to "false"
    assert_eq!(purplewood::check_debug(true), false);

    // Remove the DEBUG environment variable
    std::env::remove_var("DEBUG");
    // Test that check_debug returns false when DEBUG is not set
    assert_eq!(purplewood::check_debug(true), false);
}
