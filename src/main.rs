use std::{time::Duration, sync::{Arc, atomic::{AtomicU64, Ordering}, Mutex}};
use clap::Parser;
use futures::future::join_all;
use reqwest::{header::HeaderMap, Body};
use tokio::{self, time::Instant};

mod outputs;
mod headers;

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

    /// HTTP Method
    #[clap(short, long, value_parser, default_value = "get")]
    method: String,

    /// Headers (key=value)
    #[clap(short, long, value_parser)]
    headers: Vec<String>,

    /// JSON Body
    #[clap(short, long, value_parser)]
    body: String,
}

// Delay between queries, we take delay between two query in a same second (1/x) then we converts
// to ms
fn delay_between_queries(query_per_second: usize, request_count: usize) -> Duration {
    return Duration::from_millis(
        ((1f64 / query_per_second as f64 * 1000f64) as u64) * request_count as u64
    )
}

// Delay in the async request to prevent pausing the main thread/tokio runtime in the main function
async fn send_query(
    url: &String,
    delay_before_query: Duration,
    results: Arc<outputs::Results>,
    method: &String,
    header_map: HeaderMap,
    body: &String
) {
    tokio::time::sleep(delay_before_query).await;
    let client = reqwest::Client::new();
    let start = Instant::now();

    let request = match method.to_lowercase().as_str() {
        "get" => client.get(url),
        "post" => client.post(url),
        "patch" => client.patch(url),
        "delete" => client.delete(url),
        "put" => client.put(url),
        _ => panic!("Invalid method, availables: GET/POST/PATCH/DELETE/PUT")
    }.headers(header_map)
    .body(Body::from(body.clone()));

    match request.send().await {
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
    let header_map = match headers::get_headers(args.headers) {
        Ok(m) => m,
        Err(_) => panic!("Invalid headers format."),
    };
    
    outputs::print_intro();
    outputs::print_execution_time(args.count, args.speed);

    for i in 0..args.count {
        requests.push(
            send_query(
                &args.url,
                delay_between_queries(args.speed, i),
                results.clone(),
                &args.method,
                header_map.clone(),
                &args.body
            )
        );
    }
    join_all(requests).await;

    outputs::print_results(results);
}
