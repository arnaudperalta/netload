use std::time::Duration;

use clap::Parser;
use futures::future::join_all;
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
fn delay_between_queries(query_per_second: usize, request_count: usize) -> Duration {
    return Duration::from_millis(
        ((1f64 / query_per_second as f64 * 1000f64) as u64) * request_count as u64
    )
}

// Delay in the async request to prevent pausing the main thread/tokio runtime in the main function
async fn send_query(url: &String, delay_before_query: Duration) {
    tokio::time::sleep(delay_before_query).await;
    println!("sending");
    match reqwest::get(url).await {
        Err(_) => panic!("Host unreachable."),
        _ => ()
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut requests = Vec::new();

    // Preparing vector of Futures
    for i in 1..args.count {
        requests.push(send_query(&args.url, delay_between_queries(args.speed, i)));
    }
    join_all(requests).await;
}
