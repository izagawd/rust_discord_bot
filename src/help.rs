use poise::CreateReply;
use serenity::all::audit_log::Action::Member;
use serenity::all::Change::Color;
use serenity::all::{Colour, User};
use serenity::builder::{CreateEmbed, CreateInteractionResponseMessage};
use crate::bot::{CommandRetType, ContextToUse};

pub struct Gay{}

#[poise::command(slash_command,
            prefix_command)]
/**
help command
*/
pub async fn help(
    ctx: ContextToUse<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> CommandRetType {


    let reply =CreateReply{

        embeds: vec![
            CreateEmbed::new()
                .title("the super of the super")
                .description("yo")
                .color(Colour::BLUE)
        ],
        reply: true,
        ..Default::default()
    };
    ctx.send(reply).await.unwrap();
    Ok(())
}
fn help_shit() -> String {
    String::from("HELUPU")
}