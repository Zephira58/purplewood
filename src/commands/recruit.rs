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
    CreateCommand::new("recruit").description("Use this to register a new recruit")
}

fn discord_id_wrapper(id: String) -> String {
    format!("<@{}>", id)
}

//"**Recruited:** Orangefin (STEAM_0:1:238532748)\n**Recruiter:** 𝖲𝖺𝗈𝗂𝗋𝗌𝖾\n**Date Recruited:** 24/02/20\n**In discord:** True\n**Trained:** True"
//format!("**Recruited:** {recruit_id} ({recruit_steamid})\n**Recruiter:** {recruiter_name}\n**Date Recruited:** {recruited_date}\n**In discord:** {is_trained}\n**Trained:** {is_trained}"),
