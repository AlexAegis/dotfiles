/// Logs to stderr without timestamps or colors, for the journal.
pub fn init() {
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .init();
}
