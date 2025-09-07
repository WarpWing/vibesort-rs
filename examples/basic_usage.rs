use std::time::Instant;
use vibesort::vibesort;

#[tokio::main]
async fn main() {
    let numbers = vec![42, 7, 13, 99, 1, 56];
    println!("Original numbers: {:?}", numbers);
    let start = Instant::now();
    match vibesort(numbers.clone()).await {
        Ok(sorted_array) => {
            let duration = start.elapsed();
            println!("Sorted numbers: {:?}", sorted_array);
            println!("Time taken: {:?}", duration);
        },
        Err(e) => println!("Error: {}", e),
    }

    let numbers2 = vec![87, 23, 45, 12, 67, 34, 89, 5];
    println!("\nOriginal numbers: {:?}", numbers2);
    let start = Instant::now();
    match vibesort(numbers2.clone()).await {
        Ok(sorted_array) => {
            let duration = start.elapsed();
            println!("Sorted numbers: {:?}", sorted_array);
            println!("Time taken: {:?}", duration);
        },
        Err(e) => println!("Error: {}", e),
    }

    let numbers3 = vec![9, 2, 7, 1, 4, 8, 3, 6, 5];
    println!("\nOriginal numbers: {:?}", numbers3);
    let start = Instant::now();
    match vibesort(numbers3.clone()).await {
        Ok(sorted_array) => {
            let duration = start.elapsed();
            println!("Sorted numbers: {:?}", sorted_array);
            println!("Time taken: {:?}", duration);
        },
        Err(e) => println!("Error: {}", e),
    }
}