use std::vec;

use clap::Parser;
use rs_openai::{
    chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
    OpenAI,
};
use std::io::{stdout, Write};
use tokio_stream::{StreamExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    prompt: String,

    /// Should responses be streamed
    #[clap(long, default_value="true")]
    stream: bool,

    /// The user representing the query
    #[clap(long, default_value="cli-user")]
    user: String,

    /// OpenAI key
    #[clap(long, env)]
    api_key: String,

    /// OpenAI organization
    #[clap(long)]
    api_org: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = OpenAI::new(&OpenAI {
        api_key: cli.api_key,
        org_id: cli.api_org,
    });

    if cli.stream == true {
        let req = CreateChatRequestBuilder::default()
            .model("gpt-3.5-turbo")
            .messages(vec![ChatCompletionMessageRequestBuilder::default()
                .role(Role::System)
                .name(cli.user)
                .content(cli.prompt)
                .build()?])
            .stream(true)
            .build()?;
        
        let mut stream = client.chat().create_with_stream(&req).await?;

        let mut lock = stdout().lock();
    while let Some(response) = stream.next().await {
        response.unwrap().choices.iter().for_each(|choice| {
            if let Some(ref content) = choice.delta.content {
                write!(lock, "{}", content).unwrap();
            }
        });

        stdout().flush()?;
    }
    } else {
        let req = CreateChatRequestBuilder::default()
            .model("gpt-3.5-turbo")
            .messages(vec![ChatCompletionMessageRequestBuilder::default()
                .role(Role::System)
                .name(cli.user)
                .content(cli.prompt)
                .build()?])
            .build()?;

        let res = client.chat().create(&req).await?;
        println!("{}", res.choices[0].message.content);
    }

    Ok(())
}
