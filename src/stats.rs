#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SummaryStats {
    pub week: u32,
    pub month: u32,
    pub year: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct YearData {
    pub from: String,
    pub to: String,
    pub days: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatsFile {
    pub years: Vec<YearData>,
    pub stats: SummaryStats,
}

pub fn build_stats_file() -> Result<StatsFile, std::io::Error> {
    Err(std::io::Error::other(
        "TODO: implement contribution aggregation in Rust.",
    ))
}
