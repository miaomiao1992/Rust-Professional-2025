pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut a = 0; // F(0)
    let mut b = 1; // F(1)
    let mut sum = 0;

    while b < threshold {
        if b % 2 != 0 {
            sum += b; // 如果是奇数，累加到 sum
        }
        let next = a + b; // 计算下一个斐波那契数
        a = b;
        b = next;
    }

    sum
}
