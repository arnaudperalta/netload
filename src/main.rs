use std::{time::Duration, sync::{Arc, atomic::{AtomicU64, Ordering}, Mutex}};
use clap::Parser;
use futures::future::join_all;
use tokio::{self, time::Instant};

mod outputs;

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
async fn send_query(url: &String, delay_before_query: Duration, results: Arc<outputs::Results>) {
    tokio::time::sleep(delay_before_query).await;
    let start = Instant::now();
    match reqwest::get(url).await {
        Err(_) => results.error_count.fetch_add(1, Ordering::SeqCst),
        Ok(_) => {
            results.latencies.lock().unwrap().push(start.elapsed().as_millis());
            results.success_count.fetch_add(1, Ordering::SeqCst)
        }
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut requests = Vec::new();
    let results = Arc::new(outputs::Results {
        success_count: AtomicU64::new(0),
        error_count: AtomicU64::new(0),
        latencies: Arc::new(Mutex::new(Vec::new()))
    });
    
    outputs::print_intro();
    outputs::print_execution_time(args.count, args.speed);

    for i in 0..args.count {
        requests.push(send_query(&args.url, delay_between_queries(args.speed, i), results.clone()));
    }
    join_all(requests).await;

    outputs::print_results(results);
}
