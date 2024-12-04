use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};
use crate::tic_tac_toe_simulator::GameSimulator;
use poise::{PopArgument, SlashArgError, SlashArgument};
use sea_orm::prelude::async_trait;
use sea_orm::prelude::async_trait::async_trait;
use serenity::all::{CommandInteraction, CreateCommandOption, Message, ResolvedValue};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum XO {
    X,
    O
}

impl Display for XO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}


impl Display for XOArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
#[derive(Clone,Copy,PartialEq,Debug,Eq)]
enum XOArea{
    Three,
    Four
}
static CUSTOM_DATA: AdditionalCommandDetails =
    AdditionalCommandDetails::new(CommandType::Game);
#[poise::command(slash_command, prefix_command, custom_data = CUSTOM_DATA.clone())]
/**
Play tic tac toe with ur friend
*/
pub async fn tic_tac_toe(
    ctx: ContextToUse<'_>, area: Option<XOArea>) -> CommandRetType {

    let mut area_num: u8;
    match area.unwrap_or(XOArea::Three) {
        XOArea::Three => {
            area_num = 3;
        }
        XOArea::Four => {
            area_num = 4;
        }
    }
    GameSimulator::new(area_num,
                       ctx.author(), ctx).start().await;
    Ok(())
}
impl FromStr for XOArea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "three" => Ok(XOArea::Three),
            "four" => Ok(XOArea::Four   ),
            "3" => Ok(XOArea::Three),
            "4" => Ok(XOArea::Four),
            _ => Err(()),
        }
    }
}
#[async_trait::async_trait]
impl<'a> PopArgument<'a> for XOArea{
    async fn pop_from(args: &'a str, attachment_index: usize, ctx: &serenity::all::Context, msg: &Message) -> Result<(&'a str, usize, Self), (Box<dyn std::error::Error + Send + Sync>, Option<String>)> {
        let mut parts = args.splitn(2, ' ');
        let choice_str = parts.next().ok_or_else(|| {
            (
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No arguments provided"))
                    as Box<dyn std::error::Error + Send + Sync>,
                Some("You must specify rock, paper, or scissors.".to_string()),
            )
        })?;

        match choice_str.parse::<XOArea>() {
            Ok(choice) => {
                let remaining_args = parts.next().unwrap_or("");
                Ok((remaining_args, attachment_index, choice))
            }
            Err(err) => {

                return Err((

                    Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput,"Please use rock, paper, or scissors.")),
                    Some(choice_str.to_string()),
                ))},
        }
    }
}
#[async_trait]
impl SlashArgument for XOArea{
    async fn extract(ctx: &serenity::all::Context, interaction: &CommandInteraction, value: &ResolvedValue<'_>) -> Result<Self, SlashArgError> {
        match value{
            ResolvedValue::String(gotten) => {
                match XOArea::from_str(gotten) {
                    Ok(yay) => {
                        return Ok(yay);
                    }
                    Err(_) => {
                        Err(SlashArgError::new_command_structure_mismatch("Invalid choice"))
                    }
                }
            }
            ResolvedValue::Integer(gotten) => {
                match XOArea::from_str(gotten.to_string().as_str()) {
                    Ok(yay) => {
                        return Ok(yay);
                    }
                    Err(_) => {
                        Err(SlashArgError::new_command_structure_mismatch("Invalid choice"))
                    }
                }
            }
            _ =>return  Err(SlashArgError::new_command_structure_mismatch("Parse Issues"))
        }
    }

    fn create(builder: CreateCommandOption) -> CreateCommandOption {
        builder.add_string_choice("Four", "four")
            .add_string_choice("Three", "three")
    }
}