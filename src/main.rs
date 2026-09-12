#![forbid(unsafe_code)]

mod content;
mod fonts;
mod github;
mod render;
mod stats;

use std::{env, fs, io, path::Path, process};

use time::{Month, OffsetDateTime};

const ASSETS_DIRECTORY_PATH: &str = "assets";
const MAIN_SVG_HEIGHT_PX: u32 = 440;
const STATS_JSON_PATH: &str = "src/stats.json";
const TOP_SVG_HEIGHT_PX: u32 = 20;
const LINK_SVG_HEIGHT_PX: u32 = 17;
const LINK_SVG_WIDTH_PX: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Stats,
    Build,
    Go,
}

fn parse_command() -> Result<Command, String> {
    let command = env::args()
        .nth(1)
        .ok_or_else(|| String::from("missing command"))?;

    match command.as_str() {
        "stats" => Ok(Command::Stats),
        "build" => Ok(Command::Build),
        "go" => Ok(Command::Go),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn print_usage() {
    eprintln!("Usage: cargo run -- <stats|build|go>");
}

fn main() {
    let command = match parse_command() {
        Ok(command) => command,
        Err(err) => {
            eprintln!("Error: {err}");
            print_usage();
            process::exit(2);
        }
    };

    if let Err(err) = run(command) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn run(command: Command) -> io::Result<()> {
    match command {
        Command::Stats => run_stats(),
        Command::Build => run_build(),
        Command::Go => {
            run_stats()?;
            run_build()
        }
    }
}

fn run_stats() -> io::Result<()> {
    let now = OffsetDateTime::now_utc();
    let stats_file = stats::build_stats_file(content::GITHUB_USERNAME, now)?;
    stats::write_stats_file(Path::new(STATS_JSON_PATH), &stats_file)?;
    println!("  ✓ {STATS_JSON_PATH}");
    Ok(())
}

fn run_build() -> io::Result<()> {
    let font_data = fonts::load_font_data()?;
    let stats_file = stats::read_stats_file(Path::new(STATS_JSON_PATH))?;
    fs::create_dir_all(ASSETS_DIRECTORY_PATH)?;

    write_svg(
        "top.svg",
        render::render_top_svg(&font_data, TOP_SVG_HEIGHT_PX),
    )?;

    for (link_index, link) in content::LINKS.iter().enumerate() {
        let file_name = format!("link-{}.svg", link.label);
        let svg = render::render_link_svg(
            &font_data,
            link.label,
            LINK_SVG_WIDTH_PX,
            LINK_SVG_HEIGHT_PX,
            link_index,
        );
        write_svg(&file_name, svg)?;
    }

    let today_label = display_date(OffsetDateTime::now_utc());
    let main_svg =
        render::render_main_svg(&font_data, &stats_file, MAIN_SVG_HEIGHT_PX, &today_label);
    write_svg("main.svg", main_svg)?;

    println!("\nDone. SVGs written to {ASSETS_DIRECTORY_PATH}/");
    Ok(())
}

fn write_svg(file_name: &str, svg: String) -> io::Result<()> {
    let output_path = Path::new(ASSETS_DIRECTORY_PATH).join(file_name);
    fs::write(&output_path, svg)?;
    println!("  ✓ {}", output_path.display());
    Ok(())
}

fn display_date(timestamp: OffsetDateTime) -> String {
    format!(
        "{} {}, {}",
        month_name(timestamp.month()),
        timestamp.day(),
        timestamp.year(),
    )
}

fn month_name(month: Month) -> &'static str {
    match month {
        Month::January => "Jan",
        Month::February => "Feb",
        Month::March => "Mar",
        Month::April => "Apr",
        Month::May => "May",
        Month::June => "Jun",
        Month::July => "Jul",
        Month::August => "Aug",
        Month::September => "Sep",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Dec",
    }
}
