use crate::{
    content::{BIO, IDENTITY},
    fonts::{FontData, font_faces_css},
    stats::{StatsFile, SummaryStats, YearData, render_years},
};

const COLOR_TEXT: &str = "#c4a478";
const COLOR_DOT_0: &str = "#171B21";
const COLOR_DOT_1: &str = "#3d3028";
const COLOR_DOT_2: &str = "#6b5038";
const COLOR_DOT_3: &str = "#9a7850";
const COLOR_DOT_4: &str = "#c4a478";
const COLOR_DOT_BORDER: &str = "rgba(255,255,255,0.04)";
const BREAKPOINT_MEDIUM_PX: u32 = 550;
const DOT_ROWS: usize = 6;
const DOT_SIZE_PX: usize = 24;
const DOT_GAP_PX: usize = 5;
const YEAR_GAP_PX: usize = 5;
const LABEL_HEIGHT_PX: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct YearLayout {
    width_px: usize,
    height_px: usize,
}

pub fn render_top_svg(font_data: &FontData, height_px: u32) -> String {
    let height_text = height_px.to_string();
    let styles = format!(
        "{}\n    :root {{ --size-height: {}; }}\n\n    .wrapper {{\n      display: flex;\n      \
         justify-content: space-between;\n      align-items: center;\n      padding: 0 2px;\n    \
         }}\n\n    .reach {{\n      --delay: var(--animate-in-links-delay);\n      white-space: \
         nowrap;\n    }}\n    .reach-inner {{\n      font-family: 'Departure-Mono', monospace;\n      \
         font-size: 9px;\n      opacity: 0.4;\n      display: flex;\n      align-items: center;\n      \
         gap: 3px;\n    }}\n    .reach-dot {{\n      width: 3px;\n      height: 3px;\n      display: \
         inline-block;\n      background: var(--color-text);\n    }}\n\n    .meta {{\n      --delay: \
         var(--animate-in-org-delay);\n      display: flex;\n      align-items: baseline;\n      gap: \
         12px;\n      flex-wrap: wrap;\n      justify-content: flex-end;\n    }}\n    .meta-item {{\n      \
         display: flex;\n      align-items: baseline;\n      gap: 4px;\n      white-space: nowrap;\n    \
         }}\n    .meta-label {{\n      font-family: 'Departure-Mono', monospace;\n      font-size: \
         9px;\n      opacity: 0.4;\n    }}\n    .meta-val {{\n      font-family: 'Writer', Georgia, \
         serif;\n      font-size: 11px;\n    }}\n    .meta-dot {{\n      width: 3px;\n      height: \
         3px;\n      display: inline-block;\n      margin-right: 2px;\n      vertical-align: middle;\n      \
         background: var(--color-text);\n    }}\n    .arabic {{ font-family: 'Arabic', serif; }}\n",
        shared_styles(font_data),
        height_text,
    );
    let html = format!(
        "<div class=\"wrapper\">\n      <div class=\"reach fade-in\"><span \
         class=\"reach-inner\"><span class=\"reach-dot\"></span>links</span></div>\n      <div \
         class=\"meta fade-in\">\n        <span class=\"meta-item\">\n          <span \
         class=\"meta-label\"><span class=\"meta-dot\"></span>name</span>\n          <span \
         class=\"meta-val\">{} <span class=\"arabic\">⌊{}⌋</span></span>\n        </span>\n        \
         <span class=\"meta-item\">\n          <span class=\"meta-label\"><span \
         class=\"meta-dot\"></span>title</span>\n          <span class=\"meta-val\">{}</span>\n        \
         </span>\n        <span class=\"meta-item\">\n          <span class=\"meta-label\"><span \
         class=\"meta-dot\"></span>org</span>\n          <span class=\"meta-val\">{}</span>\n        \
         </span>\n      </div>\n    </div>",
        escape_html(IDENTITY.name),
        escape_html(IDENTITY.name_arabic),
        escape_html(IDENTITY.title),
        escape_html(IDENTITY.org),
    );

    svg_document(None, &height_text, None, &styles, &html)
}

pub fn render_link_svg(
    font_data: &FontData,
    label: &str,
    width_px: u32,
    height_px: u32,
    index: usize,
) -> String {
    let width_text = width_px.to_string();
    let height_text = height_px.to_string();
    let arrow_delay_s = deterministic_delay_tenths(index * 13, 50);
    let shine_delay_s = deterministic_delay_tenths(index * 17, 100);
    let styles = format!(
        "{}\n    :root {{\n      --size-height: {};\n      --size-width: {};\n      --i: {};\n    \
         }}\n\n    .wrapper {{\n      --delay: calc(var(--animate-in-links-delay) + var(--i) * \
         1.2s);\n    }}\n    @-moz-document url-prefix() {{\n      .wrapper {{ display: block; }}\n    \
         }}\n\n    .link {{\n      font-family: 'Writer', Georgia, serif;\n      font-size: 12px;\n      \
         display: flex;\n      justify-content: start;\n      align-items: center;\n      gap: 2px;\n    \
         }}\n    .link__arrow {{\n      font-size: 9px;\n      position: relative;\n      \
         inset-block-start: 0.5px;\n      animation-name: rotate;\n      animation-duration: 5s;\n      \
         animation-timing-function: ease-in-out;\n      animation-iteration-count: infinite;\n      \
         animation-delay: {};\n    }}\n    @keyframes rotate {{\n      0% {{ transform: rotate(0deg); \
         }}\n      10%, 100% {{ transform: rotate(360deg); }}\n    }}\n",
        shared_styles(font_data),
        height_text,
        width_text,
        index,
        arrow_delay_s,
    );
    let html = format!(
        "<main class=\"wrapper\">\n      <a class=\"link fade-in\">\n        <div \
         class=\"link__label shine\" style=\"animation-delay: {}\">{}</div>\n        <div \
         class=\"link__arrow\">↗</div>\n      </a>\n    </main>",
        shine_delay_s,
        escape_html(label),
    );

    svg_document(Some(&width_text), &height_text, None, &styles, &html)
}

pub fn render_main_svg(
    font_data: &FontData,
    stats_file: &StatsFile,
    height_px: u32,
    today_label: &str,
) -> String {
    let years = render_years(stats_file);
    let layouts = build_year_layouts(years);
    assert!(!layouts.is_empty(), "expected at least one year layout");

    let height_text = height_px.to_string();
    let graph_height_px = layouts[0].height_px;
    let total_graph_width_px = total_graph_width(&layouts);
    let styles = main_styles(font_data, &height_text);
    let bio_html = render_bio_chars(BIO);
    let years_html = render_years_html(years, &layouts, today_label);
    let html = format!(
        "<main class=\"wrapper\">\n      <article class=\"stats fade-in\">\n        <div \
         class=\"stats-title\"><span class=\"stats-dot\"></span>contributions</div>\n        {}\n      \
         </article>\n\n      <article class=\"intro\">\n        <p>{}</p>\n      </article>\n\n      \
         <article class=\"graph\">\n        <div class=\"years\" style=\"--w: {}; --h: {};\">\n          \
         {}\n        </div>\n      </article>\n    </main>",
        render_stat_rows(&stats_file.stats),
        bio_html,
        total_graph_width_px,
        graph_height_px,
        years_html,
    );

    svg_document(None, &height_text, None, &styles, &html)
}

fn main_styles(font_data: &FontData, height_text: &str) -> String {
    format!(
        "{}\n    :root {{\n      --rows: {};\n      --size-width: 100cqw;\n      --size-height: {};\n      \
         --size-dot-gap: {};\n      --size-dot: {};\n      --size-year-gap: {};\n      \
         --size-label-height: {};\n    }}\n\n    .wrapper {{\n      display: grid;\n      \
         grid-template-columns: repeat(6, 1fr);\n      grid-template-rows: 1fr auto;\n      row-gap: \
         10px;\n      padding: 0 2px 4px;\n    }}\n\n    .stats {{\n      --delay: \
         var(--animate-in-stats-delay);\n      grid-column: 1 / 3;\n      grid-row: 1;\n      \
         align-self: end;\n      padding-bottom: 6px;\n      font-family: 'Departure-Mono', monospace;\n      \
         font-size: 11px;\n      line-height: 17px;\n    }}\n    .stats-title {{\n      font-size: 9px;\n      \
         opacity: 0.4;\n      margin-bottom: 3px;\n      display: flex;\n      align-items: center;\n      \
         gap: 3px;\n    }}\n    .stats-dot {{\n      width: 3px;\n      height: 3px;\n      background: \
         var(--color-text);\n      display: inline-block;\n    }}\n    .stat-row {{\n      display: \
         flex;\n      align-items: baseline;\n      justify-content: space-between;\n    }}\n    .stat-num \
         {{ font-weight: 500; flex-shrink: 0; }}\n    .stat-line {{\n      flex: 1;\n      margin: 0 6px;\n      \
         border-bottom: 0.5px solid var(--color-text);\n      opacity: 0.12;\n      align-self: baseline;\n      \
         margin-bottom: 3px;\n    }}\n    .stat-label {{ opacity: 0.4; font-size: 9px; flex-shrink: 0; }}\n\n    \
         .intro {{\n      grid-column: 4 / 7;\n      grid-row: 1;\n      font-family: 'Writer', Georgia, \
         serif;\n      font-size: 13px;\n      font-weight: 300;\n      line-height: 1.55;\n      \
         align-self: end;\n      padding-bottom: 6px;\n      overflow-wrap: break-word;\n      word-wrap: \
         break-word;\n    }}\n    .intro span {{\n      contain: content;\n      --duration: 980ms;\n      \
         --delay: calc(var(--animate-in-copy-delay) + var(--i) * 5ms);\n    }}\n\n    @container (width > \
         {}px) {{\n      .intro {{ font-size: 14px; }}\n    }}\n\n    .graph {{\n      --delay: \
         var(--animate-in-graph-delay);\n      grid-column: 1 / 7;\n      grid-row: 2;\n    }}\n    \
         .years {{\n      --_w: var(--w);\n      --_h: calc(var(--h) + var(--size-label-height));\n      \
         display: flex;\n      gap: calc(var(--size-year-gap) * 1px);\n      contain: strict;\n      \
         inline-size: calc(var(--_w) * 1px);\n      block-size: calc(var(--_h) * 1px);\n      \
         will-change: transform;\n      backface-visibility: hidden;\n      transform: translateZ(0);\n      \
         animation-name: scroll, fade-in;\n      animation-timing-function: linear, ease-out;\n      \
         animation-duration: calc(30s + (var(--_w) * 0.06s)), 2.5s;\n      animation-fill-mode: both, \
         both;\n      animation-delay: 2s, var(--animate-in-graph-delay);\n    }}\n    @keyframes scroll \
         {{\n      0% {{ transform: translateX(0); }}\n      100% {{ transform: translateX(calc(-100% + \
         100cqw)); }}\n    }}\n    .year {{\n      contain: strict;\n      content-visibility: auto;\n      \
         inline-size: calc(var(--w) * 1px);\n      block-size: calc(var(--_h) * 1px);\n    }}\n    \
         .year__label {{\n      contain: strict;\n      block-size: calc(var(--size-label-height) * 1px);\n      \
         content-visibility: auto;\n      display: flex;\n      align-items: end;\n      font-size: 9px;\n      \
         opacity: 0.35;\n    }}\n    .year__days {{\n      display: grid;\n      grid-auto-flow: column;\n      \
         grid-template-rows: repeat(var(--rows), calc(var(--size-dot) * 1px));\n      \
         grid-auto-columns: calc(var(--size-dot) * 1px);\n      gap: calc(var(--size-dot-gap) * 1px);\n      \
         contain: strict;\n      content-visibility: auto;\n      inline-size: calc(var(--w) * 1px);\n      \
         block-size: calc(var(--h) * 1px);\n    }}\n    .year__days .dot {{\n      contain: strict;\n      \
         content-visibility: auto;\n      aspect-ratio: 1;\n      inline-size: calc(var(--size-dot) * \
         1px);\n      block-size: calc(var(--size-dot) * 1px);\n      border: calc(var(--size-dot) * 0.04 \
         * 1px) solid var(--color-dot-border);\n      border-radius: calc(var(--size-dot) * 0.12 * 1px);\n    \
         }}\n    .dot--0 {{ background-color: var(--color-dot-0); }}\n    .dot--1 {{ background-color: \
         var(--color-dot-1); }}\n    .dot--2 {{ background-color: var(--color-dot-2); }}\n    .dot--3 \
         {{ background-color: var(--color-dot-3); }}\n    .dot--4 {{\n      background-color: \
         var(--color-dot-4);\n      animation: pulse 4s ease-in-out infinite;\n      animation-delay: \
         var(--pd, 0s);\n    }}\n    @keyframes pulse {{\n      0%, 100% {{ opacity: 1; }}\n      50% {{ \
         opacity: 0.65; }}\n    }}\n",
        shared_styles(font_data),
        DOT_ROWS,
        height_text,
        DOT_GAP_PX,
        DOT_SIZE_PX,
        YEAR_GAP_PX,
        LABEL_HEIGHT_PX,
        BREAKPOINT_MEDIUM_PX,
    )
}

fn render_stat_rows(summary_stats: &SummaryStats) -> String {
    let stats = [
        (summary_stats.week, "this week", "7s"),
        (summary_stats.month, "this month", "8.5s"),
        (summary_stats.year, "this year", "10s"),
        (summary_stats.total, "all time", "11.5s"),
    ];
    let mut html = String::new();

    for &(value, label, delay_s) in &stats {
        html.push_str(&format!(
            "<div class=\"stat-row\">\n          <span class=\"stat-num shine\" \
             style=\"animation-delay: {}\">{}</span>\n          <span class=\"stat-line\"></span>\n          \
             <span class=\"stat-label\">{}</span>\n        </div>\n",
            delay_s,
            format_count(value),
            label,
        ));
    }

    html
}

fn build_year_layouts(years: &[YearData]) -> Vec<YearLayout> {
    let mut layouts = Vec::with_capacity(years.len());

    for year in years {
        let column_count = year.days.len().div_ceil(DOT_ROWS);
        assert!(column_count > 0, "year must contain at least one day");
        let width_px = column_count * DOT_SIZE_PX + (column_count - 1) * DOT_GAP_PX;
        let height_px = DOT_ROWS * DOT_SIZE_PX + (DOT_ROWS - 1) * DOT_GAP_PX;
        layouts.push(YearLayout {
            width_px,
            height_px,
        });
    }

    layouts
}

fn total_graph_width(layouts: &[YearLayout]) -> usize {
    assert!(!layouts.is_empty(), "expected at least one layout");

    let mut total_width_px = 0;
    for (layout_index, layout) in layouts.iter().enumerate() {
        total_width_px += layout.width_px;
        if layout_index + 1 < layouts.len() {
            total_width_px += YEAR_GAP_PX;
        }
    }

    total_width_px
}

fn render_years_html(years: &[YearData], layouts: &[YearLayout], today_label: &str) -> String {
    assert_eq!(
        years.len(),
        layouts.len(),
        "years and layouts must stay aligned",
    );

    let mut html = String::new();
    for (year_index, (year, layout)) in years.iter().zip(layouts.iter()).enumerate() {
        html.push_str(&render_year_html(year, layout, year_index, today_label));
    }

    html
}

fn render_year_html(
    year: &YearData,
    layout: &YearLayout,
    year_index: usize,
    today_label: &str,
) -> String {
    let year_label = year_label(year, year_index, today_label);
    let dots_html = render_dots(&year.days);
    format!(
        "<div class=\"year year--{}\" style=\"--w: {}; --h: {};\">\n            <div \
         class=\"year__days\">{}</div>\n            <div class=\"year__label\"><span>{}</span></div>\n          \
         </div>\n",
        year_index, layout.width_px, layout.height_px, dots_html, year_label,
    )
}

fn render_dots(levels: &[u8]) -> String {
    let mut html = String::with_capacity(levels.len() * 32);

    for (level_index, &level) in levels.iter().enumerate() {
        assert!(level <= 4, "unexpected contribution level {level}");
        if level == 4 {
            html.push_str(&format!(
                "<div class=\"dot dot--{}\" style=\"--pd: {}\"></div>",
                level,
                deterministic_delay_hundredths(level_index * 37, 400),
            ));
            continue;
        }

        html.push_str(&format!("<div class=\"dot dot--{}\"></div>", level));
    }

    html
}

fn render_bio_chars(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut html = String::new();
    let mut character_index = 0_usize;

    for (line_index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            html.push_str("<br/>");
            continue;
        }

        for character in line.chars() {
            if character == ' ' {
                character_index += 1;
                html.push(' ');
                continue;
            }

            html.push_str("<span class=\"fade-in\" style=\"--i: ");
            html.push_str(&character_index.to_string());
            html.push_str(";\">");
            html.push_str(&escape_html_char(character));
            html.push_str("</span>");
            character_index += 1;
        }

        if line_index + 1 < lines.len() {
            html.push_str("<br/>");
        }
    }

    html
}

fn year_label(year: &YearData, year_index: usize, today_label: &str) -> String {
    if year_index == 0 {
        return escape_html(today_label);
    }

    let Some(year_prefix) = year.from.get(..4) else {
        panic!("year start must include a YYYY prefix");
    };
    escape_html(year_prefix)
}

fn format_count(value: u32) -> String {
    let digits = value.to_string();
    let digit_count = digits.len();
    let mut formatted = String::with_capacity(digit_count + digit_count / 3);

    for (digit_index, digit) in digits.chars().enumerate() {
        if digit_index > 0 && (digit_count - digit_index).is_multiple_of(3) {
            formatted.push(',');
        }
        formatted.push(digit);
    }

    formatted
}

fn escape_html(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        escaped.push_str(&escape_html_char(character));
    }
    escaped
}

fn escape_html_char(character: char) -> String {
    match character {
        '&' => String::from("&amp;"),
        '<' => String::from("&lt;"),
        '>' => String::from("&gt;"),
        '"' => String::from("&quot;"),
        '\'' => String::from("&#39;"),
        _ => character.to_string(),
    }
}

fn shared_styles(font_data: &FontData) -> String {
    format!(
        "\n  {}\n\n  :root {{\n    --color-text: {};\n    --color-dot-0: {};\n    \
         --color-dot-1: {};\n    --color-dot-2: {};\n    --color-dot-3: {};\n    \
         --color-dot-4: {};\n    --color-dot-border: {};\n\n    --default-delay: 1s;\n    \
         --default-duration: 1.55s;\n    --default-stagger: 0.1s;\n\n    --animate-in-links-delay: \
         calc(var(--default-delay) + var(--default-stagger) * 0);\n    --animate-in-org-delay: \
         calc(var(--default-delay) + var(--default-stagger) * 2);\n    --animate-in-stats-delay: \
         calc(var(--default-delay) + var(--default-stagger) * 4);\n    --animate-in-copy-delay: \
         calc(var(--default-delay) + var(--default-stagger) * 5);\n    --animate-in-graph-delay: \
         calc(var(--default-delay) + var(--default-stagger) * 17);\n  }}\n\n  *, *::before, \
         *::after {{ box-sizing: border-box; }}\n\n  .wrapper {{\n    contain: strict;\n    \
         block-size: calc(var(--size-height) * 1px);\n    container-type: inline-size;\n    \
         position: relative;\n    overflow: clip;\n    font-family: 'Departure-Mono', monospace;\n    \
         color: var(--color-text);\n  }}\n\n  @-moz-document url-prefix() {{\n    .wrapper {{ \
         display: none; }}\n  }}\n\n  .grid {{\n    display: grid;\n    grid-template-columns: \
         repeat(6, 1fr);\n  }}\n\n  .fade-in {{\n    will-change: opacity;\n    animation-name: \
         fade-in;\n    animation-fill-mode: both;\n    animation-duration: var(--duration, \
         var(--default-duration));\n    animation-timing-function: var(--ease, ease-out);\n    \
         animation-delay: var(--delay, var(--default-delay));\n  }}\n\n  @keyframes fade-in {{\n    \
         0% {{ opacity: 0; }}\n    100% {{ opacity: 1; }}\n  }}\n\n  .shine {{\n    \
         background-color: var(--color-text);\n    background-image: linear-gradient(-75deg,\n      \
         rgba(0,0,0,0) 0%,\n      rgba(255,255,255,0.18) 15%,\n      rgba(0,0,0,0) 25%\n    );\n    \
         background-size: 200%;\n    -webkit-background-clip: text;\n    \
         -webkit-text-fill-color: transparent;\n    background-clip: text;\n    animation-name: \
         shine;\n    animation-duration: 14s;\n    animation-iteration-count: infinite;\n  }}\n\n  \
         @keyframes shine {{\n    0% {{ background-position: 200%; }}\n    10% {{ \
         background-position: 0%; }}\n    100% {{ background-position: 0%; }}\n  }}\n\n  \
         p {{ margin: 0; }}\n",
        font_faces_css(font_data),
        COLOR_TEXT,
        COLOR_DOT_0,
        COLOR_DOT_1,
        COLOR_DOT_2,
        COLOR_DOT_3,
        COLOR_DOT_4,
        COLOR_DOT_BORDER,
    )
}

fn svg_document(
    width_px: Option<&str>,
    height_px: &str,
    view_box: Option<&str>,
    styles: &str,
    html: &str,
) -> String {
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\"");
    push_attribute(&mut svg, "height", height_px);
    push_attribute(&mut svg, "width", width_px.unwrap_or("100%"));
    push_optional_attribute(&mut svg, "viewBox", view_box);
    svg.push_str(">\n    <foreignObject width=\"100%\" height=\"100%\">\n");
    svg.push_str("      <div xmlns=\"http://www.w3.org/1999/xhtml\">\n");
    svg.push_str("        <style>");
    svg.push_str(styles);
    svg.push_str("</style>\n        ");
    svg.push_str(html);
    svg.push_str("\n      </div>\n    </foreignObject>\n  </svg>");
    svg
}

fn push_attribute(svg: &mut String, name: &str, value: &str) {
    svg.push(' ');
    svg.push_str(name);
    svg.push_str("=\"");
    svg.push_str(value);
    svg.push('"');
}

fn push_optional_attribute(svg: &mut String, name: &str, value: Option<&str>) {
    if let Some(value) = value {
        push_attribute(svg, name, value);
    }
}

fn deterministic_delay_tenths(seed: usize, cycle_tenths: usize) -> String {
    assert!(cycle_tenths > 0, "cycle_tenths must be non-zero");
    let delay_tenths = seed % cycle_tenths;
    let whole_seconds = delay_tenths / 10;
    let fractional_tenths = delay_tenths % 10;
    format!("{whole_seconds}.{fractional_tenths}s")
}

fn deterministic_delay_hundredths(seed: usize, cycle_hundredths: usize) -> String {
    assert!(cycle_hundredths > 0, "cycle_hundredths must be non-zero");
    let delay_hundredths = seed % cycle_hundredths;
    let whole_seconds = delay_hundredths / 100;
    let fractional_hundredths = delay_hundredths % 100;
    format!("{whole_seconds}.{fractional_hundredths:02}s")
}

const _: () = assert!(BREAKPOINT_MEDIUM_PX > 0);
const _: () = assert!(DOT_ROWS > 0);
const _: () = assert!(DOT_SIZE_PX > 0);
const _: () = assert!(LABEL_HEIGHT_PX > 0);

