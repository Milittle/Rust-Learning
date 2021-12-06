use std::collections::HashMap;

fn main() {
    let mut valid_palindrome: Vec<String> = Vec::new();
    let mut month_days = HashMap::from([
        ("01", "31"),
        ("02", "28"),
        ("03", "31"),
        ("04", "30"),
        ("05", "31"),
        ("06", "30"),
        ("07", "31"),
        ("08", "31"),
        ("09", "30"),
        ("10", "31"),
        ("11", "30"),
        ("12", "31"),
    ]);

    for i in 1..=9999 {
        let full_year = format!("{:0width$}", i, width = 4).to_string();
        let month = &full_year[2..].chars().rev().collect::<String>();
        let day = &full_year[..2].chars().rev().collect::<String>();
        if is_leap_year(full_year.parse::<i32>().unwrap()) {
            let value = month_days.get_mut("02").unwrap();
            *value = "29";
        }
        if is_valid(month, day, &month_days) {
            let s = format!("{} {}-{}", full_year.parse::<i32>().unwrap(), month, day);
            valid_palindrome.push(s);
        }
        let value = month_days.get_mut("02").unwrap();
        *value = "28";
    }
    println!("{:?}", valid_palindrome);
}

#[allow(dead_code)]
fn is_valid(month: &str, day: &str, days: &HashMap<&str, &str>) -> bool {
    if month <= "12" && month != "00" {
        match days.get(month) {
            Some(&d) => {
                if day <= d && day != "00" {
                    return true;
                }
            }
            _ => return false,
        }
    }
    false
}

fn is_leap_year(year: i32) -> bool {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        return true;
    }
    false
}
