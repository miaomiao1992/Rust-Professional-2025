pub fn goldbach_conjecture() -> String {
    let mut count = 0;
    let mut result = String::new();
    let mut n = 9;

    while count < 2 {
        if is_composite(n) && !can_be_expressed(n) {
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str(&n.to_string());
            count += 1;
        }
        n += 2;
    }

    result
}

fn is_composite(n: u64) -> bool {
    // 判断n是否为合数
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return true;
        }
    }
    false
}

fn can_be_expressed(n: u64) -> bool {
    // 判断n是否可以表示为一个素数和一个平方的两倍之和
    for k in 1..=(n as f64).sqrt() as u64 {
        let p = n.checked_sub(2 * k * k); // 使用checked_sub避免溢出
        if let Some(p_val) = p {
            if is_prime(p_val) {
                return true;
            }
        } else {
            // 如果p为None，说明n < 2 * k * k，直接跳出循环
            break;
        }
    }
    false
}

fn is_prime(n: u64) -> bool {
    // 判断n是否为素数
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
