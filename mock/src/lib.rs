//! Mock implementation of the SDK.

/// Compares two strings and returns a similarity score.
/// For now, this is a stub that returns a fixed value distinct from the core crate.
pub fn compare(baseline: &str, input: &str, min_similarity: i32) -> f32 {
    let _ = (baseline, input, min_similarity);
    42.0
}

