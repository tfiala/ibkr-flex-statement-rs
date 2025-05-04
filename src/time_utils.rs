use anyhow::Result;
use chrono::{NaiveDate, /* NaiveDateTime, Timelike, */ TimeZone};
use chrono_tz::Tz;
pub fn timestamp_ms_at_hour(date: &str, timezone: Tz, hour: u32) -> Result<i64> {
    let naive_dt = NaiveDate::parse_from_str(date, "%Y-%m-%d")?
        .and_hms_opt(hour, 0, 0)
        .unwrap();
    let tz_aware_dt = timezone.from_local_datetime(&naive_dt).unwrap();
    Ok(tz_aware_dt.timestamp() * 1000)
}

/*

pub fn timestamp_ms_at_hour_minute(date: &str, timezone: Tz, hour: u32, minute: u32) -> Result<i64> {
    let naive_dt = NaiveDate::parse_from_str(date, "%Y-%m-%d")?.and_hms_opt(hour, minute, 0).unwrap();
    let tz_aware_dt = timezone.from_local_datetime(&naive_dt).unwrap();
    Ok(tz_aware_dt.timestamp() * 1000)
}

pub fn trading_sod_pre_market_timestamp_ms(date: &str) -> Result<i64> {
    timestamp_ms_at_hour(date, chrono_tz::America::New_York, 4)
}

 */

pub fn trading_eod_after_hours_timestamp_ms(date: &str) -> Result<i64> {
    timestamp_ms_at_hour(date, chrono_tz::America::New_York, 20)
}

/*
pub fn trading_sod_timestamp_ms(date: &str) -> Result<i64> {
    timestamp_ms_at_hour_minute(date, chrono_tz::America::New_York, 9, 30)
}

pub fn trading_eod_timestamp_ms(date: &str) -> Result<i64> {
    timestamp_ms_at_hour(date, chrono_tz::America::New_York, 20)
}

 */
