use std::{collections::BTreeMap, io, sync::OnceLock};

use regex::Regex;
use ureq::{
    Agent,
    tls::{RootCerts, TlsConfig},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayData {
    pub date: String,
    pub level: u8,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapedYear {
    pub year: i32,
    pub days: Vec<DayData>,
    pub total: u32,
}

pub fn fetch_contributions_html(username: &str, year: i32) -> io::Result<String> {
    let url = format!(
        "https://github.com/users/{username}/contributions?from={year}-01-01&to={year}-12-31"
    );
    fetch_html(username, &url)
}

fn fetch_html(username: &str, url: &str) -> io::Result<String> {
    let user_agent = format!("{username}/readme");
    let response = github_agent()
        .get(url)
        .header("User-Agent", &user_agent)
        .header("Accept", "text/html")
        .call()
        .map_err(http_error)?;

    response
        .into_body()
        .read_to_string()
        .map_err(io::Error::other)
}

fn github_agent() -> &'static Agent {
    static GITHUB_AGENT: OnceLock<Agent> = OnceLock::new();
    GITHUB_AGENT.get_or_init(|| {
        Agent::config_builder()
            .tls_config(
                TlsConfig::builder()
                    .root_certs(RootCerts::PlatformVerifier)
                    .build(),
            )
            .build()
            .new_agent()
    })
}

pub fn parse_contributions(html: &str) -> io::Result<Vec<DayData>> {
    let mut cell_map = BTreeMap::new();

    for captures in cell_pattern().captures_iter(html) {
        let component_id = capture_text(&captures, 1, "component id")?;
        let date = capture_text(&captures, 2, "date")?;
        let level = capture_level(&captures, 3)?;
        cell_map.insert(component_id, (date, level));
    }

    for captures in reversed_cell_pattern().captures_iter(html) {
        let date = capture_text(&captures, 1, "date")?;
        let component_id = capture_text(&captures, 2, "component id")?;
        let level = capture_level(&captures, 3)?;
        cell_map.insert(component_id, (date, level));
    }

    if cell_map.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "failed to parse contribution day cells from GitHub HTML",
        ));
    }

    let mut count_map = BTreeMap::new();

    for captures in count_pattern().captures_iter(html) {
        let component_id = capture_text(&captures, 1, "tooltip component id")?;
        let count = capture_count(&captures, 2)?;
        count_map.insert(component_id, count);
    }

    for captures in zero_count_pattern().captures_iter(html) {
        let component_id = capture_text(&captures, 1, "tooltip component id")?;
        count_map.insert(component_id, 0);
    }

    let mut days = Vec::with_capacity(cell_map.len());
    for (component_id, (date, level)) in cell_map {
        days.push(DayData {
            date,
            level,
            count: count_map.remove(&component_id).unwrap_or(0),
        });
    }

    days.sort_by(|left, right| left.date.cmp(&right.date));
    Ok(days)
}

pub fn parse_heading_total(html: &str) -> io::Result<u32> {
    let Some(captures) = heading_total_pattern().captures(html) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "failed to parse contribution heading total from GitHub HTML",
        ));
    };
    let count_text = capture_text(&captures, 1, "heading contribution total")?;
    let normalized = count_text.replace(',', "");
    normalized
        .parse::<u32>()
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))
}

fn cell_pattern() -> &'static Regex {
    static CELL_PATTERN: OnceLock<Regex> = OnceLock::new();
    CELL_PATTERN.get_or_init(|| {
        Regex::new(
            r#"id="(contribution-day-component-\d+-\d+)"[^>]*data-date="(\d{4}-\d{2}-\d{2})"[^>]*data-level="(\d)""#,
        )
        .expect("cell regex must compile")
    })
}

fn reversed_cell_pattern() -> &'static Regex {
    static REVERSED_CELL_PATTERN: OnceLock<Regex> = OnceLock::new();
    REVERSED_CELL_PATTERN.get_or_init(|| {
        Regex::new(
            r#"data-date="(\d{4}-\d{2}-\d{2})"[^>]*id="(contribution-day-component-\d+-\d+)"[^>]*data-level="(\d)""#,
        )
        .expect("reversed cell regex must compile")
    })
}

fn count_pattern() -> &'static Regex {
    static COUNT_PATTERN: OnceLock<Regex> = OnceLock::new();
    COUNT_PATTERN.get_or_init(|| {
        Regex::new(r#"for="(contribution-day-component-\d+-\d+)"[^>]*>(\d+) contributions? on"#)
            .expect("count regex must compile")
    })
}

fn zero_count_pattern() -> &'static Regex {
    static ZERO_COUNT_PATTERN: OnceLock<Regex> = OnceLock::new();
    ZERO_COUNT_PATTERN.get_or_init(|| {
        Regex::new(r#"for="(contribution-day-component-\d+-\d+)"[^>]*>No contributions on"#)
            .expect("zero count regex must compile")
    })
}

fn heading_total_pattern() -> &'static Regex {
    static HEADING_TOTAL_PATTERN: OnceLock<Regex> = OnceLock::new();
    HEADING_TOTAL_PATTERN.get_or_init(|| {
        Regex::new(r#"([\d,]+)\s+contributions"#).expect("heading regex must compile")
    })
}

fn capture_text(captures: &regex::Captures<'_>, index: usize, label: &str) -> io::Result<String> {
    let Some(capture) = captures.get(index) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing regex capture for {label}"),
        ));
    };

    Ok(capture.as_str().to_owned())
}

fn capture_level(captures: &regex::Captures<'_>, index: usize) -> io::Result<u8> {
    let level_text = capture_text(captures, index, "level")?;
    let level = level_text
        .parse::<u8>()
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))?;
    if level <= 4 {
        return Ok(level);
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("unexpected contribution level {level}"),
    ))
}

fn capture_count(captures: &regex::Captures<'_>, index: usize) -> io::Result<u32> {
    let count_text = capture_text(captures, index, "count")?;
    count_text
        .parse::<u32>()
        .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))
}

fn http_error(source: ureq::Error) -> io::Error {
    io::Error::other(source.to_string())
}

#[cfg(test)]
mod tests;
