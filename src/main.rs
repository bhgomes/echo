use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::{theme::SimpleTheme, Input};
use echo::reqwest::{KnownUrlClient, Url};
use echo::tide;

#[derive(Debug, Subcommand)]
enum Command {
    Client { url: Url },
    Server { url: Url },
}

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(short)]
    jobs: Option<usize>,
    #[command(subcommand)]
    command: Command,
}

impl Arguments {
    fn run(self) -> Result<()> {
        match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.jobs.unwrap_or_else(num_cpus::get))
            .enable_io()
            .enable_time()
            .build()
        {
            Ok(runtime) => match self.command {
                Command::Client { url } => runtime.block_on(async { client(url).await }),
                Command::Server { url } => runtime.block_on(async { server(url).await }),
            },
            _ => todo!(),
        }
    }
}

async fn client(url: Url) -> Result<()> {
    let client = KnownUrlClient::new(url)?;
    loop {
        let input: String = Input::with_theme(&SimpleTheme)
            .with_prompt("")
            .interact_text()
            .unwrap();
        let response: String = client.post("echo", &input).await?;
        println!("< {}", response);
    }
}

async fn echo(_: (), request: String) -> Result<String> {
    println!("INFO: message received {}", request);
    Ok(request)
}

async fn server(url: Url) -> Result<()> {
    let mut app = tide::Server::with_state(());
    tide::register_post(&mut app, "/echo", echo);
    Ok(app.listen(url).await?)
}

fn main() -> Result<()> {
    Arguments::parse().run()
}
