use super::render_main_svg;
use crate::{
    fonts::FontData,
    stats::{StatsFile, SummaryStats, YearData},
};

#[test]
fn renders_graph_as_an_infinite_loop() {
    let stats_file = sample_stats_file();
    let svg = render_main_svg(&sample_font_data(), &stats_file, 440, "Sep 11, 2026");

    assert!(svg.contains("animation-iteration-count: infinite;"));
    assert_eq!(count_substring(&svg, "class=\"year year--"), 8);
}

fn sample_font_data() -> FontData {
    FontData {
        writer_woff2_base64: String::from("writer"),
        departure_woff2_base64: String::from("departure"),
        arabic_woff2_base64: String::from("arabic"),
    }
}

fn sample_stats_file() -> StatsFile {
    StatsFile {
        years: vec![
            sample_year("2026-01-01T00:00:00.000Z"),
            sample_year("2025-01-01T00:00:00.000Z"),
            sample_year("2024-01-01T00:00:00.000Z"),
            sample_year("2023-01-01T00:00:00.000Z"),
        ],
        stats: SummaryStats {
            week: 1,
            month: 2,
            year: 3,
            total: 4,
        },
    }
}

fn sample_year(from: &str) -> YearData {
    YearData {
        from: String::from(from),
        to: String::from("2027-01-01T00:00:00.000Z"),
        days: vec![0, 1, 2, 3, 4, 0],
    }
}

fn count_substring(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}
