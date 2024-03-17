use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // Get the current time as a Duration since the Unix epoch
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!");

    // Convert the Duration to Unix time (seconds since the Unix epoch)
    let unix_time = duration.as_secs();

    let modal = CreateQuickModal::new("Who did we recruit?")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Recruit Name")
        .short_field("Your Discord name")
        .short_field("Recruit SteamID")
        .short_field("Are they in the discord?")
        .short_field("Have they been trained?");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (recruit_name, recruiter_name, recruit_steamid, in_discord, is_trained) =
        (&inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]);

    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!("**Recruited:** {recruit_name} ({recruit_steamid})\n**Recruiter:** {recruiter_name}\n**Date Recruited:** <t:{:?}>\n**In Discord**: {in_discord}\n**Is Trained:** {is_trained}", unix_time),
            )),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("recruit").description("Use this to register a new recruit")
}

//"**Recruited:** Orangefin (STEAM_0:1:238532748)\n**Recruiter:** 𝖲𝖺𝗈𝗂𝗋𝗌𝖾\n**Date Recruited:** 24/02/20\n**In discord:** True\n**Trained:** True"
//format!("**Recruited:** {recruit_name} ({recruit_steamid})\n**Recruiter:** {recruiter_name}\n**Date Recruited:** {recruited_date}\n**In discord:** {is_trained}\n**Trained:** {is_trained}"),
