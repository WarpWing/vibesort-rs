# vibesort
An improved vibesort implemented in Rust using Llama 4 Maverick on Cerebras Cloud.

<img width="925" height="517" alt="image" src="https://github.com/user-attachments/assets/40fec91e-868e-469a-b271-d9764dda750e" />

## Installation

```bash
cargo install vibesort
```

## Setup

Export your Cerebras API key:

```bash
export CEREBRAS_API_KEY=your_api_key_here
```

## Usage

```rust
use vibesort::vibesort;

#[tokio::main]
async fn main() {
    let numbers = vec![42, 7, 13, 99, 1, 56];
    println!("Original numbers: {:?}", numbers);
    
    match vibesort(numbers.clone()).await {
        Ok(sorted_array) => {
            println!("Sorted numbers: {:?}", sorted_array);
        },
        Err(e) => println!("Error: {}", e),
    }
}
```
