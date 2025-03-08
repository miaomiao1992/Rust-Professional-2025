pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0; // 如果人数少于2，概率为0
    }

    let mut probability_all_different = 1.0;
    for i in 0..n {
        probability_all_different *= (365.0 - i as f64) / 365.0;
    }

    let probability_at_least_two_same = 1.0 - probability_all_different;
    (probability_at_least_two_same * 10000.0).round() / 10000.0 // 保留四位小数
}
