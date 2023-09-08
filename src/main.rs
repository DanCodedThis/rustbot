 use std::{env, collections::HashMap};
 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
 
 const HELP_MESSAGE: &str = "Hello there, Human!";
 
 const HELP_COMMAND: &str = "!help";

 const MINE_SKIN: &str = "https://api.mineskin.org/generate/url";
 
 struct Bot {
    reqwest: reqwest::Client
 }
 #[async_trait]
 impl EventHandler for Bot {
     async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            _any => if !msg.author.bot && !msg.embeds.is_empty() && msg.embeds[0].url.is_some() {
                let mut map = HashMap::new();
                map.insert("variant", "classic");
                map.insert("name", "Test");
                map.insert("visibility", "0");
                map.insert("url", msg.embeds[0].url.as_ref().expect("fffff"));
                let res = self.reqwest.post(MINE_SKIN).json(&map).send().await.expect("fff");
                println!("{}", res);
                if let Err(why) = msg.channel_id.say(&ctx.http, res.text().await.expect("ffff")).await {
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
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    println!("{}", token);
    let mut client = Client::builder(token, intents)
        .event_handler(Bot{reqwest: reqwest::Client::new()})
        .await
        .expect("Error creating client!");
 
    if let Err(why) = client.start().await {
         println!("Client error: {:?}", why);
    }
 }