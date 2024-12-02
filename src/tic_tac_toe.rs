use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::{Error, Read};
use crate::basic_functions::random_choice;

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

pub struct GameSimulator{
    board: Vec<Vec<Option<XO>>>,
    turn_taker: XO,
    number_to_win: u8
}

impl Default for GameSimulator{
    fn default() -> GameSimulator{
        GameSimulator::new(3,3,*random_choice([XO::O,XO::X].iter()).unwrap(),
        3)
    }
}
impl GameSimulator{
    fn new(width: u8, height: u8, initial_turn_taker: XO, number_to_win: u8) -> GameSimulator{
        let mut vec = Vec::with_capacity(width as usize);
        vec.resize(width as usize, vec![None; height as usize]);
        GameSimulator{
            board: vec,
            turn_taker: initial_turn_taker,
            number_to_win: number_to_win
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
    pub fn start(&mut self) -> Option<XO>{
        loop{
        
            for i in self.board.iter(){
                for j in i.iter(){
                    if let Some(gotten) = j{
                        print!("{} ",gotten);
                    } else{
                        print!("  ",)
                    }

                }
                println!();
            }
            if let Some(x) = self.check_for_winner(){
                println!("{} won!",x);
                return Some(x);
            }
            println!("{}'s turn",self.turn_taker.to_string());
            let mut input : String = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let first = input.chars().nth(1).unwrap().to_digit(10).unwrap();
            let second = input.chars().nth(0).unwrap().to_digit(10).unwrap();
            if  self.board[first as usize][second as usize].is_none(){
                self.board[first as usize][second as usize] = Some(self.turn_taker);
            } else{
                println!("Spot already used");
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