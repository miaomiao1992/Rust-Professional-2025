const R50_MAX: i32 = 60;
const R55_MAX: i32 = 36;
const R60_MAX: i32 = 36;

#[derive(Debug, Clone)]
struct ConvertedTime {
    year: i32,
    month: i32,
}

impl ConvertedTime {
    fn new(year: i32, month: i32) -> Self {
        ConvertedTime { year, month }
    }

    fn next_month(&mut self) {
        if self.month == 12 {
            self.year += 1;
            self.month = 1;
        } else {
            self.month += 1;
        };
    }

    fn add_months(&mut self, n: i32) {
        let total_months = self.year * 12 + self.month - 1 + n;
        self.year = (total_months / 12) as i32;
        self.month = (total_months % 12) + 1;
    }

    fn to_string(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }
}

enum RetirementType {
    R50,
    R55,
    R60,
}

fn post_poned_retirement(
    retirement_type: &RetirementType,
    converted_time: &mut ConvertedTime,
    previous_converted_time: &mut ConvertedTime,
) {
    if previous_converted_time.year == 2025 && previous_converted_time.month == 1 {
        converted_time.next_month();
        previous_converted_time.next_month();
    }

    let time_diff = time_diff(converted_time, previous_converted_time);
    //println!("time_diff: {time_diff}");
    if time_diff > 0 {
        match retirement_type {
            RetirementType::R50 => {
                let temp = time_diff / 2;
                if temp < R50_MAX {
                    new_converted_time(converted_time, temp, previous_converted_time);
                    post_poned_retirement(retirement_type, converted_time, previous_converted_time)
                } else {
                    converted_time.add_months(R50_MAX - 1);
                }
            }
            RetirementType::R55 => {
                let temp = time_diff / 4;
                if temp < R55_MAX {
                    new_converted_time(converted_time, temp, previous_converted_time);
                    post_poned_retirement(retirement_type, converted_time, previous_converted_time)
                } else {
                    converted_time.add_months(R55_MAX - 1);
                }
            }
            RetirementType::R60 => {
                let temp = time_diff / 4;
                if temp < R60_MAX {
                    new_converted_time(converted_time, temp, previous_converted_time);
                    post_poned_retirement(retirement_type, converted_time, previous_converted_time)
                } else {
                    converted_time.add_months(R60_MAX - 1);
                }
            }
        }
    }
}

fn new_converted_time(
    converted_time: &mut ConvertedTime,
    post_poned_month: i32,
    previous_converted_time: &mut ConvertedTime,
) {
    previous_converted_time.year = converted_time.year;
    previous_converted_time.month = converted_time.month;
    converted_time.year += (post_poned_month + converted_time.month - 1) / 12;
    converted_time.month = (post_poned_month - 1 + converted_time.month) % 12 + 1;
    //println!("{:?}, {:?}", converted_time, previous_converted_time);
}

fn time_diff(converted_time_1: &ConvertedTime, converted_time_2: &ConvertedTime) -> i32 {
    let mut result: i32 = 0;
    let year_diff: i32 = converted_time_1.year - converted_time_2.year;
    let month_diff: i32 = converted_time_1.month - converted_time_2.month;
    result += year_diff * 12;
    result += month_diff;
    result
}

fn check_post_pone(original_retire_time: &ConvertedTime) -> bool {
    let start = ConvertedTime::new(2025, 1);
    let diff = time_diff(original_retire_time, &start);
    diff >= 0
}

fn convert_time(time: &str) -> Result<ConvertedTime, &'static str> {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse::<i32>().map_err(|_| "无法解析年份")?;
    let month = parts[1].parse::<i32>().map_err(|_| "无法解析月份")?;
    let converted_time = ConvertedTime::new(year, month);
    Ok(converted_time)
}

fn convert_tp(str: &str) -> RetirementType {
    if str.contains("男") {
        RetirementType::R60
    } else if str.contains("55") {
        RetirementType::R55
    } else {
        RetirementType::R50
    }
}

fn original_retire_time(
    birth_time: &ConvertedTime,
    retirement_type: &RetirementType,
) -> ConvertedTime {
    let mut converted_time = birth_time.clone();
    match retirement_type {
        RetirementType::R50 => {
            converted_time.add_months(50 * 12);
            converted_time
        }
        RetirementType::R55 => {
            converted_time.add_months(55 * 12);
            converted_time
        }
        RetirementType::R60 => {
            converted_time.add_months(60 * 12);
            converted_time
        }
    }
}

fn retire_age(converted_time: &ConvertedTime, birth_time: &ConvertedTime) -> String {
    let result = time_diff(&converted_time, &birth_time) as f32 / 12.0;
    if result == result.floor() {
        format!("{}", result.floor() as i32)
    } else {
        format!("{:.2}", result)
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_time = convert_time(time).unwrap();
    let retirement_type = convert_tp(tp);
    let original_retire_time = original_retire_time(&birth_time, &retirement_type);
    if check_post_pone(&original_retire_time) {
        let start = ConvertedTime::new(2025, 1);
        let mut previous_converted_time = start.clone();
        let mut converted_time = original_retire_time.clone();
        post_poned_retirement(
            &retirement_type,
            &mut converted_time,
            &mut previous_converted_time,
        );
        let post_poned_months = time_diff(&converted_time, &original_retire_time);
        let new_retire_date = converted_time.to_string();
        let new_age = retire_age(&converted_time, &birth_time);
        format!("{},{},{}", new_retire_date, new_age, post_poned_months)
    } else {
        let new_retire_date = original_retire_time.to_string();
        let post_poned_months: i32 = 0;
        let new_age = retire_age(&original_retire_time, &birth_time);
        format!("{},{},{}", new_retire_date, new_age, post_poned_months)
    }
}
