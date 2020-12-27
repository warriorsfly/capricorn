// Headers
pub const AUTHORIZATION: &str = "Authorization";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];
