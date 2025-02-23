fn calculate_weekday(year: i32, month: i32, day: i32) -> i32 {
    let mut y = year;
    let mut m = month;
    let q = day;

    if m < 3 {
        m += 12;
        y -= 1;
    }

    let k = y % 100;
    let j = y / 100;

    let h = (q + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
    let res = (h + 5) % 7;
    if res == 0 {
        return 7;
    }else {
        res + 1
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_of_year(year: i32, month: i32, day: i32) -> i32 {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_count = 0;

    for m in 1..month {
        day_count += days_in_month[m as usize - 1];
    }

    if month > 2 && is_leap_year(year) {
        day_count += 1;
    }

    day_count + day
}

fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let day_of_year = days_of_year(year, month, day);
    (day_of_year + 1 ) / 7 + 1
}


fn is_holiday(year: i32, month: i32, day: i32) -> bool {
    let holidays = vec![
        (2025, 1, 28), (2025, 1, 29), (2025, 1, 30), (2025, 1, 31), (2025, 2, 1), (2025, 2, 2), (2025, 2, 3), (2025, 2, 4),
        (2025, 10, 1), (2025, 10, 2), (2025, 10, 3), (2025, 10, 4), (2025, 10, 5), (2025, 10, 6), (2025, 10, 7),
    ];
    holidays.contains(&(year, month, day))
}

fn is_trading_day(year: i32, month: i32, day: i32) -> bool {
    let weekday = calculate_weekday(year, month, day);
    !is_holiday(year, month, day) && weekday != 1 && weekday != 7 // 非节假日且非周末
}

fn add_one_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut new_day = day + 1;
    let mut new_month = month;
    let mut new_year = year;

    if new_day > days_in_month[month as usize - 1] + if month == 2 && is_leap_year(year) { 1 } else { 0 } {
        new_day = 1;
        new_month += 1;

        if new_month > 12 {
            new_month = 1;
            new_year += 1;
        }
    }

    (new_year, new_month, new_day)
}
fn days_to_next_a_share_opening(year: i32, month: i32, day: i32) -> i32 {
    let new_year_day_open = days_of_year(year, 1, 2);
    let spring_year_day_open = days_of_year(year, 2, 5);
    let qingming_day_open = days_of_year(year, 4, 7);
    let labor_day_open = days_of_year(year, 5, 6);
    let zongzi_day_open = days_of_year(year, 6, 3);
    let autumn_day_open = days_of_year(year, 10, 9);
    let next_new_year_day_open = days_of_year(year + 1, 1, 1);

    // 处理节假日
    if month == 1 && day == 1 {
        return new_year_day_open - days_of_year(year, month, day) - 1;
    }
    if (month == 1 && (28..=31).contains(&day)) || (month == 2 && (1..=4).contains(&day)) {
        return spring_year_day_open - days_of_year(year, month, day) - 1;
    }
    if month == 4 && (4..=6).contains(&day) {
        return qingming_day_open - days_of_year(year, month, day) - 1;
    }
    if month == 5 && (1..=5).contains(&day) {
        return labor_day_open - days_of_year(year, month, day) - 1;
    }
    if (month == 5 && day == 31) || (month == 6 && (1..=2).contains(&day)) {
        return zongzi_day_open - days_of_year(year, month, day) - 1;
    }
    if month == 10 && (1..=8).contains(&day) {
        return autumn_day_open - days_of_year(year, month, day) - 1;
    }
    if month == 12 && day == 31 {
        let days = if is_leap_year(year) { 366 } else { 365 };
        return next_new_year_day_open - days_of_year(year, month, day) + days;
    }

    let weekday = calculate_weekday(year, month, day);
    match weekday {
        7 => 0, // 周日
        5 => 2, // 周五，距离下周一开盘 2 天
        6 => 1, // 周六，距离周一开盘 1 天
        _ => 0, // 周一到周四
    }
}

fn next_trading_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let mut current_year = year;
    let mut current_month = month;
    let mut current_day = day;

    while !is_trading_day(current_year, current_month, current_day) {
        let (new_year, new_month, new_day) = add_one_day(current_year, current_month, current_day);
        current_year = new_year;
        current_month = new_month;
        current_day = new_day;
    }

    (current_year, current_month, current_day)
}
pub fn time_info(time: &str) -> String {
    let times:Vec<&str> = time.split("-").collect();
    let year:i32 = times[0].parse().unwrap_or_default();
    let month:i32 = times[1].parse().unwrap_or_default();
    let day:i32 = times[2].parse().unwrap_or_default();
    let mut week_of_year = week_of_year(year, month, day);
    let day_of_week = calculate_weekday(year, month, day);
    let day_of_year = days_of_year(year, month, day);
    let day_Rest = if is_leap_year(year){
        366 - day_of_year
    }else{
        365 - day_of_year
    };
    let day_lunar = if day_of_year > 29 {
        413 - day_of_year
    }else{
        29 - day_of_year
    };
    let (next_year, next_month, next_day) = next_trading_day(year, month, day);
    //let next_trade_day = day_of_year - days_of_year(next_year, next_month, next_day);
    let mut days_to_trading = days_to_next_a_share_opening(year, month, day);
    if week_of_year >= 53 {
        week_of_year = 1;
    }
    format!(
        "{},{},{},{},{},{}",
        week_of_year,
        day_of_week,
        day_of_year,
        day_Rest,
        day_lunar,
        days_to_trading
    )
}

