use std::error::Error;
use std::fmt::{Debug, Display, Pointer};
use std::str::FromStr;
use poise::{async_trait, CommandParameterChoice, CreateReply, PopArgument, SlashArgError, SlashArgument};
use rand::{ Rng};
use serenity::all::{Color, CommandInteraction, Context, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, Message, ResolvedValue};
use crate::basic_functions::random_choice;
use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};


#[derive(Debug,Eq, PartialEq,Clone,Copy)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug,Eq, PartialEq,Clone,Copy)]
pub enum RPSActionResult{
    Win,
    Loss,
    Tie
}
#[async_trait]
impl SlashArgument for RPSChoice {
    async fn extract(ctx: &Context, interaction: &CommandInteraction, value: &ResolvedValue<'_>) -> Result<Self, SlashArgError> {
        match value{
            ResolvedValue::String(gotten) => {
                match RPSChoice::from_str(gotten) {
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
        builder.add_string_choice("Rock", "rock")
            .add_string_choice("Paper", "paper")
            .add_string_choice("Scissors","scissors")
    }

}
#[async_trait]
impl<'a> PopArgument<'a> for RPSChoice {
    async fn pop_from(args: &'a str, attachment_index: usize, ctx: &Context, msg: &Message) -> Result<(&'a str, usize, Self), (Box<dyn Error + Send + Sync>, Option<String>)> {
        let mut parts = args.splitn(2, ' ');
        let choice_str = parts.next().ok_or_else(|| {
            (
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No arguments provided"))
                    as Box<dyn std::error::Error + Send + Sync>,
                Some("You must specify rock, paper, or scissors.".to_string()),
            )
        })?;

        match choice_str.parse::<RPSChoice>() {
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
impl RPSChoice {
    fn versus(self,other: RPSChoice) -> RPSActionResult{
        match self {
            RPSChoice::Rock => {
                match other {
                    RPSChoice::Rock => return RPSActionResult::Tie,
                    RPSChoice::Paper => return RPSActionResult::Loss,
                    RPSChoice::Scissors => return RPSActionResult::Win,
                }
            }
            RPSChoice::Paper =>  match other {
                RPSChoice::Rock => return RPSActionResult::Win,
                RPSChoice::Paper => return RPSActionResult::Tie,
                RPSChoice::Scissors => return RPSActionResult::Loss,
            }
            RPSChoice::Scissors => {
                match other {
                    RPSChoice::Rock => return RPSActionResult::Loss,
                    RPSChoice::Paper => return RPSActionResult::Win,
                    RPSChoice::Scissors => return RPSActionResult::Tie,
                }
            }
        }
    }
}
impl Display for RPSChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for RPSChoice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rock" => Ok(RPSChoice::Rock),
            "paper" => Ok(RPSChoice::Paper),
            "scissors" => Ok(RPSChoice::Scissors),
            _ => Err(()),
        }
    }
}


static CUSTOM_DATA: AdditionalCommandDetails =
    AdditionalCommandDetails::new(CommandType::Game);
#[poise::command(slash_command, prefix_command, custom_data = CUSTOM_DATA.clone())]
/**
showrr
*/
pub async fn rps(
    ctx: ContextToUse<'_>,#[description = "Idk"] choice: RPSChoice )-> CommandRetType {


    let mut embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(ctx.author().name.clone())
            .icon_url(ctx.author().avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url())))
        .title("Rock, Paper, Scissors!!")
        .color(Color::BLUE);


    let random_choice = random_choice([RPSChoice::Rock, RPSChoice::Paper, RPSChoice::Scissors].iter())
        .expect("this usually should work");


    let mut confront_text = "I winn >:)";

    match   random_choice.versus(choice) {
        RPSActionResult::Win => {}
        RPSActionResult::Loss => confront_text = "I.. lost..?",
        RPSActionResult::Tie => confront_text = "It's a tie. seems like we have the same power level"
    }
    embed = embed
        .description("I chose: **".to_owned() + random_choice.to_string().as_str()
            + "**\nYou chose: **" + choice.to_string().as_str() + "**\n" + confront_text);

    _ =ctx.send(CreateReply{
        embeds: vec![embed],
        ..Default::default()
    }).await;


    Ok(())
}
