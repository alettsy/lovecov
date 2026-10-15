pub fn calc_percent(val1: f32, val2: f32) -> f32 {
    if val2 == 0.0 || val1.is_nan() || val2.is_nan() {
        return 0.0;
    }

    (val1 / val2) * 100.0
}
