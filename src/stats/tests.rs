use super::{
    START_YEAR, build_stats_file_from_scraped_years, format_date, format_month_start,
    most_recent_sunday,
};
use crate::github::{DayData, ScrapedYear};
use time::{Date, Month, OffsetDateTime, Time};

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

#[test]
fn total_tracks_all_scraped_contributions() {
    let now = OffsetDateTime::new_utc(
        Date::from_calendar_date(2026, Month::August, 7).expect("valid date"),
        Time::MIDNIGHT,
    );
    let scraped_years = vec![
        ScrapedYear {
            year: 2026,
            days: vec![
                DayData {
                    date: String::from("2026-08-04"),
                    level: 4,
                    count: 5,
                },
                DayData {
                    date: String::from("2026-08-05"),
                    level: 1,
                    count: 1,
                },
            ],
            total: 6,
        },
        ScrapedYear {
            year: 2025,
            days: vec![DayData {
                date: String::from("2025-01-10"),
                level: 2,
                count: 9,
            }],
            total: 9,
        },
    ];

    let stats_file =
        build_stats_file_from_scraped_years(&scraped_years, now).expect("stats file should build");

    assert_eq!(stats_file.stats.total, 15);
}
