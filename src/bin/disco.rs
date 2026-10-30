use anyhow::Result;
use chrono::Local;
use clap::{Args, Parser};
use disco::config::{Config, read_config, write_config};
use disco::discord::DiscordClient;
use tokio::io::AsyncReadExt;
use tokio::{fs, io};

#[derive(Args, Debug)]
struct CatOpt {
    // Print stdin to screen before posting
    #[arg(short, long)]
    tee: bool,
    // Send as snippet
    #[arg(short, long)]
    snippet: bool,
    // Comment to be given when sending as a snippet
    #[arg(short, long, default_value = "")]
    comment: String,
    // Snippet name
    #[arg(short = 'n', long)]
    snippet_name: Option<String>,
}

async fn cat(opt: CatOpt) -> Result<()> {
    let config = read_config().await?;
    let client = DiscordClient::new(config.webhook_url);

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).await?;
    if opt.tee {
        print!("{buffer}");
    }

    if !opt.snippet {
        client.send_text(buffer).await?;
    } else {
        let snippet_name = opt
            .snippet_name
            .unwrap_or_else(|| format!("{}.txt", Local::now().to_rfc3339()));
        client
            .send_file(snippet_name, buffer.into_bytes(), opt.comment)
            .await?;
    }

    Ok(())
}

#[derive(Args, Debug)]
struct SendFileOpt {
    // Path of the file to be sent
    // If specified, the standard input will be ignored
    #[arg(value_name = "FILE_PATH")]
    file_path: String,
    // Comment for file
    #[arg(short, long, default_value = "")]
    comment: String,
}

async fn send_file(opt: SendFileOpt) -> Result<()> {
    let config = read_config().await?;
    let client = DiscordClient::new(config.webhook_url);
    let bytes = fs::read(&opt.file_path).await?;
    client.send_file(opt.file_path, bytes, opt.comment).await?;
    Ok(())
}

#[derive(Args, Debug)]
struct ConfigureOpt {
    // Discord webhook url
    #[arg(short, long)]
    webhook_url: String,
}

async fn configure(opt: ConfigureOpt) -> Result<()> {
    let config = Config {
        webhook_url: opt.webhook_url,
    };
    write_config(config).await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "disco", version, about)]
enum Opt {
    /// Pipe command output.
    Cat(CatOpt),
    /// Send existing file.
    SendFile(SendFileOpt),
    /// Setup
    Configure(ConfigureOpt),
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::parse();
    match opt {
        Opt::Cat(opt) => cat(opt).await,
        Opt::SendFile(opt) => send_file(opt).await,
        Opt::Configure(opt) => configure(opt).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Opt::command().debug_assert();
    }

    #[test]
    fn existing_cat_options_are_preserved() {
        let Opt::Cat(opt) =
            Opt::try_parse_from(["disco", "cat", "-t", "-s", "-c", "comment", "-n", "log.txt"])
                .unwrap()
        else {
            panic!("expected cat command");
        };
        assert!(opt.tee);
        assert!(opt.snippet);
        assert_eq!(opt.comment, "comment");
        assert_eq!(opt.snippet_name.as_deref(), Some("log.txt"));
    }

    #[test]
    fn existing_commands_accept_their_arguments() {
        let Opt::SendFile(opt) =
            Opt::try_parse_from(["disco", "send-file", "README.md", "--comment", "comment"])
                .unwrap()
        else {
            panic!("expected send-file command");
        };
        assert_eq!(opt.file_path, "README.md");
        assert_eq!(opt.comment, "comment");
        let Opt::Configure(opt) =
            Opt::try_parse_from(["disco", "configure", "-w", "https://example.com/webhook"])
                .unwrap()
        else {
            panic!("expected configure command");
        };
        assert_eq!(opt.webhook_url, "https://example.com/webhook");
        assert!(Opt::try_parse_from(["disco", "configure"]).is_err());
        assert!(Opt::try_parse_from(["disco", "send-file"]).is_err());
    }
}
