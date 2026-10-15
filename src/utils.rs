pub fn calc_percent(val1: f32, val2: f32) -> f32 {
    if val2 == 0.0 || val1.is_nan() || val2.is_nan() {
        return 0.0;
    }

    (val1 / val2) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_percent_input() {
        assert!(calc_percent(100.0, 400.0) == 25.0);
    }

    #[test]
    fn zero_denominator_is_0() {
        assert!(calc_percent(100.0, 0.0) == 0.0);
    }

    #[test]
    fn nan_denominator() {
        assert!(calc_percent(100.0, f32::NAN) == 0.0);
    }

    #[test]
    fn nan_numerator() {
        assert!(calc_percent(f32::NAN, 400.0) == 0.0);
    }
}
