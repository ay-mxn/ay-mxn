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
}

pub fn fetch_contributions_html(_username: &str, _year: i32) -> Result<String, std::io::Error> {
    Err(std::io::Error::other(
        "TODO: implement GitHub contributions fetch in Rust.",
    ))
}

pub fn parse_contributions(_html: &str) -> Vec<DayData> {
    Vec::new()
}
