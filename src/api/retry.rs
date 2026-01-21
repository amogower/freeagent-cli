//! Retry logic and rate limit handling for API requests.

use anyhow::{Context, Result};
use reqwest::Response;
use std::time::Duration;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial backoff duration in seconds
    pub initial_backoff_secs: u64,
    /// Maximum backoff duration in seconds
    pub max_backoff_secs: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_secs: 1,
            max_backoff_secs: 60,
            exponential_backoff: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration from environment variables
    pub fn from_env() -> Self {
        let max_retries = std::env::var("FREEAGENT_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        let initial_backoff_secs = std::env::var("FREEAGENT_INITIAL_BACKOFF_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let max_backoff_secs = std::env::var("FREEAGENT_MAX_BACKOFF_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        let exponential_backoff = std::env::var("FREEAGENT_EXPONENTIAL_BACKOFF")
            .ok()
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(true);

        Self {
            max_retries,
            initial_backoff_secs,
            max_backoff_secs,
            exponential_backoff,
        }
    }

    /// Calculate the backoff duration for a given attempt
    pub fn backoff_duration(&self, attempt: u32) -> Duration {
        if !self.exponential_backoff {
            return Duration::from_secs(self.initial_backoff_secs);
        }

        let backoff_secs = self.initial_backoff_secs * 2u64.pow(attempt);
        let backoff_secs = backoff_secs.min(self.max_backoff_secs);
        Duration::from_secs(backoff_secs)
    }
}

/// Information about a rate limit response
#[derive(Debug)]
pub struct RateLimitInfo {
    /// How long to wait before retrying (in seconds)
    pub retry_after_secs: u64,
    /// The error message from the API
    pub message: String,
}

impl RateLimitInfo {
    /// Extract rate limit information from a 429 response
    pub async fn from_response(response: Response) -> Result<Self> {
        // Try to extract Retry-After header
        let retry_after_secs = response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60); // Default to 60 seconds if not specified

        // Extract error message from body
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Rate limit exceeded".to_string());

        Ok(Self {
            retry_after_secs,
            message,
        })
    }

    /// Get the duration to wait before retrying
    pub fn retry_duration(&self) -> Duration {
        Duration::from_secs(self.retry_after_secs)
    }
}

/// Result of checking if a response should be retried
#[derive(Debug)]
pub enum RetryDecision {
    /// Retry after the specified duration
    Retry(Duration),
    /// Do not retry, return the error
    Fail(String),
    /// Success, no retry needed
    Success,
}

/// Check if a response indicates a rate limit and should be retried
pub async fn check_rate_limit(response: &Response, attempt: u32, config: &RetryConfig) -> RetryDecision {
    let status = response.status();

    // Check if this is a rate limit response
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        if attempt >= config.max_retries {
            return RetryDecision::Fail(format!(
                "Rate limit exceeded. Maximum retry attempts ({}) reached.",
                config.max_retries
            ));
        }

        // Try to extract Retry-After header
        let retry_after_secs = response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or_else(|| {
                // Use exponential backoff if no Retry-After header
                config.backoff_duration(attempt).as_secs()
            });

        // Cap at max backoff
        let retry_after_secs = retry_after_secs.min(config.max_backoff_secs);

        return RetryDecision::Retry(Duration::from_secs(retry_after_secs));
    }

    RetryDecision::Success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_config_default_values() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff_secs, 1);
        assert_eq!(config.max_backoff_secs, 60);
        assert!(config.exponential_backoff);
    }

    #[test]
    fn retry_config_exponential_backoff() {
        let config = RetryConfig::default();
        
        // First attempt: 1 * 2^0 = 1 second
        assert_eq!(config.backoff_duration(0).as_secs(), 1);
        
        // Second attempt: 1 * 2^1 = 2 seconds
        assert_eq!(config.backoff_duration(1).as_secs(), 2);
        
        // Third attempt: 1 * 2^2 = 4 seconds
        assert_eq!(config.backoff_duration(2).as_secs(), 4);
        
        // Fourth attempt: 1 * 2^3 = 8 seconds
        assert_eq!(config.backoff_duration(3).as_secs(), 8);
    }

    #[test]
    fn retry_config_respects_max_backoff() {
        let config = RetryConfig {
            max_retries: 10,
            initial_backoff_secs: 1,
            max_backoff_secs: 10,
            exponential_backoff: true,
        };
        
        // Should cap at max_backoff_secs
        assert_eq!(config.backoff_duration(10).as_secs(), 10);
    }

    #[test]
    fn retry_config_linear_backoff() {
        let config = RetryConfig {
            max_retries: 3,
            initial_backoff_secs: 5,
            max_backoff_secs: 60,
            exponential_backoff: false,
        };
        
        // All attempts should use initial_backoff_secs
        assert_eq!(config.backoff_duration(0).as_secs(), 5);
        assert_eq!(config.backoff_duration(1).as_secs(), 5);
        assert_eq!(config.backoff_duration(2).as_secs(), 5);
    }

    #[test]
    fn retry_config_from_env() {
        std::env::set_var("FREEAGENT_MAX_RETRIES", "5");
        std::env::set_var("FREEAGENT_INITIAL_BACKOFF_SECS", "2");
        std::env::set_var("FREEAGENT_MAX_BACKOFF_SECS", "120");
        std::env::set_var("FREEAGENT_EXPONENTIAL_BACKOFF", "false");

        let config = RetryConfig::from_env();
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_backoff_secs, 2);
        assert_eq!(config.max_backoff_secs, 120);
        assert!(!config.exponential_backoff);

        // Clean up
        std::env::remove_var("FREEAGENT_MAX_RETRIES");
        std::env::remove_var("FREEAGENT_INITIAL_BACKOFF_SECS");
        std::env::remove_var("FREEAGENT_MAX_BACKOFF_SECS");
        std::env::remove_var("FREEAGENT_EXPONENTIAL_BACKOFF");
    }
}
