use std::sync::OnceLock;
use reqwest::{Client, RequestBuilder, Response};
use std::time::{Duration, Instant};
use std::sync::Mutex;

/// Shared HTTP client instance with connection pooling and optimized settings
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// Gets or initializes the shared HTTP client
/// 
/// The client is configured with:
/// - Connection pooling enabled
/// - Timeout settings optimized for API calls
/// - Keep-alive connections
pub fn get_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(6)
            .pool_idle_timeout(Duration::from_secs(90))
            // Removed http2_prior_knowledge() as it may fail if server doesn't support HTTP/2
            .build()
            .expect("Failed to create HTTP client")
    })
}

/// Retry configuration for HTTP requests
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(8),
        }
    }
}

/// Executes an HTTP request with exponential backoff retry logic
/// 
/// Retries on network errors and 5xx server errors (but not 4xx client errors)
/// Uses exponential backoff: 1s, 2s, 4s delays (capped at max_delay)
pub async fn execute_with_retry(
    request_builder: RequestBuilder,
    config: RetryConfig,
) -> Result<Response, reqwest::Error> {
    let mut last_error = None;
    let mut delay = config.initial_delay;

    for attempt in 0..=config.max_retries {
        match request_builder.try_clone() {
            Some(builder) => {
                match builder.send().await {
                    Ok(response) => {
                        let status = response.status();
                        // Retry on 5xx server errors, but not on 4xx client errors
                        if status.is_server_error() && attempt < config.max_retries {
                            // Clone response to read status, but we'll retry
                            drop(response);
                            tokio::time::sleep(delay).await;
                            delay = std::cmp::min(delay * 2, config.max_delay);
                            continue;
                        }
                        return Ok(response);
                    }
                    Err(e) => {
                        last_error = Some(e);
                        // Retry on network errors
                        if attempt < config.max_retries && e.is_timeout() || e.is_connect() || e.is_request() {
                            tokio::time::sleep(delay).await;
                            delay = std::cmp::min(delay * 2, config.max_delay);
                            continue;
                        }
                        // Don't retry on other errors (like decode errors)
                        break;
                    }
                }
            }
            None => {
                // RequestBuilder doesn't support cloning, execute without retry
                return request_builder.send().await;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        reqwest::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed after retries",
        ))
    }))
}

/// Convenience function to execute a request with default retry configuration and circuit breaker
pub async fn execute_with_default_retry(request_builder: RequestBuilder) -> Result<Response, reqwest::Error> {
    execute_with_circuit_breaker(request_builder, RetryConfig::default()).await
}

/// Simple circuit breaker implementation
/// Tracks consecutive failures and opens circuit after threshold
struct CircuitBreaker {
    failure_count: u32,
    last_failure_time: Option<Instant>,
    is_open: bool,
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            failure_count: 0,
            last_failure_time: None,
            is_open: false,
        }
    }

    fn record_success(&mut self) {
        self.failure_count = 0;
        self.is_open = false;
    }

    fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        // Open circuit after 5 consecutive failures
        if self.failure_count >= 5 {
            self.is_open = true;
        }
    }

    fn should_allow_request(&mut self) -> bool {
        if !self.is_open {
            return true;
        }
        // Try to recover after 60 seconds
        if let Some(last_failure) = self.last_failure_time {
            if last_failure.elapsed() >= Duration::from_secs(60) {
                self.is_open = false;
                self.failure_count = 0;
                return true;
            }
        }
        false
    }
}

/// Global circuit breaker (simplified - single breaker for all requests)
/// In production, use per-domain or per-endpoint breakers
static CIRCUIT_BREAKER: OnceLock<Mutex<CircuitBreaker>> = OnceLock::new();

fn get_circuit_breaker() -> &'static Mutex<CircuitBreaker> {
    CIRCUIT_BREAKER.get_or_init(|| Mutex::new(CircuitBreaker::new()))
}

/// Executes an HTTP request with circuit breaker and retry logic
/// Circuit breaker prevents requests when service is down
pub async fn execute_with_circuit_breaker(
    request_builder: RequestBuilder,
    config: RetryConfig,
) -> Result<Response, reqwest::Error> {
    let breaker = get_circuit_breaker();
    let mut cb = breaker.lock().unwrap();

    // Check if circuit allows request
    if !cb.should_allow_request() {
        return Err(reqwest::Error::from(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "Circuit breaker is open - service unavailable",
        )));
    }

    drop(cb); // Release lock before async operation

    // Execute request with retry
    let result = execute_with_retry(request_builder, config).await;

    // Update circuit breaker based on result
    let mut cb = breaker.lock().unwrap();
    match &result {
        Ok(_) => cb.record_success(),
        Err(_) => cb.record_failure(),
    }

    result
}

/// Convenience function to execute with circuit breaker and default retry config
pub async fn execute_with_circuit_breaker_default(request_builder: RequestBuilder) -> Result<Response, reqwest::Error> {
    execute_with_circuit_breaker(request_builder, RetryConfig::default()).await
}
