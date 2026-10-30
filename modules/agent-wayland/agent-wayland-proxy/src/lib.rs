pub mod allowed_globals;
pub mod client_connection;
pub mod logging;
pub mod registry_filter;
pub mod security_context;
pub mod socket_activation;
pub mod state_error;
pub mod systemd_notify;
pub mod window_tagging;

use wl_proxy::baseline::Baseline;

/// Upper bound of the protocol versions that the proxy parses and forwards.
pub const BASELINE: Baseline = Baseline::V6;
