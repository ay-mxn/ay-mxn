pub const FONT_REQUIREMENTS: &[(&str, &str)] = &[
    ("fonts/writer", ".woff2"),
    ("fonts/departure", ".woff2"),
    ("fonts/arabic", ".woff2"),
];

pub fn ensure_font_files() -> Result<(), std::io::Error> {
    Err(std::io::Error::other(
        "TODO: implement font preflight and base64 embedding in Rust.",
    ))
}
