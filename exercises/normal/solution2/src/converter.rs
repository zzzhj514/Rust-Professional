
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let ori_str = num_str.clone();
    let parts: Vec<&str> = num_str.split(|c| c == '(' || c == ')').collect();

    let mut first_num: i32 = parts[0].trim().parse().unwrap();
    let second_num: i32 = parts[1].trim().parse().unwrap();
    if second_num != 10{
        first_num = n_to_decimal(second_num as u32, first_num as u32)
    }
    dec_to_n(to_base, first_num as u32)
}

fn n_to_decimal(n: u32, num: u32) -> i32 {
    // 将 i32 转换为字符串
    let num_str = num.to_string();

    // 从字符串中提取每一位数字，并计算十进制值
    let mut decimal = 0;
    let len = num_str.len();
    for (i, c) in num_str.chars().enumerate() {
        // 将字符转换为数字
        let digit = c.to_digit(10).unwrap() as i32;
        // 计算当前位的权重
        let power = len - i - 1;
        // 累加到十进制结果中
        decimal += digit * n.pow(power as u32) as i32;
    }
    decimal
}
fn dec_to_n(n: u32, num: u32) -> String {
    if n < 2 || n > 36 {
        panic!("Base must be between 2 and 36");
    }

    // 定义字符集，用于表示 0-9 和 A-Z
    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

    // 特殊情况：0 的任何进制表示都是 "0"
    if num == 0 {
        return "0".to_string();
    }

    let mut num = num;
    let mut result = String::new();

    // 循环除法，直到 num 为 0
    while num > 0 {
        let remainder = num % n;
        num /= n;
        // 将余数对应的字符添加到结果字符串的前面
        result.insert_str(0, &digits[remainder as usize..remainder as usize + 1]);
    }

    result
}
