pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut num:f64 = 1.0;
    for i in 1..=n{
        num *= (365.0 + i as f64 - n as f64) / (365.0);
    }
    1.0-num
}