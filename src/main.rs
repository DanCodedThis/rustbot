use std::{env, collections::HashMap};
use serde_json::{Result, Value};
use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use redis::{AsyncCommands, RedisError};
use dotenv::dotenv;
//write a guide of usage
const HELP_MESSAGE: &str = "Hello there, Human!";

const HELP_COMMAND: &str = "!help";

const MINE_SKIN: &str = "https://api.mineskin.org/generate/url";
 
struct Bot {
    reqwest: reqwest::Client,
    redis: redis::Client,
}
impl Bot {
    fn new(redis_url: &str) -> Bot {
        Bot {
            reqwest: reqwest::Client::new(),
            redis: redis::Client::open(redis_url).unwrap(),
        }
    }
    pub async fn request(&self, json: HashMap<&str, &str>) -> core::result::Result<String, reqwest::Error> {
        Ok(self.reqwest.post(MINE_SKIN)
                .json(&json)
                .send()
                .await? 
                .text()
                .await?)
    }
    async fn to_stream_string(&self, secret_key: &str, json: &String) -> Result<String> {
        let map: Value = serde_json::from_str(&json)?;
        let mut to_send: String = String::from(secret_key) + " ";
        to_send += &serde_json::to_string(&map["data"]["texture"]["value"])?;
        to_send += " ";
        to_send += &serde_json::to_string(&map["data"]["texture"]["signature"])?;
        Ok(to_send)
    }
    pub async fn send(&self, secret_key: &str, json: &String) -> core::result::Result<String, RedisError> {
        match self.to_stream_string(secret_key, json).await {
            Ok(to_send) => {
                let mut con = self.redis.get_async_connection().await?;
                let _ : () = con.set("ff", to_send).await?;
                let back: String = con.get("ff").await?;
                let _ : () = con.del("ff").await?;
                Ok(back)
            }
            Err(why) => {
                eprintln!("Error connecting to redis: {:?}", why);
                Ok(String::from("\n"))
            }
        }
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
                        //use redis streams* (or just send) to send the any secret_key + value + signature
                        match self.send(any, &res).await {
                            Ok(str) => {
                                println!("{}", str);
                            }
                            Err(why) => {
                                eprintln!("Error getting texture: {:?}", why);
                            }
                        }
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
    dotenv().ok();
    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES; 
    let token = env::var("DISCORD_TOKEN").expect("no such var");
    let redis_url = env::var("REDIS_ENDPOINT").expect("no such var");
    let mut client = Client::builder(&token, intents)
        .event_handler(Bot::new(&redis_url))
        .await
        .expect("Error creating client!");
 
    if let Err(why) = client.start().await {
        eprintln!("Error sending message: {:?}", why);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[tokio::test]
    async fn pass_in_url() {
        dotenv().ok();
        let redis_url = env::var("REDIS_ENDPOINT").expect("no such var");
        let test_url = env::var("TEST_URL").expect("no such var");
        let bot = Bot::new(&redis_url);
        let mut map = HashMap::new();
                map.insert("variant", "classic");
                map.insert("name", "Test");
                map.insert("visibility", "0");
                map.insert("url", &test_url);
                match bot.request(map).await {
                    Ok(res) => {
                        //find value and signature
                        //use redis streams* (or just send) to send the any secret_key + value + signature
                        match bot.send("hey", &res).await {
                            Ok(str) => {
                                assert_eq!(str, "hey \"ewogICJ0aW1lc3RhbXAiIDogMTY5NDE5OTEwNTQ0NiwKICAicHJvZmlsZUlkIiA6ICJiMTM1MDRmMjMxOGI0OWNjYWFkZDcyYWVhYmMyNTQ1MCIsCiAgInByb2ZpbGVOYW1lIiA6ICJUeXBrZW4iLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZDI0ZmJjMjMxN2ZmZGY3YjE1OTIzMGE3Y2UyODVjMGM0ZDMzZmU1ZDdmNjE3MjIzOTI0YjVhNzVkZjZmZjBjNiIKICAgIH0KICB9Cn0=\" \"gY/J84yvBgt9LSVxnG8vaJk+D3Y4n03UHHGU/a+idsJCK7tlUKiJoTP6HNS5Gl4F9JKRB6tKRBBcASfp/aYbgMN2DDAbrcV00utq9u98zqwEt4ZnMVXZuNS95dPkboiTFfuWoR9cz81qAy6ihq3FWnpPB20WNvuXWLPJh45jBWs95A+NTaKHDa8KztcWkJXEq7VXqqmrXfQYnUnEtFL6KggOfwIwi5JJMwuhDFWIrbp9iCa/Xhky/SXtqiSLOxhsyca3VGt+ANHw9P7lotiYksct2UgNbqJfzHxmGKgGfCst8bJWOwFLb99NRMUqPqLJ7TTPpCfiX/FICl/I8aPsJqUIqu70rufhOauG7u3vd1VS6C8dO+MpD4sadGhQ3PfCDH+rXvWkazgLEZo4p4ycq8oo377Mci/gZj7fenqJ6bB2RerGej26NktLQAeI1KzxxwJRfl+HccghGVzlOwQaVwIYr2Q5uJ0WbAgILcrxSY6+tygtY86HPeeKKZg1LQNPykFq20NaTxkaexxWbcOmjfAK2bbCD+VY5NdLe2i8SmWQ9X8C2sVHVYP2Xf/LHvi/qSg6ABSFQaFh9KYOffr06ekcoO7QiR9YuEfXYiLrZUf+gAddGBaRVp1FGqh9scrOHvV0n8sD+huDIKR4cwEDNYp/PG7UV4XXBn1V4xHZ7Q8=\"");
                            }
                            Err(why) => {
                                eprintln!("Error getting texture: {:?}", why);
                            }
                        }
                    }
                    Err(why) => {
                        eprintln!("Error getting a response: {:?}", why);
                    }
            }
    }
}