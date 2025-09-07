# vibesort-rs
Blazingly fast vibesort implementation in Rust using Llama 4 Maverick on Cerebras Cloud. 10x faster than the current [vibesort](https://github.com/abyesilyurt/vibesort).
<div align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/crates/v/vibesort?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/AI%20POWERED-🤖-ff6b6b?style=for-the-badge&labelColor=000000" />
  <img src="https://img.shields.io/badge/ASYNC-POWERED-purple?style=for-the-badge" />
  <img src="https://img.shields.io/badge/10x-VIBECODER-brown?style=for-the-badge" />
</div>
<p></p>
<img width="925" height="517" alt="image" src="https://github.com/user-attachments/assets/7bf8bb5f-f04f-4753-acde-c81e05164f1d" />

[![asciicast](https://asciinema.org/a/wv1Byg5J8AcSKLR1KUyjeOfaG.svg)](https://asciinema.org/a/wv1Byg5J8AcSKLR1KUyjeOfaG) 



## Installation

```bash
cargo install vibesort
```

## Setup

Export your [Cerebras](https://cloud.cerebras.ai/) API key:

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
