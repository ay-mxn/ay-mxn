use super::{DayData, parse_contributions, parse_heading_total};

#[test]
fn parses_cells_with_counts_and_zero_days() {
    let html = r#"
        <td id="contribution-day-component-0-0" data-date="2026-08-03" data-level="4"></td>
        <td id="contribution-day-component-0-1" data-date="2026-08-04" data-level="0"></td>
        <tool-tip for="contribution-day-component-0-0">12 contributions on Aug 3, 2026</tool-tip>
        <tool-tip for="contribution-day-component-0-1">No contributions on Aug 4, 2026</tool-tip>
    "#;

    let days = parse_contributions(html).expect("parser should succeed");

    assert_eq!(
        days,
        vec![
            DayData {
                date: String::from("2026-08-03"),
                level: 4,
                count: 12,
            },
            DayData {
                date: String::from("2026-08-04"),
                level: 0,
                count: 0,
            },
        ]
    );
}

#[test]
fn parses_reversed_attribute_order() {
    let html = r#"
        <td data-date="2026-08-05" id="contribution-day-component-1-0" data-level="2"></td>
        <tool-tip for="contribution-day-component-1-0">3 contributions on Aug 5, 2026</tool-tip>
    "#;

    let days = parse_contributions(html).expect("parser should succeed");

    assert_eq!(days.len(), 1);
    assert_eq!(days[0].date, "2026-08-05");
    assert_eq!(days[0].level, 2);
    assert_eq!(days[0].count, 3);
}

#[test]
fn parses_heading_total() {
    let html = "<h2>1,570 contributions in the last year</h2>";
    let total = parse_heading_total(html).expect("heading total should parse");
    assert_eq!(total, 1_570);
}
