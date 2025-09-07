use std::env;
use std::time::Instant;
use vibesort::{vibesort, VibesortError};
use rand::Rng;

#[derive(Debug, Clone)]
enum DataType {
    WholeNumbers,
    Floats,
    Strings,
    All,
}

impl std::str::FromStr for DataType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "whole" | "numbers" | "int" => Ok(DataType::WholeNumbers),
            "float" | "floats" => Ok(DataType::Floats),
            "string" | "strings" => Ok(DataType::Strings),
            "all" => Ok(DataType::All),
            _ => Err(format!("Unknown data type: {}", s)),
        }
    }
}

fn generate_random_numbers(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-1000..1000)).collect()
}

fn generate_random_floats(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-1000.0..1000.0)).collect()
}

fn generate_random_strings(n: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..n).map(|_| {
        let len = rng.gen_range(3..10);
        (0..len)
            .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
            .collect()
    }).collect()
}

fn is_sorted<T: PartialOrd>(vec: &[T]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

fn compare_arrays<T: PartialOrd + Clone + std::fmt::Debug>(rust_sorted: &[T], vibe_sorted: &[T]) -> bool {
    if rust_sorted.len() != vibe_sorted.len() {
        println!("❌ Length mismatch: Rust {} vs Vibesort {}", rust_sorted.len(), vibe_sorted.len());
        return false;
    }
    
    let mut all_correct = true;
    for (i, (rust_val, vibe_val)) in rust_sorted.iter().zip(vibe_sorted.iter()).enumerate() {
        if rust_val.partial_cmp(vibe_val) != Some(std::cmp::Ordering::Equal) {
            println!("❌ Index {}: Rust {:?} vs Vibesort {:?}", i, rust_val, vibe_val);
            all_correct = false;
        }
    }
    
    if all_correct {
        println!("✅ Arrays match perfectly!");
    }
    
    all_correct
}

async fn benchmark_all_types(n: usize, data_type: DataType) -> Result<(), VibesortError> {
    match data_type {
        DataType::WholeNumbers => {
            println!("=== Integer Benchmark (n={}) ===", n);
            let data = generate_random_numbers(n);
            println!("Original array: {:?}", data);
            
            let mut rust_data = data.clone();
            let start = Instant::now();
            rust_data.sort();
            let rust_time = start.elapsed();
            println!("Rust sorted array: {:?}", rust_data);
            println!("Rust sort: {:?} - Correct: {}", rust_time, is_sorted(&rust_data));
            
            let start = Instant::now();
            match vibesort(data.clone()).await {
                Ok(vibe_sorted) => {
                    let vibe_time = start.elapsed();
                    println!("Vibesort array: {:?}", vibe_sorted);
                    println!("Vibesort: {:?}", vibe_time);
                    compare_arrays(&rust_data, &vibe_sorted);
                }
                Err(e) => println!("Vibesort failed: {}", e),
            }
        }
        DataType::Floats => {
            println!("=== Float Benchmark (n={}) ===", n);
            let data = generate_random_floats(n);
            println!("Original array: {:?}", data);
            
            let mut rust_data = data.clone();
            let start = Instant::now();
            rust_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let rust_time = start.elapsed();
            println!("Rust sorted array: {:?}", rust_data);
            println!("Rust sort: {:?} - Correct: {}", rust_time, is_sorted(&rust_data));
            
            let start = Instant::now();
            match vibesort(data.clone()).await {
                Ok(vibe_sorted) => {
                    let vibe_time = start.elapsed();
                    println!("Vibesort array: {:?}", vibe_sorted);
                    println!("Vibesort: {:?}", vibe_time);
                    compare_arrays(&rust_data, &vibe_sorted);
                }
                Err(e) => println!("Vibesort failed: {}", e),
            }
        }
        DataType::Strings => {
            println!("=== String Benchmark (n={}) ===", n);
            let data = generate_random_strings(n);
            println!("Original array: {:?}", data);
            
            let mut rust_data = data.clone();
            let start = Instant::now();
            rust_data.sort();
            let rust_time = start.elapsed();
            println!("Rust sorted array: {:?}", rust_data);
            println!("Rust sort: {:?} - Correct: {}", rust_time, is_sorted(&rust_data));
            
            let start = Instant::now();
            match vibesort(data.clone()).await {
                Ok(vibe_sorted) => {
                    let vibe_time = start.elapsed();
                    println!("Vibesort array: {:?}", vibe_sorted);
                    println!("Vibesort: {:?}", vibe_time);
                    compare_arrays(&rust_data, &vibe_sorted);
                }
                Err(e) => println!("Vibesort failed: {}", e),
            }
        }
        DataType::All => {
            Box::pin(benchmark_all_types(n, DataType::WholeNumbers)).await?;
            println!();
            Box::pin(benchmark_all_types(n, DataType::Floats)).await?;
            println!();
            Box::pin(benchmark_all_types(n, DataType::Strings)).await?;
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: {} <n> <type>", args[0]);
        println!("Types: whole, floats, strings, all");
        return Ok(());
    }
    
    let n: usize = args[1].parse()?;
    let data_type: DataType = args[2].parse().map_err(|e| e)?;
    
    benchmark_all_types(n, data_type).await?;
    
    Ok(())
}