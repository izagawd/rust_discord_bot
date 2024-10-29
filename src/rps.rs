
use std::fmt::Display;
use std::str::FromStr;
use poise::{CreateReply};
use rand::{ Rng};
use serenity::all::{Color,  CreateEmbed, CreateEmbedAuthor};
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
    ctx: ContextToUse<'_>,#[description = "your rock paper scissors choice"] 
    choice : String )-> CommandRetType {

    let player_choice : RPSChoice;
    let mut embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(ctx.author().name.clone())
            .icon_url(ctx.author().avatar_url()
                .unwrap_or_else(|| ctx.author().default_avatar_url())))
        .title("Rock, Paper, Scissors!!")
        .color(Color::BLUE);
    match  RPSChoice::from_str(choice.as_str()) {
        Ok(gotten) => {
            player_choice = gotten;
        }
        Err(_) => {
            embed = embed.title("Hmm").description("Invalid input");
           _ =ctx.send(CreateReply{
               embeds : vec![embed],
               ..Default::default()
           }).await;
            return Ok(())
        }
    }


    let options = [ RPSChoice::Rock, RPSChoice::Paper, RPSChoice::Scissors];

    let random_choice = options[rand::thread_rng().gen_range(0..options.len())];


    let mut confront_text = "I winn >:)";

    match   random_choice.versus(player_choice) {
        RPSActionResult::Win => {}
        RPSActionResult::Loss => confront_text = "I.. lost..?",
        RPSActionResult::Tie => confront_text = "It's a tie. seems like we have the same power level"
    }
    embed = embed
        .description("I chose: **".to_owned() + random_choice.to_string().as_str()
            + "**\nYou chose: **" + player_choice.to_string().as_str() + "**\n" + confront_text);

    _ =ctx.send(CreateReply{
        embeds: vec![embed],
        ..Default::default()
    }).await;


    Ok(())
}
