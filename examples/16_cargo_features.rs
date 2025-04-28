// Cargo Features in Rust
// This example demonstrates how to use Cargo features for conditional compilation

// First, in your Cargo.toml:
/*
[package]
name = "feature_example"
version = "0.1.0"
edition = "2021"

[features]
# Default features that are always enabled
default = ["logging", "json"]

# Optional features
logging = []
json = []
metrics = []
database = ["sqlx"]
cli = []

# Feature groups
full = ["logging", "json", "metrics", "database", "cli"]

[dependencies]
# Dependencies that are always included
serde = { version = "1.0", features = ["derive"] }

# Optional dependencies
sqlx = { version = "0.6", optional = true }
tokio = { version = "1.0", optional = true }
*/

// Now, let's create the Rust code that uses these features:

// Module that's only included when the "logging" feature is enabled
#[cfg(feature = "logging")]
pub mod logging {
    pub fn log_message(level: &str, message: &str) {
        println!("[{}] {}", level, message);
    }
}

// Module that's only included when the "json" feature is enabled
#[cfg(feature = "json")]
pub mod json {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub name: String,
        pub value: i32,
    }

    pub fn parse_config(json: &str) -> Result<Config, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// Module that's only included when the "metrics" feature is enabled
#[cfg(feature = "metrics")]
pub mod metrics {
    pub struct Metrics {
        count: i32,
    }

    impl Metrics {
        pub fn new() -> Self {
            Metrics { count: 0 }
        }

        pub fn increment(&mut self) {
            self.count += 1;
        }

        pub fn get_count(&self) -> i32 {
            self.count
        }
    }
}

// Module that's only included when the "database" feature is enabled
#[cfg(feature = "database")]
pub mod database {
    use sqlx::PgPool;

    pub async fn connect() -> Result<PgPool, sqlx::Error> {
        PgPool::connect("postgres://user:pass@localhost/db").await
    }
}

// Module that's only included when the "cli" feature is enabled
#[cfg(feature = "cli")]
pub mod cli {
    pub fn parse_args() -> Vec<String> {
        std::env::args().collect()
    }
}

// Main application struct that uses conditional features
pub struct App {
    #[cfg(feature = "metrics")]
    metrics: metrics::Metrics,
}

impl App {
    pub fn new() -> Self {
        App {
            #[cfg(feature = "metrics")]
            metrics: metrics::Metrics::new(),
        }
    }

    pub fn run(&mut self) {
        #[cfg(feature = "logging")]
        logging::log_message("INFO", "Application started");

        #[cfg(feature = "metrics")]
        self.metrics.increment();

        #[cfg(feature = "cli")]
        let args = cli::parse_args();

        #[cfg(feature = "json")]
        if let Ok(config) = json::parse_config(r#"{"name": "test", "value": 42}"#) {
            println!("Parsed config: {} = {}", config.name, config.value);
        }
    }
}

// Example of using feature flags in tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "logging")]
    fn test_logging() {
        logging::log_message("TEST", "This is a test message");
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json() {
        let config = json::parse_config(r#"{"name": "test", "value": 42}"#).unwrap();
        assert_eq!(config.name, "test");
        assert_eq!(config.value, 42);
    }
}

// Example of conditional compilation based on multiple features
#[cfg(all(feature = "logging", feature = "metrics"))]
pub fn log_with_metrics(message: &str) {
    logging::log_message("INFO", message);
    let mut metrics = metrics::Metrics::new();
    metrics.increment();
}

// Example of using feature flags in documentation
/// A function that demonstrates feature flags.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// app.run();
/// ```
///
/// # Features
///
/// This function's behavior depends on the following features:
///
/// - `logging`: Enables logging of operations
/// - `metrics`: Tracks operation counts
/// - `json`: Enables JSON configuration
pub fn demonstrate_features() {
    let mut app = App::new();
    app.run();
} 