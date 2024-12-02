use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::{Error, Read};
use std::task::Context;
use poise::CreateReply;
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, User, UserId};
use crate::basic_functions::random_choice;
use crate::bot::{AdditionalCommandDetails, CommandRetType, CommandType, ContextToUse};

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
pub struct GameSimulator<'a,const width: usize, const height: usize> {
    player_one: &'a User,
    player_two_user_id: Option<&'a User>,
    board: [[Option<XO>; width ]; height ],
    turn_taker: XO,
    number_to_win: u8,
    context: ContextToUse<'a>
}

impl<'a> GameSimulator<'a,3,3>{
    fn new_basic(user: &'a User, context_to_use: ContextToUse<'a>) -> GameSimulator<'a,3,3>{
        GameSimulator::new(*random_choice([XO::O,XO::X].iter()).unwrap(),
                           3,user,
                           context_to_use)
    }
}
impl<'a,const width: usize, const height: usize> GameSimulator<'a, width,height>{

    fn new(initial_turn_taker: XO, number_to_win: u8,
           user: &'a User, ctx: ContextToUse<'a>) -> GameSimulator<'a,width,height>{

        GameSimulator{
            player_one: user,
            player_two_user_id: None,
            board: [[None; width]; height ],
            turn_taker: initial_turn_taker,
            number_to_win: number_to_win,
            context: ctx
        }
    }
    fn check_horizontally(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get(y as usize)
            && let Some(value) = gotten_y.get((x + count) as usize){
            if count == self.number_to_win{
               return true
            }
            if *value != Some(to_check){
                return false;
            } 
            count += 1;
        }
        count == self.number_to_win
    }
    fn check_vertically(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && let Some(value) = gotten_y.get(x as usize){
            if count == self.number_to_win{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == self.number_to_win
    }
    fn check_diagonally_forward(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && let Some(value) = gotten_y.get((x + count) as usize){
            if count == self.number_to_win{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == self.number_to_win
    }
    fn check_diagonally_backward(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && (x as i8) - (count as i8) >= 0 && let Some(value) = gotten_y.get((x - count) as usize){
            if count == self.number_to_win{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == self.number_to_win
    }
    fn check_for_winner(&self) -> Option<XO>{
        for to_check in [XO::X, XO::O]{
            for y in 0..self.board.len(){
                for x in 0..self.board[y].len(){
                    if self.check_horizontally(x as u8,y as u8,to_check){
                        return Some(to_check)
                    } else if self.check_vertically(x as u8,y as u8,to_check){
                        return Some(to_check)
                    } else if self.check_diagonally_forward(x as u8,y as u8,to_check){
                        return Some(to_check)
                    } else if self.check_diagonally_backward(x as u8,y as u8,to_check){
                        return Some(to_check)
                    }
                }
            }
        }

        None
    }
    pub async fn display_battle_state(&self) {
        let mut to_work_with = String::new();
        for i in self.board.iter(){
            for j in i.iter(){
                if let Some(gotten) = j{
                   to_work_with.push_str(&gotten.to_string());
                    to_work_with.push_str(" ");
                } else{
                    to_work_with.push_str("  ");
                }

            }
            to_work_with.push_str("\n");
        }


        if(to_work_with.replace(" ","").replace("\n","").len() == 0){
            to_work_with = String::from("NOTHING")
        }
        self.context.channel_id().send_message(self.context.http(),
                                                   CreateMessage::new()
                                                       .content(to_work_with)).await.unwrap();
    }


    pub async fn start(&mut self) -> Option<XO>{
        loop{
        
            self.display_battle_state().await;
            if let Some(x) = self.check_for_winner(){
                self.context
                    .channel_id()
                    .say(self.context.http(),
                         format!("{} won!",x) )
                    .await.unwrap();
                ;

                return Some(x);
            }
            self.context
                .channel_id()
                .say(self.context.http(),
                     format!("{}'s turn",self.turn_taker.to_string())  )
                .await.unwrap();
            ;
            let player_one = self.player_one.clone();
            let message = self.context.channel_id()
                .await_reply(self.context.serenity_context().shard.as_ref())
                .filter(move |x| x.author == player_one)
                .await.unwrap();
            let mut input : String = message.content.clone();


            let first = input.chars().nth(1).unwrap().to_digit(10).unwrap();
            let second = input.chars().nth(0).unwrap().to_digit(10).unwrap();
            if  self.board[first as usize][second as usize].is_none(){
                self.board[first as usize][second as usize] = Some(self.turn_taker);
            } else{
                self.context
                    .channel_id()
                    .say(self.context.http(),
                         "Spot already used" )
                    .await.unwrap()
                ;
                continue
            }

            if self.turn_taker == XO::X{
                self.turn_taker = XO::O;
            } else {
                self.turn_taker = XO::X;
            }
        }
    }

}
static CUSTOM_DATA: AdditionalCommandDetails =
    AdditionalCommandDetails::new(CommandType::Game);
#[poise::command(slash_command, prefix_command, custom_data = CUSTOM_DATA.clone())]
/**
Play tic tac toe with ur friend
*/
pub async fn tic_tac_toe(
    ctx: ContextToUse<'_>) -> CommandRetType {

    GameSimulator::new_basic(ctx.author(),ctx).start().await;
    Ok(())
}
