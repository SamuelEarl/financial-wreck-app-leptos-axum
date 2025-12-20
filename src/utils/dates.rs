use chrono::Utc;

/// Rust includes nanoseconds (up to 9 decimal places: .123456789) in the timestamp. (Node.js stops at milliseconds.)
/// This Rust timestamp uses +00:00 to indicate UTC. (This is more explicit than Z, which is what Node.js uses).
pub fn get_utc_iso_date() -> String {
    // Utc::now() gets the current time.
    // to_rfc3339() formats it as YYYY-MM-DDTHH:MM:SS+00:00
    Utc::now().to_rfc3339()
}
