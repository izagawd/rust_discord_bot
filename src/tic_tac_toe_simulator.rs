
use ab_glyph::PxScale;
use image::{Rgba, RgbaImage};
use poise::CreateReply;
use serenity::all::colours::roles::GREEN;
use serenity::all::ButtonStyle::{Danger, Success};
use serenity::all::{ButtonStyle, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateEmbedAuthor, EditMessage, Message, User};
use std::io::Cursor;
use std::mem::take;
use crate::bot::ContextToUse;
use crate::commands::tic_tac_toe::XO;
use crate::functions::basic_functions::{random_choice, GLOBAL_FONT};

pub struct GameSimulator<'a> {

    curr: Option<Message>,
    text_to_show: String,
    area: u8,
    player_one: &'a User,
    player_two_user_id: Option<&'a User>,
    board: [[Option<XO>; 4 ]; 4 ],
    turn_taker: XO,

    context: ContextToUse<'a>
}


const OFFSET: u32 = 100;
// Define static lazy-loaded red background

impl<'a> GameSimulator<'a>{


    const NUMBER_TO_WIN: u8 = 3;
    pub fn new(area: u8,
           user: &'a User, ctx: ContextToUse<'a>) -> GameSimulator<'a>{

        GameSimulator{
            curr: None,
            text_to_show: String::new(),
            area,
            player_one: user,
            player_two_user_id: None,
            board: [[None; 4];  4],
            turn_taker: *random_choice([XO::O,XO::X].iter()).unwrap(),

            context: ctx
        }
    }
    fn check_horizontally(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get(y as usize)
            && let Some(value) = gotten_y.get((x + count) as usize){
            if count == Self::NUMBER_TO_WIN{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == Self::NUMBER_TO_WIN
    }
    fn check_vertically(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && let Some(value) = gotten_y.get(x as usize){
            if count == Self::NUMBER_TO_WIN{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == Self::NUMBER_TO_WIN
    }
    fn check_diagonally_forward(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && let Some(value) = gotten_y.get((x + count) as usize){
            if count == Self::NUMBER_TO_WIN{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == Self::NUMBER_TO_WIN
    }
    fn check_diagonally_backward(&self,x: u8, y: u8,to_check: XO) -> bool{
        let mut count = 0u8;
        while let Some(gotten_y) = self.board.get((y + count) as usize)
            && (x as i8) - (count as i8) >= 0 && let Some(value) = gotten_y.get((x - count) as usize){
            if count == Self::NUMBER_TO_WIN{
                return true
            }
            if *value != Some(to_check){
                return false;
            }
            count += 1;
        }
        count == Self::NUMBER_TO_WIN
    }
    fn check_for_winner(&self) -> Option<XO>{
        for to_check in [XO::X, XO::O]{
            for y in 0..self.area{
                for x in 0..self.area{
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
    pub async fn display_battle_state(&mut self) {


        const IMAGE_NAME : &'static str = "some_image.webp";
        if(self.text_to_show.len() == 0){
            self.text_to_show.push_str("NOTHING")
        }
        let mut embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::from(self.player_one))
            .title("Tic Tac Toe!")
            .description(take(&mut self.text_to_show))
            .attachment(IMAGE_NAME)
            .color(GREEN);

        let size = OFFSET * (self.area as u32);
        let mut tic_tac_image = RgbaImage::from_pixel(size, size,Rgba([128,128,128,255]));

        let mut components = Vec::with_capacity(self.area as usize);
        for y in 0..self.area{
            let mut action_row_vec = Vec::with_capacity(self.area as usize);
            for x in 0..self.area{
                let tic_tac_option = self.board[y as usize][x as usize];
                let mut  button_to_create =
                    CreateButton::new(format!("{x}{y}"))
                .style(ButtonStyle::Primary);



                match tic_tac_option {
                    None => {
                        match self.turn_taker {
                            XO::X => { button_to_create = button_to_create.label("X")}
                            XO::O => { button_to_create = button_to_create.label("O")}
                        }
                    }
                    Some(tic_tac_option) => {
                        match tic_tac_option{
                            XO::X => { button_to_create = button_to_create.label("X")}
                            XO::O => { button_to_create = button_to_create.label("O")}
                        }

                        button_to_create = button_to_create
                            .disabled(true)
                            .style( if tic_tac_option == self.turn_taker {Success} else {Danger});
                        let mut text : &'static str;
                        match tic_tac_option {
                            XO::X => text = "X",
                            XO::O => text = "O",
                        }
                        imageproc::drawing::draw_text_mut(&mut tic_tac_image,
                                                          Rgba([0,0,0,255]),(x as u32 * OFFSET) as i32,
                                                          (y as u32 * OFFSET) as i32,
                                                          PxScale::from(OFFSET as f32),&*GLOBAL_FONT,text);
                    }
                }
                action_row_vec.push(button_to_create);
            }
            components.push(CreateActionRow::Buttons(action_row_vec));
        }





        let mut bytes: Vec<u8> = Vec::new();
        tic_tac_image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::WebP).unwrap();


        let attach =CreateAttachment::bytes(bytes,IMAGE_NAME);
        match self.curr.as_mut() {
            None => {
               let handle = self.context.send(CreateReply{
                    embeds: vec![embed],

                    components: Some(components),
                    ..Default::default()
                }
                    .attachment(attach)).await.unwrap();
                    self.curr = Some(handle.into_message().await.unwrap())
            }
            Some(gotten) => {
                gotten.edit(self.context.http(),
                EditMessage::new()
                    .embed(embed)
                    .remove_all_attachments()
                    .components(components)
                    .new_attachment(attach)
                ).await.unwrap();
            }
        }
    }


    pub async fn start(&mut self) -> Option<XO>{


        loop{

            self.text_to_show.push_str(format!("{}'s turn\n",self.turn_taker.to_string()).as_str());
            if let Some(x) = self.check_for_winner(){
                self.text_to_show.push_str(format!("{} won!",x).as_str());
                self.display_battle_state().await;
                return Some(x)
            } else if self.board
                .iter()
                .take(self.area as usize)
                .all(|x| x.iter().take(self.area as usize).all(|x| x.is_some())){
                self.text_to_show.push_str(format!("No winner").as_str());
                self.display_battle_state().await;
                return None;
            }
            self.display_battle_state().await;


            let player_id = self.player_one.id;
            let interaction = self.curr
                .as_ref()
                .unwrap()
                .await_component_interactions(&self.context.serenity_context().shard)
                .filter(move |x| x.user.id == player_id)

                .await
                .unwrap();
            interaction.defer(self.context.http()).await.unwrap();
            let mut input : String = interaction.data.custom_id;


            let first = input.chars().nth(1).unwrap().to_digit(10).unwrap();
            let second = input.chars().nth(0).unwrap().to_digit(10).unwrap();
            if  self.board[first as usize][second as usize].is_none(){
                self.board[first as usize][second as usize] = Some(self.turn_taker);
            } else{
                self.text_to_show.push_str("Spot already used!\n");
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