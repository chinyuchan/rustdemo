#![allow(dead_code, unused)]
use chrono::{Datelike, FixedOffset, Local, TimeZone};

fn main() {
    let now = Local::now();
    let dt = FixedOffset::from_offset(&now.offset())
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();
    println!("当天零时时间戳（不含时区）: {}", dt.timestamp());
}
