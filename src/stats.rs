use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use time::{Date, Month, OffsetDateTime, Weekday};

use crate::github::{
    ScrapedYear, fetch_contributions_html, fetch_last_twelve_months_html, parse_contributions,
    parse_heading_total,
};

pub const MAX_RENDER_YEARS: usize = 3;
pub const START_YEAR: i32 = 2024;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryStats {
    pub week: u32,
    pub month: u32,
    pub year: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct YearData {
    pub from: String,
    pub to: String,
    pub days: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatsFile {
    pub years: Vec<YearData>,
    pub stats: SummaryStats,
}

pub fn build_stats_file(username: &str, now: OffsetDateTime) -> io::Result<StatsFile> {
    let today = now.date();
    let current_year = today.year();
    let today_text = format_date(today);
    let week_start_text = format_date(most_recent_sunday(today));
    let month_start_text = format_month_start(today);
    let current_year_start_text = format!("{current_year}-01-01");
    let now_timestamp = format_timestamp(now);

    assert!(
        current_year >= START_YEAR,
        "expected current year {current_year} to be >= start year {START_YEAR}",
    );

    let year_span = usize::try_from(current_year - START_YEAR + 1)
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))?;
    let mut years = Vec::with_capacity(year_span);
    let mut week_count = 0_u32;
    let mut month_count = 0_u32;
    let mut year_count = 0_u32;

    for year in (START_YEAR..=current_year).rev() {
        let scraped_year = scrape_year(username, year, &today_text)?;
        years.push(YearData {
            from: format!("{year}-01-01T00:00:00.000Z"),
            to: if year == current_year {
                now_timestamp.clone()
            } else {
                format!("{}-01-01T00:00:00.000Z", year + 1)
            },
            days: scraped_year
                .days
                .iter()
                .map(|day| day.level)
                .rev()
                .collect(),
        });

        for day in &scraped_year.days {
            if day.count == 0 {
                continue;
            }
            let day_text = day.date.as_str();
            if day_text >= week_start_text.as_str() && day_text <= today_text.as_str() {
                week_count = week_count.saturating_add(day.count);
            }
            if day_text >= month_start_text.as_str() && day_text <= today_text.as_str() {
                month_count = month_count.saturating_add(day.count);
            }
            if day_text >= current_year_start_text.as_str() && day_text <= today_text.as_str() {
                year_count = year_count.saturating_add(day.count);
            }
        }
    }

    let last_twelve_months_html = fetch_last_twelve_months_html(username)?;
    let last_twelve_months_total = parse_heading_total(&last_twelve_months_html)?;

    Ok(StatsFile {
        years,
        stats: SummaryStats {
            week: week_count,
            month: month_count,
            year: year_count,
            total: last_twelve_months_total,
        },
    })
}

pub fn read_stats_file(stats_path: &Path) -> io::Result<StatsFile> {
    let json_text = fs::read_to_string(stats_path)?;
    deserialize_stats_file(&json_text)
}

pub fn write_stats_file(stats_path: &Path, stats_file: &StatsFile) -> io::Result<()> {
    let json_text = serialize_stats_file(stats_file)?;
    fs::write(stats_path, json_text)
}

pub fn render_years(stats_file: &StatsFile) -> &[YearData] {
    let year_count = stats_file.years.len();
    assert!(
        year_count >= MAX_RENDER_YEARS,
        "expected at least {MAX_RENDER_YEARS} years of contribution data, got {year_count}",
    );

    &stats_file.years[..MAX_RENDER_YEARS]
}

fn deserialize_stats_file(json_text: &str) -> io::Result<StatsFile> {
    serde_json::from_str(json_text)
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))
}

fn serialize_stats_file(stats_file: &StatsFile) -> io::Result<String> {
    serde_json::to_string(stats_file).map_err(io::Error::other)
}

fn scrape_year(username: &str, year: i32, today_text: &str) -> io::Result<ScrapedYear> {
    let html = fetch_contributions_html(username, year)?;
    let all_days = parse_contributions(&html)?;
    let heading_total = parse_heading_total(&html)?;
    let year_prefix = format!("{year}-");
    let mut days = Vec::with_capacity(all_days.len());

    for day in all_days {
        if day.date.starts_with(&year_prefix) && day.date.as_str() <= today_text {
            days.push(day);
        }
    }

    Ok(ScrapedYear {
        year,
        days,
        total: heading_total,
    })
}

fn most_recent_sunday(today: Date) -> Date {
    let offset_days = weekday_offset(today.weekday());
    today - time::Duration::days(i64::from(offset_days))
}

fn weekday_offset(weekday: Weekday) -> u8 {
    match weekday {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

fn format_month_start(date: Date) -> String {
    format!("{:04}-{:02}-01", date.year(), month_number(date.month()))
}

fn format_date(date: Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        month_number(date.month()),
        date.day(),
    )
}

fn format_timestamp(timestamp: OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        timestamp.year(),
        month_number(timestamp.month()),
        timestamp.day(),
        timestamp.hour(),
        timestamp.minute(),
        timestamp.second(),
        timestamp.millisecond(),
    )
}

fn month_number(month: Month) -> u8 {
    month as u8
}

#[cfg(test)]
mod tests {
    use super::{START_YEAR, format_date, format_month_start, most_recent_sunday};
    use time::{Date, Month, Time};

    #[test]
    fn computes_most_recent_sunday() {
        let friday = Date::from_calendar_date(2026, Month::August, 7).expect("valid date");
        let sunday = most_recent_sunday(friday);
        assert_eq!(format_date(sunday), "2026-08-02");
    }

    #[test]
    fn formats_month_start() {
        let date = Date::from_calendar_date(2026, Month::August, 7).expect("valid date");
        assert_eq!(format_month_start(date), "2026-08-01");
    }

    #[test]
    fn start_year_is_reasonable() {
        let now = Date::from_calendar_date(2026, Month::August, 7)
            .expect("valid date")
            .with_time(Time::MIDNIGHT)
            .assume_utc();
        assert!(now.year() >= START_YEAR);
    }
}
