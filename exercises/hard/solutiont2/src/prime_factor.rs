use std::time::{SystemTime, UNIX_EPOCH};

pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_prime = 2;

    // 处理2的情况
    while number % 2 == 0 {
        max_prime = 2;
        number /= 2;
    }

    // 如果剩下的数是1，直接返回
    if number == 1 {
        return max_prime;
    }

    // 使用 Pollard's Rho 算法分解质因数
    let factors = pollards_rho_factorization(number);
    for factor in factors {
        if factor > max_prime {
            max_prime = factor;
        }
    }

    max_prime
}

fn pseudo_random() -> u128 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    time
}

fn pollards_rho_factorization(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    if n == 1 {
        return factors;
    }
    if is_prime(n) {
        factors.push(n);
        return factors;
    }
    let mut d = n;
    while d == n {
        let x = pseudo_random() % n;
        let c = pseudo_random() % n;
        d = pollards_rho_helper(n, x, c);
    }
    factors.extend(pollards_rho_factorization(d));
    factors.extend(pollards_rho_factorization(n / d));
    factors
}

// Pollard's Rho 辅助函数
fn pollards_rho_helper(n: u128, x: u128, c: u128) -> u128 {
    let mut x = x;
    let mut y = x;
    let mut d = 1;
    while d == 1 {
        x = ((x as u128).wrapping_mul(x).wrapping_add(c)) % n;
        y = ((y as u128).wrapping_mul(y).wrapping_add(c)) % n;
        y = ((y as u128).wrapping_mul(y).wrapping_add(c)) % n;
        d = gcd((x as i128 - y as i128).abs() as u128, n);
    }
    d
}

// Miller-Rabin 素数测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for _ in 0..10 {
        let a = pseudo_random() % (n - 2) + 2;
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                break;
            }
        }
        if x != n - 1 {
            return false;
        }
    }
    true
}

// 快速幂取模
fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

// 最大公约数
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
