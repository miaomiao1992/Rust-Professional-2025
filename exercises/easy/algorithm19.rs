/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

// 矩阵乘法函数
fn matrix_mult(a: [[i64; 2]; 2], b: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut result = [[0, 0], [0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

// 矩阵快速幂运算函数
fn matrix_pow(m: [[i64; 2]; 2], n: i32) -> [[i64; 2]; 2] {
    let mut result = [[1, 0], [0, 1]]; // 单位矩阵
    let mut base = m;
    let mut exp = n;
    while exp > 0 {
        if exp % 2 == 1 {
            result = matrix_mult(result, base);
        }
        base = matrix_mult(base, base);
        exp /= 2;
    }
    result
}

// 计算第 n 个斐波那契数
pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let m = [[1, 1], [1, 0]]; // 斐波那契矩阵
    let result = matrix_pow(m, n - 1);
    result[0][0] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
