/// General-purpose utilities that are not Bevy-specific.
///
/// This crate provides helper functions, data structures, and abstractions
/// that any game or library in the workspace may use. It intentionally has
/// no dependency on Bevy so it can be tested and used independently.

/// Linearly interpolate between two values.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Remap a value from one range to another.
pub fn remap(value: f32, from_low: f32, from_high: f32, to_low: f32, to_high: f32) -> f32 {
    let normalized = (value - from_low) / (from_high - from_low);
    lerp(to_low, to_high, normalized)
}

/// Clamp a value and return how far it is through the range as a 0..1 fraction.
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        return 0.0;
    }
    ((value - a) / (b - a)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_remap() {
        assert!((remap(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_inverse_lerp() {
        assert!((inverse_lerp(0.0, 10.0, 5.0) - 0.5).abs() < f32::EPSILON);
        assert!((inverse_lerp(0.0, 10.0, -5.0) - 0.0).abs() < f32::EPSILON);
        assert!((inverse_lerp(0.0, 10.0, 15.0) - 1.0).abs() < f32::EPSILON);
    }
}
