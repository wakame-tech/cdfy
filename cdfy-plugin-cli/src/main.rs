use anyhow::Result;
use clap::Parser;
use minio::s3::args::{GetObjectArgs, PutObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
enum Cli {
    Upload(UploadArgs),
    Download(DownloadArgs),
}

#[derive(Debug, clap::Parser)]
struct UploadArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    path: PathBuf,
}

#[derive(Debug, clap::Parser)]
struct DownloadArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    path: PathBuf,
}

struct S3PluginUploader {
    client: Client,
}

impl S3PluginUploader {
    const BUCKET_NAME: &'static str = "plugins";

    pub fn new(url: &str) -> Result<Self> {
        let access_key = std::env::var("S3_ACCESS_KEY")?;
        let secret_key = std::env::var("S3_SECRET_KEY")?;
        let provider = StaticProvider::new(&access_key, &secret_key, None);

        let base_url = url.parse::<BaseUrl>()?;
        let client = Client::new(base_url, Some(Box::new(provider)), None, None)?;

        Ok(Self { client })
    }

    pub async fn upload(&self, name: &str, path: &PathBuf) -> Result<()> {
        let mut wasm = File::open(path)?;
        let size = wasm.metadata()?.len() as usize;
        let name = format!("{}.wasm", name);
        let res = self
            .client
            .put_object(&mut PutObjectArgs::new(
                Self::BUCKET_NAME,
                &name,
                &mut wasm,
                Some(size),
                None,
            )?)
            .await?;
        dbg!(res.location);
        Ok(())
    }

    pub async fn download(&self, name: &str, path: PathBuf) -> Result<()> {
        let mut wasm = OpenOptions::new().write(true).create(true).open(path)?;
        let name = format!("{}.wasm", name);
        let res = self
            .client
            .get_object(&GetObjectArgs::new(Self::BUCKET_NAME, &name)?)
            .await?;
        wasm.write_all(&res.bytes().await?)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:9000";
    let args = Cli::try_parse()?;
    let uploader = S3PluginUploader::new(url)?;
    match args {
        Cli::Upload(args) => uploader.upload(&args.name, &args.path).await?,
        Cli::Download(args) => uploader.download(&args.name, args.path).await?,
    }
    Ok(())
}
