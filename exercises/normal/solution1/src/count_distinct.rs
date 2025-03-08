pub fn new_count_distinct(input_str: &str) -> usize {
    let mut ans = 0;
    let mut str_vec: Vec<&str> = input_str.split(",").collect();
    for _ in 0..str_vec.len() {
        let temp_option = str_vec.pop();
        match temp_option {
            Some(temp) => {
                if !str_vec.contains(&temp) {
                    ans += 1;
                }
            }
            None => (),
        }
    }
    ans
}
