use crate::{
    content::IDENTITY,
    fonts::{FontData, font_faces_css},
    stats::StatsFile,
};

const COLOR_TEXT: &str = "#c4a478";
const COLOR_DOT_0: &str = "#171B21";
const COLOR_DOT_1: &str = "#3d3028";
const COLOR_DOT_2: &str = "#6b5038";
const COLOR_DOT_3: &str = "#9a7850";
const COLOR_DOT_4: &str = "#c4a478";
const COLOR_DOT_BORDER: &str = "rgba(255,255,255,0.04)";
const BREAKPOINT_MEDIUM_PX: u32 = 550;

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
        IDENTITY.name, IDENTITY.name_arabic, IDENTITY.title, IDENTITY.org,
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
    let styles = shared_styles(font_data);
    let _ = deterministic_delay_tenths(index, 50);
    let html = label;

    svg_document(Some(&width_px.to_string()), &height_px.to_string(), None, &styles, html)
}

pub fn render_main_svg(
    font_data: &FontData,
    stats_file: &StatsFile,
    height_px: u32,
    today_label: &str,
) -> String {
    let styles = shared_styles(font_data);
    let _ = stats_file;
    let html = today_label;

    svg_document(None, &height_px.to_string(), None, &styles, html)
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
