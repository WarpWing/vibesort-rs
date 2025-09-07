use serde_json::{json, Value};
use std::env;
use std::fmt::Debug;
use std::future::Future;
use std::sync::OnceLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VibesortError {
    #[error("Environment variable CEREBRAS_API_KEY not found")]
    MissingApiKey,
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Invalid response format")]
    InvalidResponse,
}

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
static API_KEY: OnceLock<String> = OnceLock::new();

fn get_http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| reqwest::Client::new())
}

fn get_api_key() -> Result<&'static str, VibesortError> {
    if let Some(key) = API_KEY.get() {
        return Ok(key.as_str());
    }

    let key = env::var("CEREBRAS_API_KEY").map_err(|_| VibesortError::MissingApiKey)?;
    let _ = API_KEY.set(key);

    Ok(API_KEY.get().unwrap().as_str())
}

/// Sorts Vec<T> blazingly fast with enterprise AI powered precision
///
/// ```rust
/// let my_vec = vec![69, 41, 67, 9 + 10, 21, 420, 4];
///
/// // [4, 19, 21, 41, 67, 69, 420] (hopefully)
/// let sorted_vec = vibesort(my_vec).await;
/// ```
pub async fn vibesort<T: std::fmt::Display + std::str::FromStr + Clone>(
    vec: Vec<T>,
) -> Result<Vec<T>, VibesortError>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let api_key = get_api_key()?;
    let client = get_http_client();

    let mut input_str = String::new();
    for (i, item) in vec.iter().enumerate() {
        if i > 0 {
            input_str.push_str(", ");
        }
        input_str.push_str(&item.to_string());
    }

    let payload = json!({
        "model": "llama-4-maverick-17b-128e-instruct",
        "stream": false,
        "max_tokens": 128,
        "temperature": 0.1,
        "messages": [
            {
                "role": "system",
                "content": "Return only sorted array in format [1, 2, 3]. No text."
            },
            {
                "role": "user",
                "content": format!("[{}]", input_str)
            }
        ]
    });

    let response = client
        .post("https://api.cerebras.ai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or(VibesortError::InvalidResponse)?
        .trim();

    let clean_content = content.trim_start_matches('[').trim_end_matches(']');

    let result: Result<Vec<T>, _> = clean_content
        .split(", ")
        .map(|s| {
            s.trim()
                .parse()
                .map_err(|_| VibesortError::ParseError(s.to_string()))
        })
        .collect();

    result
}

pub trait VibeSort<T: std::fmt::Display + std::str::FromStr + Clone>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    /// Sorts Vec<T> blazingly fast with enterprise AI powered precision
    ///
    /// ```rust
    /// let my_vec = vec![69, 41, 67, 9 + 10, 21, 420, 4];
    ///
    /// // [4, 19, 21, 41, 67, 69, 420] (hopefully)
    /// let sorted_vec = my_vec.vibe_sort().await;
    /// ```
    fn vibesort(self) -> impl Future<Output = Result<Vec<T>, VibesortError>>;
}

impl<T: std::fmt::Display + std::str::FromStr + Clone> VibeSort<T> for Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    async fn vibesort(self) -> Result<Vec<T>, VibesortError> {
        vibesort(self).await
    }
}
