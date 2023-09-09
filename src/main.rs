use std::{env, collections::HashMap};
use serde_json::{Result, Value};
use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
 //write a guide of usage
 const HELP_MESSAGE: &str = "Hello there, Human!";
 
 const HELP_COMMAND: &str = "!help";

 const MINE_SKIN: &str = "https://api.mineskin.org/generate/url";
 
 struct Texture;
 impl Texture {
    fn send(json: &String) -> Result<()> {
        let map: Value = serde_json::from_str(&json)?;
        println!("{} {}", map["data"]["texture"]["value"], map["data"]["texture"]["signature"]);
        Ok(())
    }
 }
 struct Bot {
    reqwest: reqwest::Client,
    //redis: redis::Client,
 }
 impl Bot {
    fn new(/*redis_url: &str*/) -> Bot {
        Bot {
            reqwest: reqwest::Client::new(),
            //redis: redis::Client::open(redis_url).unwrap(),
        }
    }
    async fn request(&self, json: HashMap<&str, &str>) -> core::result::Result<String, reqwest::Error> {
        Ok(self.reqwest.post(MINE_SKIN)
                .json(&json)
                .send()
                .await? 
                .text()
                .await?)
    }
 }
 #[async_trait]
 impl EventHandler for Bot {
     async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    eprintln!("Error sending message: {:?}", why);
                }
            }
            any => if !msg.author.bot && !msg.attachments.is_empty() {
                let mut map = HashMap::new();
                map.insert("variant", "classic");
                map.insert("name", "Test");
                map.insert("visibility", "0");
                map.insert("url", msg.attachments[0].url.as_str());
                match self.request(map).await {
                    Ok(res) => {
                        //find value and signature
                        if let Err(why) = Texture::send(&res) {
                            eprintln!("Error getting texture: {:?}", why);
                        }
                        //use redis streams to send the any secret_key + value + signature
                        println!("{}", any);
                    }
                    Err(why) => {
                        eprintln!("Error getting a response: {:?}", why);
                    }
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
    //let redis_url = &args[2];
    let mut client = Client::builder(token, intents)
        .event_handler(Bot::new(/*redis_url*/))
        .await
        .expect("Error creating client!");
 
    if let Err(why) = client.start().await {
        eprintln!("Error sending message: {:?}", why);
    }
 }

 #[cfg(test)]
 mod tests {
    #[test]
    fn pass_in_many_urls() {
        assert!(true)
    }
 }