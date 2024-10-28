use std::any::Any;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::string;
use poise::{async_trait, CreateReply, PopArgument, SlashArgError, SlashArgument};
use serenity::all::audit_log::Action::Member;
use serenity::all::Change::Color;
use serenity::all::{Colour, CommandInteraction, Context, CreateCommandOption, Message, ResolvedValue, User};
use serenity::builder::{CreateEmbed, CreateInteractionResponseMessage};
use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};
use crate::help;
use crate::help::help;


#[derive(Debug,Eq, PartialEq)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
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
    ctx: ContextToUse<'_>,#[description = "your rock paper scissors choice"] 
    choice : String )-> CommandRetType {

    let player_choice : RPSChoice;
    match  RPSChoice::from_str(choice.as_str()) {
        Ok(gotten) => {
            player_choice = gotten;
        }
        Err(_) => {
            panic!("NOO")
        }
    }

    _ =ctx.say("you chose ".to_string() + player_choice.to_string().as_str()).await;

    Ok(())
}
