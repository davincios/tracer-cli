// tests/test-events.rs
// please fix this issue:
// unresolved import `crate::events`
// could not find `events` in the crate rootrustcClick for full compiler diagnostic

use tracer_cli::send_event;

const BASE_URL: &str = "https://app.tracer.bio/api/fluent-bit-webhook";
const API_KEY: &str = "dDRE5rxJEjktQxCtzsYyz";

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_event_success_production() {
        let result = send_event(
            "test_event",
            "test-events.rs/test_send_event_success_production",
            Some("completed"),
            BASE_URL,
            API_KEY,
        )
        .await;
        assert!(result.is_ok());
    }
}
