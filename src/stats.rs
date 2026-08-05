use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

pub const MAX_RENDER_YEARS: usize = 3;

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
    serde_json::from_str(json_text).map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))
}

fn serialize_stats_file(stats_file: &StatsFile) -> io::Result<String> {
    serde_json::to_string(stats_file).map_err(io::Error::other)
}
