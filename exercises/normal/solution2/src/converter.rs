pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入字符串，提取数字和原始基数
    let (num, from_base) = parse_num_str(num_str);

    // 将数字从原始基数转换为十进制
    let decimal = to_decimal(&num, from_base);

    // 将十进制数转换为目标基数
    from_decimal(decimal, to_base)
}

fn parse_num_str(num_str: &str) -> (String, u32) {
    let parts: Vec<&str> = num_str.split('(').collect();
    let num = parts[0].to_string();
    let base = parts[1].trim_end_matches(')').parse::<u32>().unwrap();
    (num, base)
}

fn to_decimal(num: &str, from_base: u32) -> u32 {
    u32::from_str_radix(num, from_base).unwrap()
}

fn from_decimal(mut decimal: u32, to_base: u32) -> String {
    if decimal == 0 {
        return "0".to_string();
    }
    let mut result = String::new();
    while decimal > 0 {
        let remainder = decimal % to_base;
        // 处理大于 9 的余数，转换为对应的字符
        let char = match remainder {
            0..=9 => (remainder as u8 + b'0') as char,
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => panic!("Unsupported base"),
        };
        result.push(char);
        decimal /= to_base;
    }
    result.chars().rev().collect()
}
