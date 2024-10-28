use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};

static CUSTOM_DATA: AdditionalCommandDetails =
    AdditionalCommandDetails::new(CommandType::Other);
#[poise::command(slash_command, prefix_command, custom_data = CUSTOM_DATA.clone())]
/**
showrr
*/
pub async fn ping(
    ctx: ContextToUse<'_>) -> CommandRetType {
   ctx.say("Pong!").await.expect("TODO: panic message");


    Ok(())
}
