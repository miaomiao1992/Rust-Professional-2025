pub fn dp_rec_mc(amount: u32) -> u32 {
    let cashes = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &cash in &cashes {
            if i >= cash {
                dp[i as usize] = dp[i as usize].min(dp[(i - cash) as usize] + 1);
            }
        }
    }

    dp[amount as usize]
}
