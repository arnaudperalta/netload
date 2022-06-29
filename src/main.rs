use clap::Parser;
use reqwest::Client;
use futures::{stream, StreamExt};
use tokio;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target URL
    #[clap(short, long, value_parser)]
    url: String,

    /// Number of times to query the target
    #[clap(short, long, value_parser, default_value_t = 100)]
    count: usize,
}

async fn query_target(url: &String, count: usize) {
    let client = Client::new();

    let urls = vec![url; count];

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(count);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got {} bytes", b.len()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    query_target(&args.url, args.count).await;
}
