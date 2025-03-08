const MONTH_DAYS: [u32; 12] = [
    31, // 一月
    28, // 二月（平年）
    31, // 三月
    30, // 四月
    31, // 五月
    30, // 六月
    31, // 七月
    31, // 八月
    30, // 九月
    31, // 十月
    30, // 十一月
    31, // 十二月
];

const FIRSTMONDAY: u32 = 6;
const FIRSTSUNDAY: u32 = 5;
const DAYSAWEEK: u32 = 7;
const MAXWEEK: u32 = 52;
const SPRING2025: u32 = 29;
const SPRING2026: u32 = 48;
const DAYS2025: u32 = 365;

// 2025年法定节假日日期列表（按顺序排列）
const HOLIDAY_DATES: &'static [&'static str; 28] = &[
    "2025-01-01", // 元旦
    "2025-01-28", // 春节
    "2025-01-29",
    "2025-01-30",
    "2025-01-31",
    "2025-02-01",
    "2025-02-02",
    "2025-02-03",
    "2025-02-04",
    "2025-04-04", // 清明节
    "2025-04-05",
    "2025-04-06",
    "2025-05-01", // 劳动节
    "2025-05-02",
    "2025-05-03",
    "2025-05-04",
    "2025-05-05",
    "2025-05-31", // 端午节
    "2025-06-01",
    "2025-06-02",
    "2025-10-01", // 国庆节&中秋节
    "2025-10-02",
    "2025-10-03",
    "2025-10-04",
    "2025-10-05",
    "2025-10-06",
    "2025-10-07",
    "2025-10-08",
];

#[derive(Debug, PartialEq)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
}

pub fn time_info(time: &str) -> String {
    let time = convert_str_to_time(time).unwrap();
    let accumulate_days = accumulate_days(&time);
    let week = week_info(&time);
    let dayofweek = dayofweek_info(&time);
    let spring = nextspring_info(&time);
    let left = daysleft_info(&time);
    let stockmarket = stockmarket_info(&time);
    format!("{week},{dayofweek},{accumulate_days},{left},{spring},{stockmarket}")
}

fn week_info(time: &Time) -> u32 {
    let accumulate_days = accumulate_days(time);
    if accumulate_days < FIRSTMONDAY {
        1 as u32
    } else if accumulate_days > (MAXWEEK - 1) * 7 {
        1 as u32
    } else {
        (accumulate_days - FIRSTMONDAY) / DAYSAWEEK + 2
    }
}

fn accumulate_days(time: &Time) -> u32 {
    let mut result: u32 = 0;
    if time.month != 1 {
        for month in 0..(time.month - 1) {
            result += MONTH_DAYS[month as usize];
        }
    }
    result += time.day as u32;
    result
}

fn dayofweek_info(time: &Time) -> u32 {
    let accumulate_days = accumulate_days(time);
    if accumulate_days > FIRSTSUNDAY {
        let mut dayofweek = (accumulate_days - FIRSTSUNDAY) % DAYSAWEEK;
        if dayofweek == 0 {
            dayofweek = 7
        }
        dayofweek
    } else {
        let mut dayofweek = (accumulate_days + 2) % DAYSAWEEK;
        if dayofweek == 0 {
            dayofweek = 7
        }
        dayofweek
    }
}

fn nextspring_info(time: &Time) -> u32 {
    let accumulate_days = accumulate_days(time);
    if accumulate_days > SPRING2025 {
        DAYS2025 + SPRING2026 - accumulate_days
    } else {
        SPRING2025 - accumulate_days
    }
}

fn daysleft_info(time: &Time) -> u32 {
    let accumulate_days = accumulate_days(time);
    DAYS2025 - accumulate_days
}

fn convert_str_to_time(date: &str) -> Result<Time, &'static str> {
    // 使用 '.' 作为分隔符分割字符串
    let parts: Vec<&str> = date.split('-').collect();

    // 检查是否有三个部分：年、月、日
    if parts.len() != 3 {
        return Err("日期格式不正确，应为 'YYYY-MM-DD'");
    }

    // 解析年份
    let year = parts[0].parse::<u32>().map_err(|_| "无法解析年份")?;

    // 解析月份
    let month = parts[1].parse::<u32>().map_err(|_| "无法解析月份")?;

    // 解析日期
    let day = parts[2].parse::<u32>().map_err(|_| "无法解析日期")?;

    // （可选）验证月份和日期的有效性
    if month < 1 || month > 12 {
        return Err("月份应在1到12之间");
    }

    if day < 1 || day > 31 {
        return Err("日期应在1到31之间");
    }

    Ok(Time { year, month, day })
}

fn holidays_info() -> Vec<Time> {
    let mut holidays: Vec<Time> = Vec::new();
    for holiday in HOLIDAY_DATES {
        let temp = convert_str_to_time(holiday).unwrap();
        holidays.push(temp);
    }
    holidays
}

fn acc_to_time(mut acc: u32) -> Time {
    let mut month: u32 = 1;
    for (index, &month_days) in MONTH_DAYS.iter().enumerate() {
        if acc > month_days {
            acc -= month_days;
        } else {
            month = index as u32 + 1;
            break;
        }
    }
    Time {
        year: 2025,
        month,
        day: acc,
    }
}

fn stockmarket_info(time: &Time) -> u32 {
    let holidays_info = holidays_info();
    let accumulate_days = accumulate_days(time);
    let mut temp: u32 = 0;
    for i in 0..10 {
        let new_time = acc_to_time(accumulate_days + i + 1);
        let dayofweek = dayofweek_info(&new_time);
        if holidays_info.contains(&new_time) || dayofweek == 6 || dayofweek == 7 {
            temp += 1
        } else {
            break;
        }
    }
    temp
}
