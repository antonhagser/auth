use crate::state::AppState;

/// MetricsError enumerates the possible errors that can occur
/// when running the metrics thread.
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {}

pub async fn run(_state: AppState) -> Result<(), MetricsError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 24 * 7)).await;

    Ok(())
}
