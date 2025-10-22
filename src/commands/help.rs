use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};
use poise::CreateReply;
use serenity::all::{Colour, CreateEmbedAuthor};
use serenity::builder::CreateEmbed;


static CUSTOM_DATA: AdditionalCommandDetails =
    AdditionalCommandDetails::new(CommandType::Other);

#[poise::command(slash_command,
            prefix_command, custom_data = CUSTOM_DATA.clone())]


/**
help command
*/
pub async fn help(
    ctx: ContextToUse<'_>,
) -> CommandRetType {


    struct NamenDescr<'a>{
        pub name: &'a String,
        pub description: Option<&'a String>,
        pub custom_data : &'a AdditionalCommandDetails,
    }
    static DEFAULT_ADD_COMMS: AdditionalCommandDetails = AdditionalCommandDetails::default();
    let commands_name_descr_mapped = &ctx
        .framework()
        .options()
        .commands
        .iter()
        .map(|x| NamenDescr{
            name: &x.name,
            description: (&x.description).as_ref(),
            custom_data: &x.custom_data
                .downcast_ref::<AdditionalCommandDetails>()
                .unwrap_or(&DEFAULT_ADD_COMMS)
        });

    let mut available_comm_types = commands_name_descr_mapped
        .clone()
        .map(|x| x.custom_data.command_type)
        .collect::<Vec<_>>();
    available_comm_types.dedup();
    let mut embed_to_make = CreateEmbed::new()
        .title("Commands")
        .color(Colour::BLUE)
        .author(CreateEmbedAuthor::new(ctx.author().name.as_str())
            .icon_url(ctx.author().avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url())));
    for i in available_comm_types {
        let commands_under =
        commands_name_descr_mapped
            .clone()
            .filter(|x| x.custom_data.command_type == i)
            .map(|x|
                format!("{}{}",x.name,x.description.map(|y| format!(": {y}")).unwrap_or(String::new())))
            .collect::<Vec<String>>();

        embed_to_make = embed_to_make
            .field(i.to_string(),commands_under.join("\n"),false);
    }
    let reply =CreateReply{

        embeds: vec![embed_to_make],
        reply: true,
        ..Default::default()
    };
    ctx.send(reply).await.unwrap();
    Ok(())
}
