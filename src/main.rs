use std::{time::Duration, sync::{Arc, atomic::{AtomicU64, Ordering}}};
use clap::Parser;
use futures::future::join_all;
use tokio;
use colored::*;

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

struct Results {
    success_count: AtomicU64,
    error_count: AtomicU64,
    latencies: Vec<u64>
}

// Delay between queries, we take delay between two query in a same second (1/x) then we converts
// to ms
fn delay_between_queries(query_per_second: usize, request_count: usize) -> Duration {
    return Duration::from_millis(
        ((1f64 / query_per_second as f64 * 1000f64) as u64) * request_count as u64
    )
}

// Delay in the async request to prevent pausing the main thread/tokio runtime in the main function
async fn send_query(url: &String, delay_before_query: Duration, results: Arc<Results>) {
    tokio::time::sleep(delay_before_query).await;
    match reqwest::get(url).await {
        Err(_) => results.error_count.fetch_add(1, Ordering::SeqCst),
        Ok(_) => results.success_count.fetch_add(1, Ordering::SeqCst)
    };
}

fn print_results(results: Arc<Results>) {
    println!("Results:");
    println!("Success: {}", results.success_count.load(Ordering::SeqCst).to_string().green());
    println!("Errors: {}", results.error_count.load(Ordering::SeqCst).to_string().red());
}

fn print_execution_time(request_count: usize, requests_per_second: usize) {
    println!(
        "The execution will takes approximatly {} seconds.",
        (request_count / requests_per_second).to_string().blue()
    );
    println!("The duration can vary with your CPU if you gave an high speed argument.");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut requests = Vec::new();
    let results = Arc::new(Results {
        success_count: AtomicU64::new(0),
        error_count: AtomicU64::new(0),
        latencies: Vec::new()
    });

    print_execution_time(args.count, args.speed);

    for i in 0..args.count {
        requests.push(send_query(&args.url, delay_between_queries(args.speed, i), results.clone()));
    }
    join_all(requests).await;

    print_results(results);
}
