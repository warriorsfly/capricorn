use std::time::Duration;

// Headers
pub const AUTHORIZATION: &str = "Authorization";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const KEY_LAB_MESSAGE: &str = "lab_message";

/// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
pub const HEARTBEAT_STREAM: Duration = Duration::from_millis(10);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
