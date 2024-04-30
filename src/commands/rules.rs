use serenity::builder::CreateCommand;

pub fn run() -> String {
    let rules = "# Rules
    1. **Common Sense**: Apply this to all activities. If unsure, assume it's not okay until confirmed.
    2. **Respect**: No hateful or demeaning behavior. Don't spoil the experience for others.
    3. **No Malice**: Don't harm org mates (raiding, mugging, stealing, etc.). Avoid actions that could get you banned.
    4. **Avoid Harm**: Don't arrest or take hits on org mates without permission. If their loss is greater than your demotion, take the demote.
    5. **Private Issues**: Handle problems privately. Public complaints make everyone look bad. If serious, contact a commanding officer.
    6. **Leniency**: We're often lenient, but don't abuse it. No bickering or tantrums.
    ";
    rules.to_owned()
}

pub fn register() -> CreateCommand {
    colour::magenta!(" rules,");
    CreateCommand::new("rules").description("The discord and sect community rules")
}
