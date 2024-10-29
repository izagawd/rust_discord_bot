use std::cmp::PartialEq;
use std::fmt::Display;
use poise::async_trait;

use serenity::all::{EventHandler, GatewayIntents};
use serenity::Client;
use serenity::model::error::Error;


pub type CommandRetType = Result<(), Error>;
pub type ContextToUse<'a> = poise::Context<'a, (), Error>;
#[derive(Clone, PartialEq,Eq, Debug)]
pub enum CommandType{
    Other,
    Game
}



impl Display for CommandType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone)]
pub struct AdditionalCommandDetails{
     pub command_type: CommandType,
}

impl AdditionalCommandDetails{
    pub const  fn new(command_type: CommandType) -> AdditionalCommandDetails{
        AdditionalCommandDetails{command_type}
    }
}
impl Default for AdditionalCommandDetails{
    fn default() -> AdditionalCommandDetails{
        AdditionalCommandDetails{command_type: CommandType::Other}
    }
}
pub async fn start() {
    let token ="OTYxNDA1NTYzODgxNzg3Mzky.GsXd_v.rf36oVKhf-1xqZJN1p7a-ZPGcL5Dxnjs2awaow";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let framework = poise::Framework::builder()


        .options(poise::FrameworkOptions {
            commands:  vec![crate::help::help(), crate::ping::ping(),
            crate::rps::rps()],

            prefix_options: poise::PrefixFrameworkOptions{

                prefix: Some(String::from("&")),
                ..Default::default()

            },

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
                Ok(())
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await
            .expect("Err creating client");


    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
struct Handler;
#[async_trait]
impl EventHandler for Handler {

}


