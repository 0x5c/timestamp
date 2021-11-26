/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use chrono::SecondsFormat;
use crossterm::style::{
    Stylize,
    style
};

use crate::types::{
    DT,
    DiscordID,
    Operation,
    TimeSource,
    TwitterSnowflake
};


pub fn printit(op: Operation, source: TimeSource) {
    println!("{} {}", "Using".dark_green(), style(source).dark_green());
    match op {
        Operation::Timestamp(time) => print_timestamp(time),
        Operation::Discord(id) => print_discord(id),
        Operation::Twitter(sf) => print_twitter(sf),
    }
}

macro_rules! iso8601 {
    ($time:expr, $format:expr) => {
        println!("{} {}",
            "ISO 8601/RFC 3339:".dark_cyan(),
            $time.to_rfc3339_opts($format, true).blue()
        );
    };
}

macro_rules! rfc2822 {
    ($time:expr) => {
        println!("{} {}", "RFC 2822:".dark_cyan(), $time.to_rfc2822().blue());
    };
}

macro_rules! ts_info {
    ($time:expr, $format:expr) => {
        iso8601!($time, $format);
        rfc2822!($time);
        println!("{}", combo_ts_line($time));
    };
}

fn print_timestamp(time: DT) {
    ts_info!(time, SecondsFormat::AutoSi);
}

fn print_discord(id: DiscordID) {
    ts_info!(id.time, SecondsFormat::AutoSi);
    println!("{} {}", "Discord internal worker ID:".dark_cyan(), style(id.wid).blue());
    println!("{} {}", "Discord internal process ID:".dark_cyan(), style(id.pid).blue());
    println!("{} {}", "Discord internal sequential number:".dark_cyan(), style(id.seq).blue());
}

fn print_twitter(sf: TwitterSnowflake) {
    ts_info!(sf.time, SecondsFormat::AutoSi);
    println!("{} {}", "Twitter internal machine ID:".dark_cyan(), style(sf.mid).blue());
    println!("{} {}", "Twitter internal sequential number:".dark_cyan(), style(sf.seq).blue());
}

fn prepare_combined_ts(time: DT) -> Option<String> {
    let num = format!("{:0>10}", safe_timestamp_nanos(time)?.to_string());
    let (sec, txt) = num.split_at(num.len()-9);
    let (ms, txt2) = txt.split_at(3);
    let (us, ns) = txt2.split_at(3);
    Some(format!("{}{}{}{}", sec.blue(), ms.dark_yellow(), us.red(), ns.dark_magenta()))
}

fn combo_ts_line(time: DT) -> String {
    match prepare_combined_ts(time) {
        Some(ts) => {
            let u = format!("{0}{4}{1}{4}{2}{4}{3}", 
                            "s".blue(),
                            "ms".dark_yellow(),
                            "µs".red(),
                            "ns".dark_magenta(),
                            "/".dark_cyan());
            format!("{} {} {}", "Unix timestamp:".dark_cyan(), ts, u)
        },
        None => format!("{} {}","Unix timestamp:".dark_cyan(), "[time out of bounds]".red()),
    }
}

fn safe_timestamp_millis(dt: DT) -> Option<i64> {
    let as_ns = dt.timestamp().checked_mul(1_000)?;
    as_ns.checked_add(i64::from(dt.timestamp_subsec_millis()))
}

fn safe_timestamp_micros(dt: DT) -> Option<i64> {
    let as_ns = dt.timestamp().checked_mul(1_000_000)?;
    as_ns.checked_add(i64::from(dt.timestamp_subsec_micros()))
}

fn safe_timestamp_nanos(dt: DT) -> Option<i64> {
    let as_ns = dt.timestamp().checked_mul(1_000_000_000)?;
    as_ns.checked_add(i64::from(dt.timestamp_subsec_nanos()))
}


#[cfg(test)]
mod tests {
    use chrono::{
        DateTime,
        NaiveDateTime,
        Utc
    };

    use super::{
        safe_timestamp_millis,
        safe_timestamp_micros,
        safe_timestamp_nanos,
    };

    #[test]
    fn ts_millis() {
        let ndt = NaiveDateTime::from_timestamp_opt(1632802669, 990000000).unwrap();
        let dt = DateTime::<Utc>::from_utc(ndt, Utc);

        assert_eq!(safe_timestamp_millis(dt).unwrap(), 1632802669_990);
    }

    #[test]
    fn ts_micros() {
        let ndt = NaiveDateTime::from_timestamp_opt(1632802669, 990574000).unwrap();
        let dt = DateTime::<Utc>::from_utc(ndt, Utc);

        assert_eq!(safe_timestamp_micros(dt).unwrap(), 1632802669_990_574);
    }

    #[test]
    fn ts_nanos() {
        let ndt = NaiveDateTime::from_timestamp_opt(1632802669, 990574670).unwrap();
        let dt = DateTime::<Utc>::from_utc(ndt, Utc);

        assert_eq!(safe_timestamp_nanos(dt).unwrap(), 1632802669_990_574_670);
    }
}
