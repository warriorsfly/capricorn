use std::time::Duration;

// Headers
pub const AUTHORIZATION: &str = "Authorization";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const STREAM_LEN: usize = 1000;

/// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
