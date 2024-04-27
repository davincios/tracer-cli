// tests/test-events.rs
// please fix this issue:
// unresolved import `crate::events`
// could not find `events` in the crate rootrustcClick for full compiler diagnostic

use tracer_cli::send_event;

const BASE_URL: &str = "https://app.tracer.bio/api/fluent-bit-webhook";

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_event_success_production() {
        let result = send_event(
            "test_event",
            "Test message",
            Some("completed"),
            BASE_URL, // Change to BASE_URL_PRODUCTION if you want to test against production
        )
        .await;
        assert!(result.is_ok());
    }
}
