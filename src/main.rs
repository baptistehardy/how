use std::{env};
use clap::Parser;
use std::error::Error;
use std::io::stdin;
use chrono::prelude::*;
use arboard::Clipboard;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role
    },
    Client,
};

#[derive(Parser, Debug)]
#[command(
    author = "Baptiste Hardy",
    version = "0.1",
    about = "OpenAI-powered CLI code and commands generator"
)]
struct Args {
    #[arg()]
    question: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = Client::new();

    let question = Args::parse()
        .question
        .join(" "); // breaks for languages like japanese

    // TODO: add the possibility to change the default model in the config file
    let model: &str = "gpt-3.5-turbo";
    let date = Local::now().to_string();
    let platform = env::consts::OS;

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(model)
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(format!("You are a coding assistant and trained large language model. \
                Respond with code that is as concisely as possible and do not add comments. \
                Today is {date}."))
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(format!(r#"How {question}? I'm on {platform} CLI. Don't add any comment or explanation. \
                         Your entire answer must be machine executable. Do not add "related" commands."#))
                .build()?,
        ])
        .build()?;

    if !question.is_empty() {
        let response = client.chat().create(request).await?;

        println!("{}", response.choices[0].message.content);
        println!("y/n?");

        // Awful basic user input handling that will *definitely* change
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Unable to read input");
        if answer.starts_with('y') {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(response.choices[0].message.content.to_string()).unwrap();
        }
    } else {
        println!("No question was asked");
    }

    Ok(())
}