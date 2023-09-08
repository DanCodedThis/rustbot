 use std::env;
 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready, user::User},
     prelude::*,
 };
 
 const HELP_MESSAGE: &str = "Hello there, Human!";
 
 const HELP_COMMAND: &str = "!help";
 
 struct Handler;
 #[async_trait]
 impl EventHandler for Handler {
     async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            inf => if !msg.author.bot {
                if let Err(why) = msg.channel_id.say(&ctx.http, inf).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
     }
 
     async fn ready(&self, _: Context, ready: Ready) {
         println!("{} is connected!", ready.user.name);
     }
 }
 
 #[tokio::main(flavor="current_thread")]
 async fn main() {
    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;
     let token: Vec<String> = env::args().collect();
     println!("{}", token[1]);
     let mut client = Client::builder(&token[1], intents)
         .event_handler(Handler)
         .await
         .expect("Err creating client");
 
     if let Err(why) = client.start().await {
         println!("Client error: {:?}", why);
     }
 }