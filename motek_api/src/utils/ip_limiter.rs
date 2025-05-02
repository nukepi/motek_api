use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Limits the number of registration attempts per IP address within a given time frame.
#[derive(Clone)]
pub struct IpLimiter {
    // IpAddr -> list of registration times within the last hour
    pub map: Arc<Mutex<HashMap<IpAddr, Vec<SystemTime>>>>,
    pub per_hour: u32,
}

impl IpLimiter {
    /// Creates a new limiter with a given per-hour limit.
    pub fn new(per_hour: u32) -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
            per_hour,
        }
    }

    /// Checks if registration is allowed for an IP, updates the record, and logs the decision.
    pub async fn check_and_update(&self, ip: IpAddr) -> bool {
        let mut map = self.map.lock().await;
        let now = SystemTime::now();
        let hour = Duration::from_secs(60 * 60);

        let entry = map.entry(ip).or_insert_with(Vec::new);

        // Remove old registrations older than one hour
        entry.retain(|&t| now.duration_since(t).unwrap_or(hour) < hour);

        if entry.len() as u32 >= self.per_hour {
            warn!(
                "Registration denied for IP {}: limit {}/hour exceeded",
                ip, self.per_hour
            );
            return false; // limit exceeded
        }

        entry.push(now);
        info!(
            "Registration allowed for IP {} (total in last hour: {})",
            ip,
            entry.len()
        );
        true
    }
}
