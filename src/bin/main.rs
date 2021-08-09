use anyhow::Result;
use chrono::Local;
use disco::config::read_config;
use disco::discord::DiscordClient;
use structopt::StructOpt;
use tokio::io::AsyncReadExt;
use tokio::{fs, io};

#[derive(StructOpt, Debug)]
struct CatOpt {
    // Print stdin to screen before posting
    #[structopt(short, long)]
    tee: bool,
    // Send as snippet
    #[structopt(short, long)]
    snippet: bool,
    // Comment to be given when sending as a snippet
    #[structopt(short, long, default_value = "")]
    comment: String,
    // Snippet name
    #[structopt(short = "n", long)]
    snippet_name: Option<String>,
}

async fn cat(opt: CatOpt) -> Result<()> {
    let config = read_config().await?;
    let client = DiscordClient::new(config.webhook_url);

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).await?;
    if opt.tee {
        print!("{}", &buffer);
    }

    if !opt.snippet {
        client.send_text(buffer).await?;
    } else {
        let snippet_name = opt
            .snippet_name
            .unwrap_or(format!("{}.txt", Local::now().to_rfc3339()));
        client
            .send_file(snippet_name, buffer.into_bytes(), opt.comment)
            .await?;
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
struct SendFileOpt {
    // Path of the file to be sent
    // If specified, the standard input will be ignored
    #[structopt(name = "FILE_PATH")]
    file_path: String,
    // Comment for file
    #[structopt(short, long, default_value = "")]
    comment: String,
}

async fn send_file(opt: SendFileOpt) -> Result<()> {
    let config = read_config().await?;
    let client = DiscordClient::new(config.webhook_url);
    let bytes = fs::read(&opt.file_path).await?;
    client.send_file(opt.file_path, bytes, opt.comment).await?;
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "disco")]
enum Opt {
    /// Pipe command output.
    Cat(CatOpt),
    /// Send existing file.
    SendFile(SendFileOpt),
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Cat(opt) => cat(opt).await,
        Opt::SendFile(opt) => send_file(opt).await,
    }
}
