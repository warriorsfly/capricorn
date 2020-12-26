// Headers
pub const AUTHORIZATION: &str = "Authorization";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// application private streams
pub const PRIVATE_STREAM: &str = "private-stream";

/// collab stream for all applications
pub const COLLAB_STREAM: &str = "collab-stream";

pub const STREAMS: &[&str] = &[PRIVATE_STREAM, COLLAB_STREAM];

/// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];
