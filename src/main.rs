use std::env;

use serenity::{
async_trait,
model::{channel::Message, gateway::Ready},
prelude::*,
};

const HELP_MESSAGE: &str = "
I am MathWalter

I solve math problems

Need assistance?
=> Post in the <#932031701398458380> channel
";

const HELP_COMMAND: &str = "!help";

const ADD_COMMAND: &str = "!add";
const MATH_COMMAND: &str = "!math";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            //if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
            //println!("Error sending message: {:?}", why);
            //}
            send_message(ctx, msg, HELP_MESSAGE).await;
        }
        else if msg.content.contains(ADD_COMMAND) {
            let add_args = msg.content.replace(ADD_COMMAND, "");
            let mysplit = add_args.split_whitespace();
            let mut sum: f64 = 0.0;
            for number in mysplit {
                sum = sum + number.parse::<f64>().unwrap();
            }
            send_message(ctx, msg, &("Result is: ".to_string() + &sum.to_string())).await;
        }
        else if msg.content.contains(MATH_COMMAND) {
            let spaced_equation = msg.content.replace(MATH_COMMAND, " ");
            let equation = remove_whitespace(spaced_equation);
            let mut prompt = String::new();
            for substring in equation.split(char::is_bracket) {
                prompt.push_str(substring);
                prompt.push_str("\n");
            }
            send_message(ctx, msg, &prompt).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn send_message(ctx: Context, msg: Message, message: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}

fn remove_whitespace(mut s: String) -> String {
    s.retain(|c| !c.is_whitespace());
    s
}

trait Brackets {
    fn is_bracket(self) -> bool;
}

impl Brackets for char {
    fn is_bracket(self) -> bool {
        match self {
            '(' => true,
            ')' => true,
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the env");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
    }
}
