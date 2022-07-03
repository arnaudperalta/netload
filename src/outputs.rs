use std::sync::{Arc, atomic::{Ordering, AtomicU64}, Mutex};
use colored::*;

pub struct Results {
    pub success_count: AtomicU64,
    pub error_count: AtomicU64,
    pub latencies: Arc<Mutex<Vec<u128>>>
}

pub fn print_results(results: Arc<Results>) {
    let avg_latency = average_latency(results.latencies.lock().unwrap().clone());
    let min_latency = minimum_latency(results.latencies.lock().unwrap().clone());
    let max_latency = maximum_latency(results.latencies.lock().unwrap().clone());
    println!("Results:");
    println!("Success: {}", results.success_count.load(Ordering::SeqCst).to_string().green());
    println!("Errors: {}", results.error_count.load(Ordering::SeqCst).to_string().red());
    println!(
        "Latency (in ms): Average {} / Min {} / Max {}",
        avg_latency,
        min_latency,
        max_latency
    );
}

pub fn print_intro() {
    println!("            _   _                 _ ");
    println!("           | | | |               | |");
    println!(" _ __   ___| |_| | ___   __ _  __| |");
    println!("| '_ \\ / _ \\ __| |/ _ \\ / _` |/ _` |");
    println!("| | | |  __/ |_| | (_) | (_| | (_| |");
    println!("|_| |_|\\___|\\__|_|\\___/ \\__,_|\\__,_|\n");
}

pub fn print_execution_time(request_count: usize, requests_per_second: usize) {
    println!(
        "The execution will takes approximatly {} seconds.",
        (request_count / requests_per_second).to_string().blue()
    );
    println!("The duration can vary with your CPU if you gave an high speed argument.");
}

fn average_latency(latencies: Vec<u128>) -> u128 {
    let total : u128 = latencies.iter().sum();
    return total / latencies.len() as u128;
}

fn minimum_latency(latencies: Vec<u128>) -> u128 {
    return latencies.iter().min().unwrap().clone();
}

fn maximum_latency(latencies: Vec<u128>) -> u128 {
    return latencies.iter().max().unwrap().clone();
}
