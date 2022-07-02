use std::{thread, time::Duration};

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

    /// Number of total requests to perform on the API
    #[clap(short, long, value_parser, default_value_t = 1000)]
    count: usize,

    /// Number of query per second
    #[clap(short, long, value_parser, default_value_t = 100)]
    speed: usize,
}

// Delay between queries, we take delay between two query in a same second (1/x) then we converts
// to ms
fn delay_between_queries(query_per_second: f64) -> Duration {
    return Duration::from_millis((1f64 / query_per_second * 1000f64) as u64)
}

async fn query_target(args: &Args) {
    let client = Client::new();
    let urls = vec![args.url.clone(); args.count];
    let delay = delay_between_queries(args.speed as f64);

    // Prepare queries for parallelize calls
    let requests = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                client.get(url).send().await
            }
        })
    .buffer_unordered(args.count);

    // Sending queries
    requests.for_each(|responses| async {
        match responses {
            Err(e) => eprintln!("Got an error: {}", e),
            _ => ()
        }
        thread::sleep(delay);
    }).await;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    query_target(&args).await;
}
