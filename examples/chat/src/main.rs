use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

use std::io::{self, Write};

use dotenv::dotenv;

pub fn initialize() {
    dotenv().ok();
}

fn process_input(input: &str) -> String {
    // Perform processing logic on the input string
    // Replace this with your own processing logic
    input.to_uppercase()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    initialize();
    let client=Client::new();

    let mut messages = vec![
    ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content("You are a helpful assistant.")
        .build()?,
    ];

    loop {
        // Prompt for input
        print!("Enter prompt (or 'quit' to exit): ");
        io::stdout().flush().unwrap();

        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim leading/trailing whitespace and remove newline
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            // Exit the program if the user enters "quit"
            break;
        }

        // Process the input and print the output
        let output = process_input(input);

        messages.push(
            ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(output.to_string())
            .build()?,
        );

        let currentMessage=messages.clone();

        let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages(currentMessage)
        .build()?;
        println!("Processed output: {}", output);
        

         let response = client.chat().create(request).await?;

        println!("\nResponse:\n");
        for choice in response.choices {
            println!(
                "{}: Role: {}  Content: {}",
                choice.index, choice.message.role, choice.message.content
            );
            messages.push(
                ChatCompletionRequestMessageArgs::default()
                .role(choice.message.role)
                .content(choice.message.content)
                .build()?,
            );
        }
    }

    
    // let request = CreateChatCompletionRequestArgs::default()
    //     .max_tokens(512u16)
    //     .model("gpt-3.5-turbo")
    //     .messages([
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::System)
    //             .content("You are a helpful assistant.")
    //             .build()?,
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Who won the world series in 2020?")
    //             .build()?,
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::Assistant)
    //             .content("The Los Angeles Dodgers won the World Series in 2020.")
    //             .build()?,
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("do you know about rapid innovation?")
    //             .build()?,
    //     ])
    //     .build()?;

    // let response = client.chat().create(request).await?;

    // println!("\nResponse:\n");
    // for choice in response.choices {
    //     println!(
    //         "{}: Role: {}  Content: {}",
    //         choice.index, choice.message.role, choice.message.content
    //     );
    // }

    Ok(())
}
