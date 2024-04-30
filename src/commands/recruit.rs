use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    recruiter_id: i64,
) -> Result<(), serenity::Error> {
    // Get the current time as a Duration since the Unix epoch
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!");

    // Convert the Duration to Unix time (seconds since the Unix epoch)
    let unix_time = duration.as_secs();

    let modal = CreateQuickModal::new("Who did we recruit?")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Recruit Discord ID")
        .short_field("Recruit SteamID")
        .short_field("Have they been trained?");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (recruit_id, recruit_steamid, is_trained) = (&inputs[0], &inputs[1], &inputs[2]);

    // Check if the recruit_id is a valid integer
    if recruit_id.parse::<i64>().is_err() {
        response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("The recruits Discord ID must be an integer (a number)")
                        .flags(InteractionResponseFlags::EPHEMERAL),
                ),
            )
            .await?;
        return Ok(());
    }

    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!("**Recruited:** {} ({recruit_steamid})\n**Recruiter:** {}\n**Date Recruited:** <t:{:?}>\n**Is Trained:** {is_trained}", discord_id_wrapper(recruit_id.to_string()), discord_id_wrapper(recruiter_id.to_string()), unix_time,),
            )),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    colour::magenta!(" recruit,");
    CreateCommand::new("recruit").description("Use this to register a new recruit")
}

pub fn discord_id_wrapper(id: String) -> String {
    format!("<@{}>", id)
}
