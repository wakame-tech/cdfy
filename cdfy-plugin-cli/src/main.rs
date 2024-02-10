use anyhow::Result;
use api::plugin::{CreatePlugin, Plugin};
use clap::Parser;

mod api;

#[derive(Debug, clap::Parser)]
enum Cli {
    List,
    Create(CreateArgs),
    Delete(DeleteArgs),
}

#[derive(Debug, clap::Parser)]
struct CreateArgs {
    #[arg(long)]
    author: String,
    #[arg(long)]
    repo: String,
    #[arg(long)]
    version: String,
}

impl CreateArgs {
    pub fn github_release_url(&self) -> String {
        format!(
            "https://github.com/{}/{}/releases/download/v{}/{}.wasm",
            self.author, self.repo, self.version, self.repo
        )
    }
}

#[derive(Debug, clap::Parser)]
struct DeleteArgs {
    #[arg(long)]
    id: String,
}

fn main() -> Result<()> {
    let origin = "http://localhost:4000/api";
    let args = Cli::try_parse()?;
    let client = reqwest::blocking::Client::new();
    let repo = api::Repo::<Plugin, CreatePlugin>::new(&client, origin, "plugins", "plugin");
    match args {
        Cli::List => {
            let plugins = repo.index()?;
            for plugin in plugins {
                println!("{} {} {}", plugin.id, plugin.title, plugin.version);
            }
        }
        Cli::Create(args) => {
            let wasm_url = args.github_release_url();
            let res = reqwest::blocking::get(&wasm_url)?;
            if res.status() == reqwest::StatusCode::NOT_FOUND {
                anyhow::bail!("Release not found");
            }
            println!("{} {}", res.status(), res.url());
            let plugin = CreatePlugin {
                title: args.repo,
                version: args.version,
                url: wasm_url,
            };
            let plugin = repo.create(&plugin)?;
            println!("{} {} {}", plugin.id, plugin.title, plugin.version);
        }
        Cli::Delete(args) => {
            repo.delete(&args.id)?;
            println!("Deleted {}", args.id);
        }
    }
    Ok(())
}
